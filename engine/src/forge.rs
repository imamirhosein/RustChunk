use serde_json::{json, Value};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

const TARGET_VERSION: &str = "1.20.1";
const MAX_INHERITANCE_DEPTH: usize = 8;
const MAX_SCAN_ENTRIES: usize = 6000;

pub fn list_profiles(app_data_dir: &Path) -> Result<String, String> {
    if !app_data_dir.is_absolute() {
        return Err("Android app data path must be absolute".to_string());
    }
    let versions_root = app_data_dir.join("minecraft").join("versions");
    if !versions_root.is_dir() {
        return Err(format!("Minecraft versions directory is missing: {}", versions_root.display()));
    }

    let discovered = discover_profiles(&versions_root)?;
    let mut seen = HashSet::new();
    let mut profiles = Vec::new();
    for (directory, _json_path, profile) in discovered {
        let Some(id) = profile.get("id").and_then(Value::as_str) else { continue };
        if !seen.insert(id.to_string()) { continue; }
        let loader = profile_loader(&profile);
        let minecraft_version = profile_minecraft_version(&profile, id);
        profiles.push(json!({
            "id": id,
            "display_name": id,
            "loader": loader,
            "minecraft_version": minecraft_version,
            "directory": directory.file_name().and_then(|value| value.to_str()).unwrap_or(""),
            "supported_for_forge_analysis": loader == "Forge" && minecraft_version == TARGET_VERSION
        }));
    }
    profiles.sort_by(|left, right| {
        left.get("id").and_then(Value::as_str).unwrap_or("")
            .cmp(right.get("id").and_then(Value::as_str).unwrap_or(""))
    });
    serde_json::to_string(&profiles)
        .map_err(|error| format!("Could not encode profile list: {}", error))
}

pub fn prepare_forge_profile(app_data_dir: &Path, selected_profile_id: &str) -> Result<String, String> {
    if !app_data_dir.is_absolute() {
        return Err("Android app data path must be absolute".to_string());
    }
    if !is_safe_version_id(selected_profile_id) {
        return Err("Selected profile identifier is invalid.".to_string());
    }

    let game_root = app_data_dir.join("minecraft");
    let versions_root = game_root.join("versions");
    if !versions_root.is_dir() {
        return Err(format!("Minecraft versions directory is missing: {}", versions_root.display()));
    }

    let (profile_dir, profile_json_path, profile) =
        find_forge_profile(&versions_root, selected_profile_id)?;
    let profile_id = profile.get("id").and_then(Value::as_str)
        .unwrap_or(selected_profile_id).to_string();

    let mut chain = Vec::new();
    let mut visited = HashSet::new();
    let mut notices = Vec::new();
    collect_inheritance_chain(
        &versions_root, &profile, &mut visited, &mut chain, &mut notices, 0,
    );

    let minecraft_version = chain.first()
        .and_then(|item| item.get("id"))
        .and_then(Value::as_str)
        .filter(|id| !id.to_lowercase().contains("forge")
            && !id.to_lowercase().contains("fabric")
            && !id.to_lowercase().contains("quilt")
            && !id.to_lowercase().contains("neoforge"))
        .map(str::to_string)
        .or_else(|| profile.get("inheritsFrom").and_then(Value::as_str).map(str::to_string))
        .unwrap_or_else(|| profile_minecraft_version(&profile, &profile_id));

    if minecraft_version != TARGET_VERSION {
        return Err(format!(
            "The selected profile '{}' targets Minecraft {}, but this analysis stage currently supports Forge {} only.",
            profile_id, minecraft_version, TARGET_VERSION
        ));
    }

    let main_class = inherited_string(&chain, "mainClass");
    let java_major = inherited_integer(&chain, "javaVersion", "majorVersion").unwrap_or(17);
    let asset_index = inherited_value(&chain, "assetIndex").cloned();
    let libraries = merge_libraries(&chain);
    let game_arguments = merge_arguments(&chain, "game");
    let jvm_arguments = merge_arguments(&chain, "jvm");
    let legacy_arguments = merge_legacy_arguments(&chain);

    let mut checked_libraries = 0usize;
    let mut skipped_desktop_native = 0usize;
    let mut missing_libraries = Vec::new();
    let mut missing_library_total = 0usize;
    let mut unresolved_libraries = Vec::new();

    for library in &libraries {
        let name = library.get("name").and_then(Value::as_str).unwrap_or("(unnamed library)");
        if is_desktop_native_library(library) {
            skipped_desktop_native += 1;
            continue;
        }
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
    let asset_index_id = asset_index.as_ref()
        .and_then(|value| value.get("id"))
        .and_then(Value::as_str)
        .map(str::to_string);
    let asset_index_file = asset_index_id.as_deref()
        .map(|id| game_root.join("assets").join("indexes").join(format!("{}.json", id)));
    let asset_index_present = asset_index_file.as_ref().is_some_and(|path| path.is_file());

    // Keep mods/config and generated launch files isolated inside the selected profile folder.
    let mods_dir = profile_dir.join("mods");
    let config_dir = profile_dir.join("config");
    let mod_jar_count = count_direct_jars(&mods_dir);

    // Zalith may keep its Android Java runtime outside the .minecraft tree.
    let runtime_roots = runtime_search_roots(app_data_dir);
    let mut runtime_candidates = Vec::new();
    for root in &runtime_roots {
        runtime_candidates.extend(find_runtime_candidates(root));
    }

    let relative_game = |path: &Path| relative_path(&game_root, path);
    let relative_app = |path: &Path| relative_path(app_data_dir, path);
    let chain_ids: Vec<String> = chain.iter()
        .filter_map(|item| item.get("id").and_then(Value::as_str).map(str::to_string))
        .collect();
    let runtime_root_strings: Vec<String> = runtime_roots.iter()
        .map(|path| relative_app(path)).collect();

    let spec = json!({
        "schema_version": 2,
        "status": "prepared_for_review_only",
        "profile_id": profile_id.clone(),
        "profile_json": relative_game(&profile_json_path),
        "profile_directory": relative_game(&profile_dir),
        "minecraft_version": minecraft_version.clone(),
        "inheritance_chain": chain_ids.clone(),
        "main_class": main_class.clone(),
        "java_major_version": java_major,
        "client_jar": client_jar.as_ref().map(|path| relative_game(path)),
        "client_jar_present": client_jar.as_ref().is_some_and(|path| path.is_file()),
        "asset_index": asset_index,
        "asset_index_present": asset_index_present,
        "libraries": libraries.clone(),
        "library_artifacts_checked": checked_libraries,
        "library_artifacts_missing": missing_libraries.clone(),
        "library_artifacts_missing_total": missing_library_total,
        "library_artifacts_unresolved": unresolved_libraries.clone(),
        "desktop_native_artifacts_skipped": skipped_desktop_native,
        "arguments": { "game": game_arguments, "jvm": jvm_arguments },
        "legacy_minecraft_arguments": legacy_arguments,
        "game_directory": relative_game(&profile_dir),
        "mods_directory": relative_game(&mods_dir),
        "mods_directory_present": mods_dir.is_dir(),
        "mods_jar_count": mod_jar_count,
        "config_directory": relative_game(&config_dir),
        "config_directory_present": config_dir.is_dir(),
        "java_runtime_search_roots": runtime_root_strings,
        "java_runtime_candidates": runtime_candidates.iter().map(|item| json!({
            "java_executable": relative_app(&item.java_executable),
            "libjvm": relative_app(&item.libjvm),
            "aarch64_elf": item.aarch64_elf
        })).collect::<Vec<Value>>(),
        "launch_performed": false
    });

    let spec_path = profile_dir.join("rustchunk-launch-spec.json");
    let report_path = profile_dir.join("rustchunk-readiness-report.txt");
    let spec_bytes = serde_json::to_vec_pretty(&spec)
        .map_err(|error| format!("Could not encode Forge launch specification: {}", error))?;
    write_atomic(&spec_path, &spec_bytes)?;

    let client_jar_present = client_jar.as_ref().is_some_and(|path| path.is_file());
    let mut report = String::new();
    report.push_str("RustChunk Forge launch-profile preparation (Stage 9)\n");
    report.push_str(&format!("Selected profile: {}\n", profile_id));
    report.push_str(&format!("Profile JSON: {}\n", profile_json_path.display()));
    report.push_str(&format!("Profile directory: {}\n", profile_dir.display()));
    report.push_str(&format!("Minecraft target: {}\n", minecraft_version));
    report.push_str(&format!("Inheritance chain (base -> selected): {}\n", chain_ids.join(" -> ")));
    report.push_str(&format!("Main class: {}\n", main_class.as_deref().unwrap_or("MISSING")));
    report.push_str(&format!("Java major version requested by metadata: {}\n", java_major));
    report.push_str(&format!(
        "Client JAR: {}\n",
        if client_jar_present { client_jar.as_ref().unwrap().display().to_string() }
        else { "MISSING".to_string() }
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
        "Libraries: {} entries; {} non-desktop-native artifact paths checked; {} missing (up to 30 listed); {} unresolved; {} desktop-native entries skipped\n",
        libraries.len(), checked_libraries, missing_library_total,
        unresolved_libraries.len(), skipped_desktop_native
    ));
    for item in &missing_libraries { report.push_str(&format!("  MISSING: {}\n", item)); }
    for item in unresolved_libraries.iter().take(20) {
        report.push_str(&format!("  UNRESOLVED: {}\n", item));
    }
    report.push_str(&format!(
        "Mods: {} | {} JAR files\n",
        if mods_dir.is_dir() { "PRESENT" } else { "MISSING" }, mod_jar_count
    ));
    report.push_str(&format!("Config: {}\n", if config_dir.is_dir() { "PRESENT" } else { "MISSING" }));
    report.push_str("Java runtime search roots:\n");
    for root in &runtime_roots {
        report.push_str(&format!("  - {} | {}\n", root.display(), if root.is_dir() { "PRESENT" } else { "MISSING" }));
    }
    report.push_str(&format!("Java runtime candidates: {}\n", runtime_candidates.len()));
    for runtime in runtime_candidates.iter().take(10) {
        report.push_str(&format!(
            "  - java: {} | libjvm: {} | AArch64 ELF: {}\n",
            runtime.java_executable.display(), runtime.libjvm.display(), runtime.aarch64_elf
        ));
    }
    for notice in &notices { report.push_str(&format!("NOTICE: {}\n", notice)); }
    report.push_str(&format!("Profile-specific launch specification: {}\n", spec_path.display()));
    report.push_str(&format!("Profile-specific report: {}\n", report_path.display()));
    report.push_str(
        "\nThis step only analyzes metadata and writes a profile-specific launch specification. It does NOT start Java, launch Forge, load mods, or verify Aternos compatibility.\n",
    );

    write_atomic(&report_path, report.as_bytes())?;
    Ok(report)
}


pub fn prepare_forge_dependency_plan(
    app_data_dir: &Path,
    selected_profile_id: &str,
) -> Result<String, String> {
    if !app_data_dir.is_absolute() {
        return Err("Android app data path must be absolute".to_string());
    }
    if !is_safe_version_id(selected_profile_id) {
        return Err("Selected profile identifier is invalid.".to_string());
    }

    let game_root = app_data_dir.join("minecraft");
    let versions_root = game_root.join("versions");
    let (profile_dir, _profile_json_path, profile) =
        find_forge_profile(&versions_root, selected_profile_id)?;
    let mut chain = Vec::new();
    let mut visited = HashSet::new();
    let mut notices = Vec::new();
    collect_inheritance_chain(
        &versions_root, &profile, &mut visited, &mut chain, &mut notices, 0,
    );

    let libraries = merge_libraries(&chain);
    let (files, skipped_native, skipped_rules, unresolved) =
        build_forge_library_entries(&libraries)?;
    if files.is_empty() {
        return Err(format!(
            "No checksum-verifiable Java library artifacts could be planned for '{}'. Unresolved entries: {}.",
            selected_profile_id,
            unresolved.len()
        ));
    }

    let total_bytes = files.iter().try_fold(0u64, |total, file| {
        total.checked_add(file["size"].as_u64().unwrap_or(0))
            .ok_or_else(|| "Forge dependency download size overflow".to_string())
    })?;
    let missing_on_disk = files.iter().filter(|file| {
        file["path"].as_str()
            .map(|relative| !game_root.join(relative).is_file())
            .unwrap_or(true)
    }).count();

    let plan = json!({
        "minecraft_version": TARGET_VERSION,
        "instance": selected_profile_id,
        "plan_type": "forge_profile_java_libraries",
        "files": files,
        "library_count": libraries.len(),
        "skipped_native_classifiers": skipped_native,
        "skipped_non_android_rules": skipped_rules,
        "unresolved_libraries": unresolved,
        "total_bytes": total_bytes
    });
    let plan_path = profile_dir.join("rustchunk-dependency-plan.json");
    let bytes = serde_json::to_vec_pretty(&plan)
        .map_err(|error| format!("Could not encode Forge dependency plan: {}", error))?;
    write_atomic(&plan_path, &bytes)?;

    let unresolved_names = plan["unresolved_libraries"].as_array().cloned().unwrap_or_default();
    let planned_count = plan["files"].as_array().map(Vec::len).unwrap_or(0);
    let mut report = String::new();
    report.push_str("RustChunk Forge profile dependency plan (Stage 12)\n");
    report.push_str(&format!("Selected profile: {}\n", selected_profile_id));
    report.push_str(&format!("Merged library declarations: {}\n", libraries.len()));
    report.push_str(&format!("Checksum-verifiable Java artifacts: {}\n", planned_count));
    report.push_str(&format!("Artifacts not present on disk (pre-download check): {}\n", missing_on_disk));
    report.push_str(&format!("Native classifiers skipped: {}\n", skipped_native));
    report.push_str(&format!("Libraries excluded by Android/Linux ARM64 rules: {}\n", skipped_rules));
    report.push_str(&format!("Unresolved library declarations: {}\n", unresolved_names.len()));
    for item in unresolved_names.iter().take(30) {
        report.push_str(&format!("  UNRESOLVED: {}\n", item.as_str().unwrap_or("(unknown)")));
    }
    report.push_str(&format!("Estimated artifact data: {}\n", format_bytes(total_bytes)));
    report.push_str(&format!("Plan saved: {}\n", plan_path.display()));
    for notice in notices {
        report.push_str(&format!("NOTICE: {}\n", notice));
    }
    report.push_str("\nOnly HTTPS artifacts with valid SHA-1 values, declared sizes, and safe relative paths are eligible. This step does not install native libraries or launch Forge.\n");
    Ok(report)
}

fn build_forge_library_entries(
    libraries: &[Value],
) -> Result<(Vec<Value>, usize, usize, Vec<String>), String> {
    let mut by_path = BTreeMap::<String, Value>::new();
    let mut skipped_native = 0usize;
    let mut skipped_rules = 0usize;
    let mut unresolved = Vec::new();

    for library in libraries {
        if !library_rules_allow_android(library) {
            skipped_rules += 1;
            continue;
        }
        if is_any_native_library(library) {
            skipped_native += 1;
            continue;
        }

        let name = library.get("name").and_then(Value::as_str).unwrap_or("(unnamed library)");
        let path = match library_artifact_path(library) {
            Some(value) => match validate_relative_path(&value) {
                Ok(safe) => safe,
                Err(_) => {
                    unresolved.push(format!("{} (unsafe artifact path)", name));
                    continue;
                }
            },
            None => {
                unresolved.push(format!("{} (no Maven artifact path)", name));
                continue;
            }
        };

        let artifact = library.get("downloads").and_then(|value| value.get("artifact"));
        let url = artifact
            .and_then(|value| value.get("url"))
            .and_then(Value::as_str)
            .filter(|value| !value.trim().is_empty())
            .map(str::to_string)
            .or_else(|| {
                library.get("url").and_then(Value::as_str).map(|base| {
                    format!("{}/{}", base.trim_end_matches('/'), path.trim_start_matches('/'))
                })
            });
        let sha1 = artifact.and_then(|value| value.get("sha1")).and_then(Value::as_str);
        let size = artifact.and_then(|value| value.get("size")).and_then(Value::as_u64);

        let (Some(url), Some(sha1), Some(size)) = (url, sha1, size) else {
            unresolved.push(format!("{} (missing URL, SHA-1, or size metadata)", name));
            continue;
        };
        if !url.starts_with("https://") || !is_sha1_value(sha1) {
            unresolved.push(format!("{} (invalid HTTPS URL or SHA-1 metadata)", name));
            continue;
        }

        let entry = json!({
            "kind": "library",
            "url": url,
            "path": path,
            "sha1": sha1,
            "size": size
        });
        if let Some(existing) = by_path.get(&path) {
            if existing["sha1"] != entry["sha1"] || existing["size"] != entry["size"] {
                return Err(format!("Conflicting Forge library declarations target {}", path));
            }
        } else {
            by_path.insert(path, entry);
        }
    }
    Ok((by_path.into_values().collect(), skipped_native, skipped_rules, unresolved))
}

fn library_rules_allow_android(library: &Value) -> bool {
    let Some(rules) = library.get("rules").and_then(Value::as_array) else {
        return true;
    };
    let mut allowed = false;
    for rule in rules {
        if android_library_rule_matches(rule) {
            allowed = rule.get("action").and_then(Value::as_str) == Some("allow");
        }
    }
    allowed
}

fn android_library_rule_matches(rule: &Value) -> bool {
    if let Some(os) = rule.get("os") {
        if os.get("name").and_then(Value::as_str)
            .is_some_and(|name| name != "linux" && name != "android")
        {
            return false;
        }
        if os.get("arch").and_then(Value::as_str)
            .is_some_and(|arch| arch != "aarch64" && arch != "arm64" && arch != "arm64-v8a")
        {
            return false;
        }
        if os.get("version").is_some() {
            return false;
        }
    }
    if let Some(features) = rule.get("features").and_then(Value::as_object) {
        if features.values().any(|required| required.as_bool() == Some(true)) {
            return false;
        }
    }
    true
}

fn is_any_native_library(library: &Value) -> bool {
    let name = library.get("name").and_then(Value::as_str).unwrap_or("");
    let classifier = name.split(':').nth(3).unwrap_or("");
    if classifier.starts_with("natives-") {
        return true;
    }
    let artifact_path = library.get("downloads")
        .and_then(|value| value.get("artifact"))
        .and_then(|value| value.get("path"))
        .and_then(Value::as_str)
        .unwrap_or("");
    artifact_path.rsplit('/').next()
        .is_some_and(|filename| filename.contains("-natives-"))
}

fn is_sha1_value(value: &str) -> bool {
    value.len() == 40 && value.chars().all(|character| character.is_ascii_hexdigit())
}

fn discover_profiles(versions_root: &Path) -> Result<Vec<(PathBuf, PathBuf, Value)>, String> {
    let entries = fs::read_dir(versions_root)
        .map_err(|error| format!("Cannot list versions directory: {}", error))?;
    let mut candidates = Vec::new();

    for entry in entries {
        let entry = entry.map_err(|error| format!("Cannot read version entry: {}", error))?;
        let directory = entry.path();
        if !directory.is_dir() { continue; }

        let files = match fs::read_dir(&directory) {
            Ok(value) => value,
            Err(_) => continue,
        };
        for file in files.flatten() {
            let path = file.path();
            if path.extension().and_then(|value| value.to_str()) != Some("json") { continue; }
            let Ok(bytes) = fs::read(&path) else { continue };
            let Ok(value) = serde_json::from_slice::<Value>(&bytes) else { continue };
            let Some(id) = value.get("id").and_then(Value::as_str) else { continue };
            if !is_safe_version_id(id) { continue; }
            let looks_like_profile = value.get("mainClass").is_some()
                || value.get("inheritsFrom").is_some()
                || value.get("libraries").is_some();
            if looks_like_profile {
                candidates.push((directory.clone(), path, value));
            }
        }
    }

    candidates.sort_by(|left, right| {
        let left_id = left.2.get("id").and_then(Value::as_str).unwrap_or("");
        let right_id = right.2.get("id").and_then(Value::as_str).unwrap_or("");
        let left_exact = left_id == left.0.file_name().and_then(|value| value.to_str()).unwrap_or("");
        let right_exact = right_id == right.0.file_name().and_then(|value| value.to_str()).unwrap_or("");
        right_exact.cmp(&left_exact).then_with(|| left_id.cmp(right_id))
    });
    Ok(candidates)
}

fn find_forge_profile(
    versions_root: &Path,
    selected_profile_id: &str,
) -> Result<(PathBuf, PathBuf, Value), String> {
    let candidates = discover_profiles(versions_root)?;
    let matches: Vec<_> = candidates.into_iter()
        .filter(|(_, _, profile)| profile.get("id").and_then(Value::as_str) == Some(selected_profile_id))
        .collect();
    let Some((directory, json_path, profile)) = matches.into_iter().next() else {
        return Err(format!("The selected profile '{}' was not found under versions/.", selected_profile_id));
    };
    if profile_loader(&profile) != "Forge" {
        return Err(format!("The selected profile '{}' is not a Forge profile.", selected_profile_id));
    }
    let base_version = profile_minecraft_version(&profile, selected_profile_id);
    if base_version != TARGET_VERSION {
        return Err(format!(
            "The selected Forge profile targets Minecraft {}, while this stage supports {}.",
            base_version, TARGET_VERSION
        ));
    }
    Ok((directory, json_path, profile))
}

fn profile_loader(profile: &Value) -> &'static str {
    let id = profile.get("id").and_then(Value::as_str).unwrap_or("").to_lowercase();
    let main = profile.get("mainClass").and_then(Value::as_str).unwrap_or("").to_lowercase();
    if id.contains("neoforge") { "NeoForge" }
    else if id.contains("forge") || main.contains("bootstraplauncher") { "Forge" }
    else if id.contains("quilt") || main.contains("org.quiltmc") { "Quilt" }
    else if id.contains("fabric") || main.contains("net.fabricmc") { "Fabric" }
    else { "Vanilla" }
}

fn profile_minecraft_version(profile: &Value, id: &str) -> String {
    if let Some(parent) = profile.get("inheritsFrom").and_then(Value::as_str)
        .filter(|value| !value.trim().is_empty())
    {
        return parent.to_string();
    }

    // Some launchers export a standalone Forge profile without inheritsFrom.
    // In that case, recover the Minecraft version from IDs such as
    // "1.20.1 Forge 47.4.22" or "1.20.1-forge-47.4.22".
    let prefix = id.split(|character: char| character.is_ascii_whitespace() || character == '-')
        .next().unwrap_or(id);
    let looks_like_version = prefix.chars().next().is_some_and(|value| value.is_ascii_digit())
        && prefix.contains('.');
    if looks_like_version { prefix.to_string() } else { id.to_string() }
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
    // Parent entries establish order; a child entry with the same Maven name overrides it.
    let mut positions: HashMap<String, usize> = HashMap::new();
    let mut merged: Vec<Value> = Vec::new();
    for profile in chain {
        if let Some(items) = profile.get("libraries").and_then(Value::as_array) {
            for item in items {
                let key = item.get("name").and_then(Value::as_str)
                    .map(str::to_string)
                    .unwrap_or_else(|| format!("anonymous-{}", merged.len()));
                if let Some(index) = positions.get(&key).copied() {
                    merged[index] = item.clone();
                } else {
                    positions.insert(key, merged.len());
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
    // Legacy minecraftArguments is a single inherited value, not a parent+child concatenation.
    chain.iter().rev()
        .find_map(|profile| profile.get("minecraftArguments").and_then(Value::as_str))
        .map(str::to_string)
}

fn is_desktop_native_library(library: &Value) -> bool {
    let name = library.get("name").and_then(Value::as_str).unwrap_or("");
    let classifier = name.split(':').nth(3).unwrap_or("");
    if classifier.starts_with("natives-") && !classifier.contains("android") {
        return true;
    }
    let artifact_path = library.get("downloads")
        .and_then(|value| value.get("artifact"))
        .and_then(|value| value.get("path"))
        .and_then(Value::as_str).unwrap_or("");
    let filename = artifact_path.rsplit('/').next().unwrap_or("");
    filename.contains("-natives-") && !filename.contains("natives-android")
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

fn runtime_search_roots(app_data_dir: &Path) -> Vec<PathBuf> {
    let game_root = app_data_dir.join("minecraft");
    vec![
        app_data_dir.join("runtime"),
        app_data_dir.join("runtimes"),
        app_data_dir.join("games").join("runtimes"),
        game_root.join("runtime"),
    ]
}

fn find_runtime_candidates(runtime_root: &Path) -> Vec<RuntimeCandidate> {
    if !runtime_root.is_dir() { return Vec::new(); }

    let mut pending = vec![(runtime_root.to_path_buf(), 0usize)];
    let mut visited = 0usize;
    let mut java_executables = Vec::new();
    let mut jvm_libraries = Vec::new();

    while let Some((directory, depth)) = pending.pop() {
        if visited >= MAX_SCAN_ENTRIES || depth > 10 { break; }
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
        let matching_jvm = jvm_libraries.iter()
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
    candidates.sort_by(|left, right| left.java_executable.cmp(&right.java_executable));
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
            character.is_ascii_alphanumeric() || "._- +".contains(character)
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

fn format_bytes(bytes: u64) -> String {
    const MIB: u64 = 1024 * 1024;
    format!("{:.1} MiB", bytes as f64 / MIB as f64)
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
    fn child_library_overrides_parent_with_same_coordinate() {
        let parent: Value = serde_json::from_str(r#"{"libraries":[{"name":"org.example:demo:1.0","marker":"parent"}]}"#).unwrap();
        let child: Value = serde_json::from_str(r#"{"libraries":[{"name":"org.example:demo:1.0","marker":"child"}]}"#).unwrap();
        let merged = merge_libraries(&[parent, child]);
        assert_eq!(merged.len(), 1);
        assert_eq!(merged[0]["marker"], "child");
    }

    #[test]
    fn legacy_arguments_use_nearest_child_value_without_concatenation() {
        let parent: Value = serde_json::from_str(r#"{"minecraftArguments":"--username parent"}"#).unwrap();
        let child: Value = serde_json::from_str(r#"{"minecraftArguments":"--username child"}"#).unwrap();
        assert_eq!(
            merge_legacy_arguments(&[parent, child]).as_deref(),
            Some("--username child")
        );
    }

    #[test]
    fn desktop_native_classifiers_are_not_required_for_android_audit() {
        let linux: Value = serde_json::from_str(r#"{"name":"org.lwjgl:lwjgl:3.3.1:natives-linux"}"#).unwrap();
        let ordinary: Value = serde_json::from_str(r#"{"name":"org.lwjgl:lwjgl:3.3.1"}"#).unwrap();
        let android: Value = serde_json::from_str(r#"{"name":"org.example:demo:1.0:natives-android"}"#).unwrap();
        assert!(is_desktop_native_library(&linux));
        assert!(!is_desktop_native_library(&ordinary));
        assert!(!is_desktop_native_library(&android));
    }


    #[test]
    fn identifies_loader_families_and_inherited_minecraft_version() {
        let forge: Value = serde_json::from_str(
            r#"{"id":"1.20.1 Forge 47.4.22","inheritsFrom":"1.20.1","mainClass":"cpw.mods.bootstraplauncher.BootstrapLauncher"}"#
        ).unwrap();
        let fabric: Value = serde_json::from_str(
            r#"{"id":"fabric-loader-0.15.11-1.20.1","inheritsFrom":"1.20.1","mainClass":"net.fabricmc.loader.impl.launch.knot.KnotClient"}"#
        ).unwrap();
        let neo: Value = serde_json::from_str(
            r#"{"id":"1.20.1-neoforge-20.4.1","inheritsFrom":"1.20.1","mainClass":"cpw.mods.bootstraplauncher.BootstrapLauncher"}"#
        ).unwrap();
        assert_eq!(profile_loader(&forge), "Forge");
        assert_eq!(profile_loader(&fabric), "Fabric");
        assert_eq!(profile_loader(&neo), "NeoForge");
        assert_eq!(profile_minecraft_version(&forge, "fallback"), "1.20.1");

        let standalone_forge: Value = serde_json::from_str(
            r#"{"id":"1.20.1 Forge 47.4.22","mainClass":"cpw.mods.bootstraplauncher.BootstrapLauncher"}"#
        ).unwrap();
        assert_eq!(
            profile_minecraft_version(&standalone_forge, "1.20.1 Forge 47.4.22"),
            "1.20.1"
        );

        let hyphenated_forge: Value = serde_json::from_str(
            r#"{"id":"1.20.1-forge-47.4.22","mainClass":"cpw.mods.bootstraplauncher.BootstrapLauncher"}"#
        ).unwrap();
        assert_eq!(
            profile_minecraft_version(&hyphenated_forge, "1.20.1-forge-47.4.22"),
            "1.20.1"
        );
    }

    #[test]
    fn selected_profile_resolves_to_its_own_version_directory() {
        let root = std::env::temp_dir().join(format!("rustchunk-profile-select-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        let first = root.join("1.20.1 Forge 47.4.5");
        let second = root.join("1.20.1 Forge 47.4.22");
        fs::create_dir_all(&first).unwrap();
        fs::create_dir_all(&second).unwrap();
        fs::write(
            first.join("1.20.1 Forge 47.4.5.json"),
            r#"{"id":"1.20.1 Forge 47.4.5","inheritsFrom":"1.20.1","mainClass":"cpw.mods.bootstraplauncher.BootstrapLauncher"}"#
        ).unwrap();
        fs::write(
            second.join("1.20.1 Forge 47.4.22.json"),
            r#"{"id":"1.20.1 Forge 47.4.22","inheritsFrom":"1.20.1","mainClass":"cpw.mods.bootstraplauncher.BootstrapLauncher"}"#
        ).unwrap();
        let (selected_dir, _, selected) =
            find_forge_profile(&root, "1.20.1 Forge 47.4.22").unwrap();
        assert_eq!(selected_dir, second);
        assert_eq!(selected["id"], "1.20.1 Forge 47.4.22");
        fs::remove_dir_all(root).unwrap();
    }

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
    fn includes_zalith_runtime_directory_in_search_roots() {
        let app = Path::new("/data/user/0/dev.rustchunk.app/files");
        let roots = runtime_search_roots(app);
        assert!(roots.contains(&app.join("games").join("runtimes")));
        assert!(roots.contains(&app.join("minecraft").join("runtime")));
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

    #[test]
    fn forge_dependency_plan_includes_verified_java_artifacts_and_skips_natives() {
        let libraries: Vec<Value> = serde_json::from_str(r#"[
            {"name":"org.example:core:1.0","rules":[{"action":"allow","os":{"name":"linux","arch":"aarch64"}}],"downloads":{"artifact":{"path":"org/example/core/1.0/core-1.0.jar","url":"https://repo.example/core-1.0.jar","sha1":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","size":123}}},
            {"name":"org.lwjgl:lwjgl:3.3.1:natives-linux","downloads":{"artifact":{"path":"org/lwjgl/lwjgl/3.3.1/lwjgl-3.3.1-natives-linux.jar","url":"https://repo.example/native.jar","sha1":"bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb","size":456}}},
            {"name":"org.example:windows-only:1.0","rules":[{"action":"allow","os":{"name":"windows"}}],"downloads":{"artifact":{"path":"org/example/windows-only/1.0/windows-only-1.0.jar","url":"https://repo.example/windows.jar","sha1":"cccccccccccccccccccccccccccccccccccccccc","size":789}}}
        ]"#).unwrap();
        let (entries, native_count, rule_count, unresolved) = build_forge_library_entries(&libraries).unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0]["path"], "org/example/core/1.0/core-1.0.jar");
        assert_eq!(native_count, 1);
        assert_eq!(rule_count, 1);
        assert!(unresolved.is_empty());
    }

    #[test]
    fn forge_dependency_plan_rejects_unverified_or_unsafe_artifacts() {
        let libraries: Vec<Value> = serde_json::from_str(r#"[
            {"name":"org.example:no-checksum:1.0","downloads":{"artifact":{"path":"org/example/no-checksum/1.0/no-checksum-1.0.jar","url":"https://repo.example/file.jar","size":4}}},
            {"name":"org.example:unsafe:1.0","downloads":{"artifact":{"path":"../../outside.jar","url":"https://repo.example/file.jar","sha1":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","size":4}}}
        ]"#).unwrap();
        let (entries, _, _, unresolved) = build_forge_library_entries(&libraries).unwrap();
        assert!(entries.is_empty());
        assert_eq!(unresolved.len(), 2);
        assert!(unresolved.iter().any(|value| value.contains("missing URL")));
        assert!(unresolved.iter().any(|value| value.contains("unsafe artifact path")));
    }

}
