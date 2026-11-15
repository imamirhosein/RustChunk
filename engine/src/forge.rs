use serde_json::{json, Value};
use std::collections::HashSet;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

const TARGET_VERSION: &str = "1.20.1";
const MAX_INHERITANCE_DEPTH: usize = 8;
const MAX_SCAN_ENTRIES: usize = 6000;

pub fn prepare_forge_profile(app_data_dir: &Path) -> Result<String, String> {
    if !app_data_dir.is_absolute() {
        return Err("Android app data path must be absolute".to_string());
    }

    let game_root = app_data_dir.join("minecraft");
    let versions_root = game_root.join("versions");
    if !versions_root.is_dir() {
        return Err(format!("Minecraft versions directory is missing: {}", versions_root.display()));
    }

    let (profile_dir, profile_json_path, profile) = find_forge_profile(&versions_root)?;
    let profile_id = profile
        .get("id")
        .and_then(Value::as_str)
        .unwrap_or_else(|| profile_dir.file_name().and_then(|v| v.to_str()).unwrap_or("unknown"))
        .to_string();

    let mut chain = Vec::new();
    let mut visited = HashSet::new();
    let mut notices = Vec::new();
    collect_inheritance_chain(
        &versions_root,
        &profile,
        &mut visited,
        &mut chain,
        &mut notices,
        0,
    );

    let minecraft_version = inherited_string(&chain, "inheritsFrom")
        .unwrap_or_else(|| TARGET_VERSION.to_string());
    let main_class = inherited_string(&chain, "mainClass");
    let java_major = inherited_integer(&chain, "javaVersion", "majorVersion").unwrap_or(17);
    let asset_index = inherited_value(&chain, "assetIndex").cloned();
    let libraries = merge_libraries(&chain);
    let game_arguments = merge_arguments(&chain, "game");
    let jvm_arguments = merge_arguments(&chain, "jvm");
    let legacy_arguments = merge_legacy_arguments(&chain);

    let mut checked_libraries = 0usize;
    let mut missing_libraries = Vec::new();
    let mut missing_library_total = 0usize;
    let mut unresolved_libraries = Vec::new();

    for library in &libraries {
        let name = library.get("name").and_then(Value::as_str).unwrap_or("(unnamed library)");
        match library_artifact_path(library) {
            Some(relative) => {
                checked_libraries += 1;
                match validate_relative_path(&relative) {
                    Ok(safe_relative) => {
                        if !game_root.join("libraries").join(&safe_relative).is_file() {
                            missing_library_total += 1;
                            if missing_libraries.len() < 30 {
                                missing_libraries.push(format!("{} -> {}", name, safe_relative));
                            }
                        }
                    }
                    Err(_) => unresolved_libraries.push(name.to_string()),
                }
            }
            None => unresolved_libraries.push(name.to_string()),
        }
    }

    let client_jar = find_client_jar(&game_root, &profile_dir, &profile, &minecraft_version);
    let asset_index_id = asset_index
        .as_ref()
        .and_then(|value| value.get("id"))
        .and_then(Value::as_str)
        .map(str::to_string);
    let asset_index_file = asset_index_id
        .as_deref()
        .map(|id| game_root.join("assets").join("indexes").join(format!("{}.json", id)));
    let asset_index_present = asset_index_file.as_ref().is_some_and(|path| path.is_file());

    // This imported Zalith profile keeps its mods/config beside its version JSON/JAR.
    // Record the existing paths; never move or duplicate the user's files.
    let mods_dir = profile_dir.join("mods");
    let config_dir = profile_dir.join("config");
    let mod_jar_count = count_direct_jars(&mods_dir);
    let runtime_root = game_root.join("runtime");
    let runtime_candidates = find_runtime_candidates(&runtime_root);

    let relative = |path: &Path| relative_path(&game_root, path);
    let chain_ids: Vec<String> = chain
        .iter()
        .filter_map(|item| item.get("id").and_then(Value::as_str).map(str::to_string))
        .collect();

    let spec = json!({
        "schema_version": 1,
        "status": "prepared_for_review_only",
        "profile_id": profile_id.clone(),
        "profile_json": relative(&profile_json_path),
        "profile_directory": relative(&profile_dir),
        "minecraft_version": minecraft_version.clone(),
        "inheritance_chain": chain_ids.clone(),
        "main_class": main_class.clone(),
        "java_major_version": java_major,
        "client_jar": client_jar.as_ref().map(|path| relative(path)),
        "client_jar_present": client_jar.as_ref().is_some_and(|path| path.is_file()),
        "asset_index": asset_index,
        "asset_index_present": asset_index_present,
        "libraries": libraries.clone(),
        "library_artifacts_checked": checked_libraries,
        "library_artifacts_missing": missing_libraries.clone(),
        "library_artifacts_missing_total": missing_library_total,
        "library_artifacts_unresolved": unresolved_libraries.clone(),
        "arguments": {
            "game": game_arguments,
            "jvm": jvm_arguments
        },
        "legacy_minecraft_arguments": legacy_arguments,
        "game_directory": relative(&profile_dir),
        "mods_directory": relative(&mods_dir),
        "mods_directory_present": mods_dir.is_dir(),
        "mods_jar_count": mod_jar_count,
        "config_directory": relative(&config_dir),
        "config_directory_present": config_dir.is_dir(),
        "java_runtime_root": relative(&runtime_root),
        "java_runtime_candidates": runtime_candidates.iter().map(|item| json!({
            "java_executable": relative(&item.java_executable),
            "libjvm": relative(&item.libjvm),
            "aarch64_elf": item.aarch64_elf
        })).collect::<Vec<Value>>(),
        "launch_performed": false
    });

    let spec_path = game_root.join("forge-launch-spec.json");
    let report_path = game_root.join("forge-readiness-report.txt");
    let spec_bytes = serde_json::to_vec_pretty(&spec)
        .map_err(|error| format!("Could not encode Forge launch specification: {}", error))?;
    write_atomic(&spec_path, &spec_bytes)?;

    let client_jar_present = client_jar.as_ref().is_some_and(|path| path.is_file());
    let mut report = String::new();
    report.push_str("RustChunk Forge launch-profile preparation\n");
    report.push_str(&format!("Profile: {}\n", profile_id));
    report.push_str(&format!("Profile JSON: {}\n", profile_json_path.display()));
    report.push_str(&format!("Profile directory: {}\n", profile_dir.display()));
    report.push_str(&format!("Minecraft target: {}\n", minecraft_version));
    report.push_str(&format!("Inheritance chain: {}\n", chain_ids.join(" -> ")));
    report.push_str(&format!("Main class: {}\n", main_class.as_deref().unwrap_or("MISSING")));
    report.push_str(&format!("Java major version requested by metadata: {}\n", java_major));
    report.push_str(&format!(
        "Client JAR: {}\n",
        if client_jar_present {
            client_jar.as_ref().unwrap().display().to_string()
        } else {
            "MISSING".to_string()
        }
    ));
    report.push_str(&format!(
        "Asset index: {}\n",
        match (asset_index_id.as_deref(), asset_index_present) {
            (Some(id), true) => format!("PRESENT ({})", id),
            (Some(id), false) => format!("MISSING ({})", id),
            (None, _) => "NOT DECLARED".to_string(),
        }
    ));
    report.push_str(&format!(
        "Libraries: {} entries; {} artifact paths checked; {} missing (up to 30 listed); {} unresolved\n",
        libraries.len(),
        checked_libraries,
        missing_library_total,
        unresolved_libraries.len()
    ));
    for item in &missing_libraries {
        report.push_str(&format!("  MISSING: {}\n", item));
    }
    for item in unresolved_libraries.iter().take(20) {
        report.push_str(&format!("  UNRESOLVED: {}\n", item));
    }
    report.push_str(&format!(
        "Mods: {} | {} JAR files\n",
        if mods_dir.is_dir() { "PRESENT" } else { "MISSING" },
        mod_jar_count
    ));
    report.push_str(&format!(
        "Config: {}\n",
        if config_dir.is_dir() { "PRESENT" } else { "MISSING" }
    ));
    report.push_str(&format!(
        "Java runtime root: {}\n",
        if runtime_root.is_dir() { "PRESENT" } else { "MISSING" }
    ));
    report.push_str(&format!("Java runtime candidates: {}\n", runtime_candidates.len()));
    for runtime in runtime_candidates.iter().take(10) {
        report.push_str(&format!(
            "  - java: {} | libjvm: {} | AArch64 ELF: {}\n",
            runtime.java_executable.display(),
            runtime.libjvm.display(),
            runtime.aarch64_elf
        ));
    }
    for notice in &notices {
        report.push_str(&format!("NOTICE: {}\n", notice));
    }
    report.push_str(&format!("Launch specification: {}\n", spec_path.display()));
    report.push_str(&format!("Saved report: {}\n", report_path.display()));
    report.push_str(
        "\nThis step only analyzes metadata and writes a launch specification. It does NOT start Java, launch Forge, load mods, or verify Aternos compatibility.\n",
    );

    write_atomic(&report_path, report.as_bytes())?;
    Ok(report)
}

fn find_forge_profile(versions_root: &Path) -> Result<(PathBuf, PathBuf, Value), String> {
    let entries = fs::read_dir(versions_root)
        .map_err(|error| format!("Cannot list versions directory: {}", error))?;
    let mut candidates: Vec<(PathBuf, PathBuf, Value)> = Vec::new();

    for entry in entries {
        let entry = entry.map_err(|error| format!("Cannot read version entry: {}", error))?;
        let directory = entry.path();
        if !directory.is_dir() {
            continue;
        }

        let folder_name = directory.file_name().and_then(|value| value.to_str()).unwrap_or("");
        let lower_name = folder_name.to_lowercase();
        if !lower_name.contains(TARGET_VERSION) || !lower_name.contains("forge") {
            continue;
        }

        let files = match fs::read_dir(&directory) {
            Ok(value) => value,
            Err(_) => continue,
        };
        for file in files.flatten() {
            let path = file.path();
            if path.extension().and_then(|value| value.to_str()) != Some("json") {
                continue;
            }
            let Ok(bytes) = fs::read(&path) else { continue };
            let Ok(value) = serde_json::from_slice::<Value>(&bytes) else { continue };
            let Some(id) = value.get("id").and_then(Value::as_str) else { continue };
            let looks_like_profile = value.get("mainClass").is_some()
                || value.get("inheritsFrom").is_some()
                || value.get("libraries").is_some();
            if !looks_like_profile {
                continue;
            }
            let lower_id = id.to_lowercase();
            if !lower_id.contains(TARGET_VERSION) || !lower_id.contains("forge") {
                continue;
            }
            candidates.push((directory.clone(), path, value));
        }
    }

    candidates.sort_by(|left, right| {
        let left_id = left.2.get("id").and_then(Value::as_str).unwrap_or("");
        let right_id = right.2.get("id").and_then(Value::as_str).unwrap_or("");
        let left_exact = left_id == left.0.file_name().and_then(|value| value.to_str()).unwrap_or("");
        let right_exact = right_id == right.0.file_name().and_then(|value| value.to_str()).unwrap_or("");
        right_exact.cmp(&left_exact).then_with(|| left_id.cmp(right_id))
    });

    candidates.into_iter().next().ok_or_else(|| {
        "No Minecraft 1.20.1 Forge profile JSON was found under versions/. Run the game-data inspection first.".to_string()
    })
}

fn collect_inheritance_chain(
    versions_root: &Path,
    profile: &Value,
    visited: &mut HashSet<String>,
    chain: &mut Vec<Value>,
    notices: &mut Vec<String>,
    depth: usize,
) {
    let id = profile.get("id").and_then(Value::as_str).unwrap_or("unknown");
    if depth == 0 {
        visited.insert(id.to_string());
    }

    if let Some(parent_id) = profile.get("inheritsFrom").and_then(Value::as_str) {
        if depth >= MAX_INHERITANCE_DEPTH {
            notices.push("Version inheritance exceeded the safety depth limit.".to_string());
        } else if !is_safe_version_id(parent_id) {
            notices.push(format!("Ignored unsafe parent version identifier: {}", parent_id));
        } else if !visited.insert(parent_id.to_string()) {
            notices.push(format!("Detected a cyclic inheritance reference at {}.", parent_id));
        } else if let Some(parent) = read_version_profile(versions_root, parent_id) {
            collect_inheritance_chain(versions_root, &parent, visited, chain, notices, depth + 1);
        } else {
            notices.push(format!(
                "Parent version JSON '{}' is not present in versions/. Inherited arguments/libraries may be incomplete.",
                parent_id
            ));
        }
    }

    chain.push(profile.clone());
}

fn read_version_profile(versions_root: &Path, version_id: &str) -> Option<Value> {
    if !is_safe_version_id(version_id) {
        return None;
    }
    let directory = versions_root.join(version_id);
    if !directory.is_dir() {
        return None;
    }

    let exact = directory.join(format!("{}.json", version_id));
    if exact.is_file() {
        if let Ok(bytes) = fs::read(exact) {
            if let Ok(value) = serde_json::from_slice::<Value>(&bytes) {
                return Some(value);
            }
        }
    }

    let entries = fs::read_dir(directory).ok()?;
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|value| value.to_str()) != Some("json") {
            continue;
        }
        let Ok(bytes) = fs::read(path) else { continue };
        let Ok(value) = serde_json::from_slice::<Value>(&bytes) else { continue };
        if value.get("id").and_then(Value::as_str) == Some(version_id) {
            return Some(value);
        }
    }
    None
}

fn inherited_value<'a>(chain: &'a [Value], key: &str) -> Option<&'a Value> {
    chain.iter().rev().find_map(|profile| profile.get(key))
}

fn inherited_string(chain: &[Value], key: &str) -> Option<String> {
    inherited_value(chain, key)
        .and_then(Value::as_str)
        .map(str::to_string)
}

fn inherited_integer(chain: &[Value], object_key: &str, value_key: &str) -> Option<i64> {
    inherited_value(chain, object_key)
        .and_then(|value| value.get(value_key))
        .and_then(Value::as_i64)
}

fn merge_libraries(chain: &[Value]) -> Vec<Value> {
    let mut names = HashSet::new();
    let mut merged = Vec::new();
    for profile in chain {
        if let Some(items) = profile.get("libraries").and_then(Value::as_array) {
            for item in items {
                let key = item
                    .get("name")
                    .and_then(Value::as_str)
                    .map(str::to_string)
                    .unwrap_or_else(|| format!("anonymous-{}", merged.len()));
                if names.insert(key) {
                    merged.push(item.clone());
                }
            }
        }
    }
    merged
}

fn merge_arguments(chain: &[Value], group: &str) -> Vec<Value> {
    let mut merged = Vec::new();
    for profile in chain {
        if let Some(items) = profile
            .get("arguments")
            .and_then(|value| value.get(group))
            .and_then(Value::as_array)
        {
            merged.extend(items.iter().cloned());
        }
    }
    merged
}

fn merge_legacy_arguments(chain: &[Value]) -> Option<String> {
    let values: Vec<&str> = chain
        .iter()
        .filter_map(|profile| profile.get("minecraftArguments").and_then(Value::as_str))
        .collect();
    if values.is_empty() {
        None
    } else {
        Some(values.join(" "))
    }
}

fn library_artifact_path(library: &Value) -> Option<String> {
    if let Some(path) = library
        .get("downloads")
        .and_then(|value| value.get("artifact"))
        .and_then(|value| value.get("path"))
        .and_then(Value::as_str)
    {
        return Some(path.to_string());
    }

    library
        .get("name")
        .and_then(Value::as_str)
        .and_then(maven_coordinate_to_path)
}

fn maven_coordinate_to_path(coordinate: &str) -> Option<String> {
    let mut coordinate_parts = coordinate.split('@');
    let coordinates = coordinate_parts.next()?;
    let extension = coordinate_parts.next().unwrap_or("jar");
    let parts: Vec<&str> = coordinates.split(':').collect();
    if !(3..=4).contains(&parts.len())
        || parts.iter().any(|part| part.is_empty() || part.contains('/') || part.contains('\\'))
        || extension.is_empty()
        || !extension.chars().all(|character| character.is_ascii_alphanumeric())
    {
        return None;
    }

    let group_path = parts[0].replace('.', "/");
    let artifact = parts[1];
    let version = parts[2];
    let classifier = parts.get(3).map(|value| format!("-{}", value)).unwrap_or_default();
    Some(format!(
        "{}/{}/{}/{}-{}{}.{}",
        group_path, artifact, version, artifact, version, classifier, extension
    ))
}

fn find_client_jar(
    game_root: &Path,
    profile_dir: &Path,
    profile: &Value,
    minecraft_version: &str,
) -> Option<PathBuf> {
    let jar_id = profile
        .get("jar")
        .and_then(Value::as_str)
        .unwrap_or(minecraft_version);
    let candidates = [
        game_root.join("versions").join(jar_id).join(format!("{}.jar", jar_id)),
        profile_dir.join(format!(
            "{}.jar",
            profile.get("id").and_then(Value::as_str).unwrap_or("")
        )),
        game_root.join("versions").join(minecraft_version).join(format!("{}.jar", minecraft_version)),
    ];
    candidates.into_iter().find(|path| path.is_file())
}

fn count_direct_jars(directory: &Path) -> usize {
    fs::read_dir(directory)
        .ok()
        .map(|entries| {
            entries
                .flatten()
                .filter(|entry| {
                    entry.path().is_file()
                        && entry.path().extension().and_then(|value| value.to_str())
                            .is_some_and(|extension| extension.eq_ignore_ascii_case("jar"))
                })
                .count()
        })
        .unwrap_or(0)
}

struct RuntimeCandidate {
    java_executable: PathBuf,
    libjvm: PathBuf,
    aarch64_elf: bool,
}

fn find_runtime_candidates(runtime_root: &Path) -> Vec<RuntimeCandidate> {
    if !runtime_root.is_dir() {
        return Vec::new();
    }

    let mut pending = vec![(runtime_root.to_path_buf(), 0usize)];
    let mut visited = 0usize;
    let mut java_executables = Vec::new();
    let mut jvm_libraries = Vec::new();

    while let Some((directory, depth)) = pending.pop() {
        if visited >= MAX_SCAN_ENTRIES || depth > 8 {
            break;
        }
        visited += 1;
        let Ok(entries) = fs::read_dir(&directory) else { continue };

        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                pending.push((path, depth + 1));
                continue;
            }
            let name = path.file_name().and_then(|value| value.to_str()).unwrap_or("");
            if (name == "java" || name == "javaw")
                && path.parent().and_then(Path::file_name).and_then(|value| value.to_str()) == Some("bin")
            {
                java_executables.push(path);
            } else if name == "libjvm.so" {
                jvm_libraries.push(path);
            }
        }
    }

    let mut candidates = Vec::new();
    for java_executable in java_executables {
        let java_root = java_executable.parent().and_then(Path::parent);
        let matching_jvm = jvm_libraries
            .iter()
            .find(|path| path.starts_with(java_root.unwrap_or(runtime_root)))
            .or_else(|| jvm_libraries.first());
        if let Some(libjvm) = matching_jvm {
            candidates.push(RuntimeCandidate {
                java_executable,
                libjvm: libjvm.clone(),
                aarch64_elf: is_aarch64_elf(libjvm),
            });
        }
    }
    candidates
}

fn is_aarch64_elf(path: &Path) -> bool {
    let Ok(mut file) = fs::File::open(path) else { return false };
    let mut header = [0u8; 20];
    if file.read_exact(&mut header).is_err() {
        return false;
    }
    header[0..4] == [0x7f, b'E', b'L', b'F']
        && header[4] == 2
        && header[5] == 1
        && u16::from_le_bytes([header[18], header[19]]) == 183
}

fn is_safe_version_id(value: &str) -> bool {
    !value.is_empty()
        && value != "."
        && value != ".."
        && value.chars().all(|character| {
            character.is_ascii_alphanumeric() || "._- ".contains(character)
        })
}

fn validate_relative_path(path: &str) -> Result<String, String> {
    if path.is_empty() || path.starts_with('/') || path.contains('\\') {
        return Err(format!("Unsafe relative path: {}", path));
    }
    if path
        .split('/')
        .any(|part| part.is_empty() || part == "." || part == "..")
    {
        return Err(format!("Unsafe relative path: {}", path));
    }
    Ok(path.to_string())
}

fn relative_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .map(|value| value.to_string_lossy().replace('\\', "/"))
        .unwrap_or_else(|_| path.to_string_lossy().into_owned())
}

fn write_atomic(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let parent = path
        .parent()
        .ok_or_else(|| format!("Invalid output path: {}", path.display()))?;
    fs::create_dir_all(parent)
        .map_err(|error| format!("Cannot create {}: {}", parent.display(), error))?;
    let file_name = path
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or_else(|| format!("Invalid output filename: {}", path.display()))?;
    let temporary = parent.join(format!(".{}.tmp", file_name));
    fs::write(&temporary, bytes)
        .map_err(|error| format!("Cannot write {}: {}", temporary.display(), error))?;
    fs::rename(&temporary, path)
        .map_err(|error| format!("Cannot finalize {}: {}", path.display(), error))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_maven_coordinates_to_relative_jar_paths() {
        assert_eq!(
            maven_coordinate_to_path("net.minecraftforge:forge:1.20.1-47.4.22"),
            Some("net/minecraftforge/forge/1.20.1-47.4.22/forge-1.20.1-47.4.22.jar".to_string())
        );
        assert_eq!(
            maven_coordinate_to_path("org.example:demo:1.2:linux@zip"),
            Some("org/example/demo/1.2/demo-1.2-linux.zip".to_string())
        );
        assert!(maven_coordinate_to_path("../../bad:thing:1").is_none());
    }

    #[test]
    fn merges_parent_arguments_before_forge_arguments() {
        let parent: Value = serde_json::from_str(
            r#"{"arguments":{"game":["--username","Steve"],"jvm":["-Xmx2G"]}}"#,
        ).unwrap();
        let child: Value = serde_json::from_str(
            r#"{"arguments":{"game":["--launchTarget","forgeserver"],"jvm":["-Dforge=true"]}}"#,
        ).unwrap();
        let merged = merge_arguments(&[parent, child], "game");
        assert_eq!(merged.len(), 4);
        assert_eq!(merged[0], "--username");
        assert_eq!(merged[2], "--launchTarget");
    }

    #[test]
    fn rejects_unsafe_version_ids_and_library_paths() {
        assert!(is_safe_version_id("1.20.1"));
        assert!(is_safe_version_id("1.20.1 Forge 47.4.22"));
        assert!(!is_safe_version_id("../outside"));
        assert!(validate_relative_path("net/minecraftforge/forge/file.jar").is_ok());
        assert!(validate_relative_path("../../outside.jar").is_err());
    }

    #[test]
    fn verifies_android_arm64_elf_machine_header() {
        let temp = std::env::temp_dir().join(format!("rustchunk-elf-test-{}", std::process::id()));
        let mut bytes = [0u8; 20];
        bytes[0..4].copy_from_slice(&[0x7f, b'E', b'L', b'F']);
        bytes[4] = 2;
        bytes[5] = 1;
        bytes[18..20].copy_from_slice(&183u16.to_le_bytes());
        fs::write(&temp, bytes).unwrap();
        assert!(is_aarch64_elf(&temp));
        fs::remove_file(temp).unwrap();
    }
}
