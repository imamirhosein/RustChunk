use md5::{Digest, Md5};
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


pub fn prepare_jni_forge_launch_plan(
    app_data_dir: &Path,
    selected_profile_id: &str,
    offline_username: &str,
    app_native_library_dir: &Path,
) -> Result<String, String> {
    if !app_data_dir.is_absolute() || !app_native_library_dir.is_absolute() {
        return Err("RustChunk data and Android native-library paths must be absolute.".to_string());
    }
    if !is_safe_version_id(selected_profile_id) {
        return Err("Selected Forge profile identifier is invalid.".to_string());
    }
    if offline_username.is_empty() || offline_username.len() > 16
        || !offline_username.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        return Err("Offline username must contain 1-16 ASCII letters, numbers, or underscores.".to_string());
    }

    let game_root = app_data_dir.join("minecraft");
    let versions_root = game_root.join("versions");
    let (profile_dir, profile_json_path, profile) = find_forge_profile(&versions_root, selected_profile_id)?;
    let profile_id = profile.get("id").and_then(Value::as_str).unwrap_or(selected_profile_id).to_string();

    let mut chain = Vec::new();
    let mut visited = HashSet::new();
    let mut notices = Vec::new();
    collect_inheritance_chain(&versions_root, &profile, &mut visited, &mut chain, &mut notices, 0);

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

    if profile_loader(&profile) != "Forge" || minecraft_version != TARGET_VERSION {
        return Err(format!("Selected profile '{}' is not a supported Minecraft {} Forge profile.", profile_id, TARGET_VERSION));
    }
    let main_class = inherited_string(&chain, "mainClass")
        .ok_or_else(|| "Forge profile does not declare a mainClass.".to_string())?;
    if main_class != "cpw.mods.bootstraplauncher.BootstrapLauncher" {
        return Err(format!("Unexpected Forge 1.20.1 main class '{}'.", main_class));
    }

    let library_dir = game_root.join("libraries");
    let natives_dir = profile_dir.join("rustchunk-natives");
    fs::create_dir_all(&natives_dir)
        .map_err(|error| format!("Cannot create native-library staging directory: {}", error))?;

    let mut classpath_entries = Vec::<String>::new();
    let mut classpath_seen = HashSet::<String>::new();
    let mut missing_classpath = Vec::<String>::new();
    let mut skipped_native = 0usize;
    let mut skipped_rules = 0usize;
    let mut unresolved_libraries = Vec::<String>::new();

    for library in merge_libraries(&chain).iter() {
        if !library_rules_allow_android(library) { skipped_rules += 1; continue; }
        if is_any_native_library(library) || is_desktop_native_library(library) {
            skipped_native += 1;
            continue;
        }
        let name = library.get("name").and_then(Value::as_str).unwrap_or("(unnamed library)");
        let Some(relative) = library_artifact_path(library) else {
            unresolved_libraries.push(format!("{} (no artifact path)", name));
            continue;
        };
        let relative = match validate_relative_path(&relative) {
            Ok(value) => value,
            Err(_) => {
                unresolved_libraries.push(format!("{} (unsafe artifact path)", name));
                continue;
            }
        };
        let extension = Path::new(&relative).extension().and_then(|v| v.to_str()).unwrap_or("");
        if extension != "jar" && extension != "zip" { continue; }

        let absolute = library_dir.join(&relative);
        let key = absolute.to_string_lossy().into_owned();
        if !classpath_seen.insert(key.clone()) { continue; }
        if absolute.is_file() { classpath_entries.push(key); }
        else { missing_classpath.push(relative); }
    }

    let client_jar = find_client_jar(&game_root, &profile_dir, &profile, &minecraft_version);
    let client_jar_present = client_jar.as_ref().is_some_and(|path| path.is_file());
    if let Some(path) = &client_jar {
        if client_jar_present {
            let key = path.to_string_lossy().into_owned();
            if classpath_seen.insert(key.clone()) { classpath_entries.push(key); }
        }
    } else {
        missing_classpath.push(format!("versions/{}/{}.jar", minecraft_version, minecraft_version));
    }

    let classpath = classpath_entries.join(":");
    let jvm_raw = flatten_launch_arguments(&merge_arguments(&chain, "jvm"));
    let mut game_raw = flatten_launch_arguments(&merge_arguments(&chain, "game"));
    if game_raw.is_empty() {
        game_raw = merge_legacy_arguments(&chain).unwrap_or_default()
            .split_whitespace().map(str::to_string).collect();
    }

    let asset_index_id = inherited_value(&chain, "assetIndex")
        .and_then(|value| value.get("id")).and_then(Value::as_str).map(str::to_string)
        .or_else(|| inherited_string(&chain, "assets"));
    let assets_root = game_root.join("assets");
    let assets_index_path = asset_index_id.as_ref()
        .map(|id| assets_root.join("indexes").join(format!("{}.json", id)));
    let assets_index_present = assets_index_path.as_ref().is_some_and(|path| path.is_file());
    let offline_uuid = offline_player_uuid(offline_username);
    let version_type = profile.get("type").and_then(Value::as_str).unwrap_or("release");

    let keys = [
        "auth_player_name", "version_name", "game_directory", "assets_root", "assets_index_name",
        "auth_uuid", "auth_access_token", "user_type", "version_type", "user_properties",
        "resolution_width", "resolution_height", "game_assets", "auth_session", "library_directory",
        "classpath_separator", "classpath", "natives_directory", "launcher_name", "launcher_version",
        "clientid", "auth_xuid", "quickPlayPath", "quickPlaySingleplayer", "quickPlayMultiplayer",
        "quickPlayRealms"
    ];
    let values = [
        offline_username.to_string(),
        profile_id.clone(),
        profile_dir.to_string_lossy().into_owned(),
        assets_root.to_string_lossy().into_owned(),
        asset_index_id.clone().unwrap_or_else(|| minecraft_version.clone()),
        offline_uuid.clone(),
        "0".to_string(),
        "legacy".to_string(),
        version_type.to_string(),
        "{}".to_string(),
        "854".to_string(),
        "480".to_string(),
        assets_root.join("virtual/legacy").to_string_lossy().into_owned(),
        "0".to_string(),
        library_dir.to_string_lossy().into_owned(),
        ":".to_string(),
        classpath.clone(),
        natives_dir.to_string_lossy().into_owned(),
        "RustChunk".to_string(),
        "0.18.0".to_string(),
        String::new(),
        String::new(),
        String::new(),
        String::new(),
        String::new(),
        String::new(),
    ];
    let replacements: Vec<(String, String)> = keys.iter().zip(values.iter())
        .map(|(key, value)| (placeholder(key), value.clone())).collect();

    let (mut jvm_options, module_path_values, unsupported_jvm_arguments) =
        normalize_jvm_arguments(&jvm_raw, &replacements);
    let mut module_path_entries = Vec::<String>::new();
    let mut missing_module_path = Vec::<String>::new();
    for module_path in &module_path_values {
        for entry in module_path.split(':').filter(|value| !value.trim().is_empty()) {
            let entry = entry.trim().to_string();
            if !module_path_entries.contains(&entry) {
                if Path::new(&entry).exists() { module_path_entries.push(entry); }
                else { missing_module_path.push(entry); }
            }
        }
    }

    let mut game_arguments: Vec<String> = game_raw.iter()
        .map(|value| replace_launch_placeholders(value, &replacements)).collect();
    if !game_arguments.iter().any(|value| value == "--width") {
        game_arguments.extend(["--width".to_string(), "854".to_string()]);
    }
    if !game_arguments.iter().any(|value| value == "--height") {
        game_arguments.extend(["--height".to_string(), "480".to_string()]);
    }

    let mut unresolved_placeholders = collect_unresolved_placeholders(
        jvm_options.iter().chain(game_arguments.iter()).map(String::as_str)
    );
    if classpath_entries.is_empty() {
        unresolved_placeholders.push(format!("{}{{classpath}} (no resolved classpath entries)", "$"));
    }
    unresolved_placeholders.sort();
    unresolved_placeholders.dedup();

    ensure_vm_property(&mut jvm_options, "java.class.path", &classpath);
    ensure_vm_property(&mut jvm_options, "java.home", &app_data_dir.join("games/runtimes/Internal-17").to_string_lossy());
    ensure_vm_property(&mut jvm_options, "java.io.tmpdir", &app_data_dir.join("cache/java").to_string_lossy());
    ensure_vm_property(&mut jvm_options, "user.home", &profile_dir.to_string_lossy());
    ensure_vm_property(&mut jvm_options, "java.library.path",
        &format!("{}:{}", natives_dir.to_string_lossy(), app_native_library_dir.to_string_lossy()));
    ensure_vm_property(&mut jvm_options, "org.lwjgl.librarypath", &natives_dir.to_string_lossy());
    ensure_vm_property(&mut jvm_options, "org.lwjgl.system.SharedLibraryExtractPath", &natives_dir.to_string_lossy());
    ensure_vm_property(&mut jvm_options, "jna.tmpdir", &natives_dir.to_string_lossy());
    ensure_vm_property(&mut jvm_options, "jna.boot.library.path", &app_native_library_dir.to_string_lossy());
    ensure_vm_property(&mut jvm_options, "fml.earlyprogresswindow", "false");
    ensure_vm_property(&mut jvm_options, "log4j2.formatMsgNoLookups", "true");
    ensure_vm_property(&mut jvm_options, "net.minecraft.clientmodname", "RustChunk");
    if !jvm_options.iter().any(|value| value.starts_with("-Xms")) {
        jvm_options.push("-Xms256m".to_string());
    }
    if !jvm_options.iter().any(|value| value.starts_with("-Xmx")) {
        jvm_options.push("-Xmx2048m".to_string());
    }
    if !jvm_options.iter().any(|value| value.starts_with("-XX:ActiveProcessorCount=")) {
        let processors = std::thread::available_parallelism().map(|v| v.get()).unwrap_or(4);
        jvm_options.push(format!("-XX:ActiveProcessorCount={}", processors));
    }

    let mut blockers = Vec::<String>::new();
    if classpath_entries.is_empty() { blockers.push("Java classpath is empty.".to_string()); }
    if !missing_classpath.is_empty() { blockers.push(format!("{} classpath artifact(s) are missing.", missing_classpath.len())); }
    if !missing_module_path.is_empty() { blockers.push(format!("{} Forge module-path artifact(s) are missing.", missing_module_path.len())); }
    if module_path_entries.is_empty() { blockers.push("Forge module path is empty.".to_string()); }
    if !unresolved_placeholders.is_empty() { blockers.push(format!("{} launch placeholder(s) remain unresolved.", unresolved_placeholders.len())); }
    if !unsupported_jvm_arguments.is_empty() { blockers.push(format!("{} JVM argument token(s) could not be interpreted safely.", unsupported_jvm_arguments.len())); }
    if !unresolved_libraries.is_empty() { blockers.push(format!("{} library declaration(s) could not be mapped to JAR files.", unresolved_libraries.len())); }
    if !client_jar_present { blockers.push("Minecraft client JAR is missing.".to_string()); }
    if !assets_index_present { blockers.push("Minecraft asset index is missing.".to_string()); }

    let metadata_ready = blockers.is_empty();
    let plan = json!({
        "schema_version": 1,
        "status": "prepared_for_review_only",
        "profile_id": profile_id.clone(),
        "profile_json": relative_path(&game_root, &profile_json_path),
        "profile_directory": relative_path(&game_root, &profile_dir),
        "minecraft_version": minecraft_version.clone(),
        "main_class": main_class.clone(),
        "offline_identity": {
            "username": offline_username,
            "uuid": offline_uuid.clone(),
            "access_token": "0",
            "user_type": "legacy",
            "server_compatibility_note": "Only for servers configured for offline/cracked authentication. Not a Microsoft/Mojang login."
        },
        "working_directory": profile_dir.to_string_lossy(),
        "classpath": classpath_entries.clone(),
        "classpath_entry_count": classpath_entries.len(),
        "module_path": module_path_entries.clone(),
        "module_path_entry_count": module_path_entries.len(),
        "jni_vm_options": jvm_options.clone(),
        "main_arguments": game_arguments.clone(),
        "missing_classpath_artifacts": missing_classpath.clone(),
        "missing_module_path_artifacts": missing_module_path.clone(),
        "unresolved_library_declarations": unresolved_libraries.clone(),
        "unsupported_jvm_arguments": unsupported_jvm_arguments.clone(),
        "unresolved_placeholders": unresolved_placeholders.clone(),
        "native_classifiers_skipped": skipped_native,
        "libraries_excluded_by_platform_rules": skipped_rules,
        "client_jar_present": client_jar_present,
        "asset_index_present": assets_index_present,
        "assets_index": asset_index_id,
        "metadata_ready": metadata_ready,
        "android_native_lwjgl_ready": false,
        "android_graphics_bridge_ready": false,
        "game_launch_ready": false,
        "launch_performed": false,
        "launch_blockers": blockers.clone(),
        "runtime_note": "Java 17 JNI startup was verified separately. Android LWJGL native mapping, OpenGL/EGL surface integration, and actual Forge execution are not implemented by this planning step."
    });

    let plan_path = profile_dir.join("rustchunk-jni-launch-plan.json");
    let report_path = profile_dir.join("rustchunk-jni-launch-plan-report.txt");
    let plan_bytes = serde_json::to_vec_pretty(&plan)
        .map_err(|error| format!("Could not encode JNI Forge launch plan: {}", error))?;
    write_atomic(&plan_path, &plan_bytes)?;

    let mut report = String::new();
    report.push_str("RustChunk Forge JNI launch-plan preparation (Stage 19)\n");
    report.push_str(&format!("Selected profile: {}\n", profile_id));
    report.push_str(&format!("Minecraft target: {}\n", minecraft_version));
    report.push_str(&format!("Main class: {}\n", main_class));
    report.push_str(&format!("Offline username: {}\n", offline_username));
    report.push_str(&format!("Offline UUID: {}\n", offline_uuid));
    report.push_str(&format!("Java classpath entries present: {}\n", classpath_entries.len()));
    report.push_str(&format!("Forge module-path entries present: {}\n", module_path_entries.len()));
    report.push_str(&format!("Missing classpath artifacts: {}\n", missing_classpath.len()));
    report.push_str(&format!("Missing module-path artifacts: {}\n", missing_module_path.len()));
    report.push_str(&format!("Unresolved library declarations: {}\n", unresolved_libraries.len()));
    report.push_str(&format!("Unresolved launch placeholders: {}\n", unresolved_placeholders.len()));
    report.push_str(&format!("Unsupported JVM argument tokens: {}\n", unsupported_jvm_arguments.len()));
    report.push_str(&format!("Native library declarations skipped: {}\n", skipped_native));
    report.push_str(&format!("Libraries excluded by Android/Linux ARM64 rules: {}\n", skipped_rules));
    report.push_str(&format!("Minecraft client JAR present: {}\n", client_jar_present));
    report.push_str(&format!("Minecraft asset index present: {}\n", assets_index_present));
    report.push_str(&format!("Launch metadata ready: {}\n", metadata_ready));
    report.push_str("Android LWJGL/OpenGL integration ready: NO\n");
    report.push_str("Actual Forge launch performed: NO\n");
    for blocker in &blockers { report.push_str(&format!("BLOCKER: {}\n", blocker)); }
    for item in missing_classpath.iter().take(30) { report.push_str(&format!("MISSING CLASSPATH: {}\n", item)); }
    for item in missing_module_path.iter().take(30) { report.push_str(&format!("MISSING MODULE PATH: {}\n", item)); }
    for item in unsupported_jvm_arguments.iter().take(20) { report.push_str(&format!("UNSUPPORTED JVM ARGUMENT: {}\n", item)); }
    for notice in notices { report.push_str(&format!("NOTICE: {}\n", notice)); }
    report.push_str(&format!("Launch plan JSON: {}\n", plan_path.display()));
    report.push_str(&format!("Launch plan report: {}\n", report_path.display()));
    report.push_str("\nThis step converts the Forge profile into JNI_CreateJavaVM options and main-class arguments. It does not execute Forge, create an OpenGL surface, or connect to a server.\n");

    write_atomic(&report_path, report.as_bytes())?;
    Ok(report)
}

fn placeholder(key: &str) -> String {
    format!("{}{{{}}}", "$", key)
}

fn offline_player_uuid(username: &str) -> String {
    let digest = Md5::digest(format!("OfflinePlayer:{}", username).as_bytes());
    let mut bytes = digest.to_vec();
    bytes[6] = (bytes[6] & 0x0f) | 0x30;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;
    format!(
        "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        bytes[0], bytes[1], bytes[2], bytes[3],
        bytes[4], bytes[5], bytes[6], bytes[7],
        bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15]
    )
}

fn flatten_launch_arguments(arguments: &[Value]) -> Vec<String> {
    let mut flattened = Vec::new();
    for argument in arguments {
        match argument {
            Value::String(value) => flattened.push(value.clone()),
            Value::Object(object) => {
                if !launch_argument_rules_allow_android(argument) { continue; }
                let Some(value) = object.get("value") else { continue };
                match value {
                    Value::String(value) => flattened.push(value.clone()),
                    Value::Array(values) => flattened.extend(values.iter().filter_map(Value::as_str).map(str::to_string)),
                    _ => {}
                }
            }
            _ => {}
        }
    }
    flattened
}

fn launch_argument_rules_allow_android(argument: &Value) -> bool {
    let Some(rules) = argument.get("rules").and_then(Value::as_array) else { return true; };
    let mut allowed = false;
    for rule in rules {
        if launch_argument_rule_matches(rule) {
            allowed = rule.get("action").and_then(Value::as_str) == Some("allow");
        }
    }
    allowed
}

fn launch_argument_rule_matches(rule: &Value) -> bool {
    if let Some(os) = rule.get("os") {
        if os.get("name").and_then(Value::as_str)
            .is_some_and(|name| name != "linux" && name != "android") { return false; }
        if os.get("arch").and_then(Value::as_str)
            .is_some_and(|arch| arch != "aarch64" && arch != "arm64" && arch != "arm64-v8a") { return false; }
        if os.get("version").is_some() { return false; }
    }
    if let Some(features) = rule.get("features").and_then(Value::as_object) {
        for (name, required) in features {
            let actual = match name.as_str() {
                "has_custom_resolution" => true,
                "is_demo_user" => false,
                "has_quick_plays_support" => false,
                "is_quick_play_singleplayer" => false,
                "is_quick_play_multiplayer" => false,
                "is_quick_play_realms" => false,
                _ => false,
            };
            if required.as_bool() != Some(actual) { return false; }
        }
    }
    true
}

fn replace_launch_placeholders(value: &str, replacements: &[(String, String)]) -> String {
    let mut result = value.to_string();
    for (placeholder, replacement) in replacements { result = result.replace(placeholder, replacement); }
    result
}

fn normalize_jvm_arguments(
    raw: &[String],
    replacements: &[(String, String)],
) -> (Vec<String>, Vec<String>, Vec<String>) {
    let mut options = Vec::new();
    let mut module_paths = Vec::new();
    let mut unsupported = Vec::new();
    let mut index = 0usize;
    while index < raw.len() {
        let argument = &raw[index];
        if argument == "-p" || argument == "--module-path" {
            if let Some(value) = raw.get(index + 1) {
                let value = replace_launch_placeholders(value, replacements);
                module_paths.push(value.clone());
                options.push(format!("--module-path={}", value));
                index += 2;
                continue;
            }
            unsupported.push(format!("{} (missing value)", argument));
            index += 1;
            continue;
        }
        if argument == "-cp" || argument == "-classpath" || argument == "--class-path" {
            if raw.get(index + 1).is_some() { index += 2; }
            else { unsupported.push(format!("{} (missing value)", argument)); index += 1; }
            continue;
        }

        const VALUE_OPTIONS: [&str; 7] = [
            "--add-modules", "--add-exports", "--add-opens", "--add-reads",
            "--limit-modules", "--patch-module", "--upgrade-module-path",
        ];
        if VALUE_OPTIONS.contains(&argument.as_str()) {
            if let Some(value) = raw.get(index + 1) {
                let value = replace_launch_placeholders(value, replacements);
                options.push(format!("{}={}", argument, value));
                index += 2;
                continue;
            }
            unsupported.push(format!("{} (missing value)", argument));
            index += 1;
            continue;
        }

        let resolved = replace_launch_placeholders(argument, replacements);
        if resolved.starts_with("-Djava.class.path=") {
            // Use the class path assembled from verified artifacts.
        } else if resolved.starts_with("--module-path=") {
            let value = resolved["--module-path=".len()..].to_string();
            module_paths.push(value);
            options.push(resolved);
        } else if resolved.starts_with("--add-modules=")
            || resolved.starts_with("--add-exports=")
            || resolved.starts_with("--add-opens=")
            || resolved.starts_with("--add-reads=")
            || resolved.starts_with("--limit-modules=")
            || resolved.starts_with("--patch-module=")
            || resolved.starts_with("--upgrade-module-path=")
            || resolved.starts_with("-D")
            || resolved.starts_with("-X")
            || resolved.starts_with("-XX:")
            || resolved.starts_with("-agentlib:")
            || resolved.starts_with("-javaagent:")
            || resolved == "--enable-preview"
        {
            options.push(resolved);
        } else if resolved.starts_with('-') {
            unsupported.push(resolved);
        } else {
            unsupported.push(format!("unexpected JVM argument token: {}", resolved));
        }
        index += 1;
    }
    (options, module_paths, unsupported)
}

fn ensure_vm_property(options: &mut Vec<String>, name: &str, value: &str) {
    let prefix = format!("-D{}=", name);
    options.retain(|argument| !argument.starts_with(&prefix));
    options.push(format!("{}{}", prefix, value));
}

fn collect_unresolved_placeholders<'a>(arguments: impl Iterator<Item = &'a str>) -> Vec<String> {
    let mut unresolved = Vec::new();
    let opening = format!("{}{{", "$");
    for argument in arguments {
        let mut rest = argument;
        while let Some(start) = rest.find(&opening) {
            let after = &rest[start..];
            if let Some(end) = after.find('}') {
                unresolved.push(after[..=end].to_string());
                rest = &after[end + 1..];
            } else {
                unresolved.push(after.to_string());
                break;
            }
        }
    }
    unresolved.sort();
    unresolved.dedup();
    unresolved
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
    fn offline_uuid_uses_name_based_version_three_format() {
        let uuid = offline_player_uuid("RustPlayer");
        assert_eq!(uuid.len(), 36);
        assert_eq!(uuid.chars().filter(|character| *character == '-').count(), 4);
        assert_eq!(uuid.chars().nth(14), Some('3'));
        assert!(matches!(uuid.chars().nth(19), Some('8' | '9' | 'a' | 'b')));
        assert_eq!(uuid, offline_player_uuid("RustPlayer"));
    }

    #[test]
    fn flattens_profile_arguments_and_applies_android_rules() {
        let values: Vec<Value> = serde_json::from_str(
            r#"[
                "--always",
                {"rules":[{"action":"allow","os":{"name":"linux"}}],"value":["--linux","yes"]},
                {"rules":[{"action":"allow","os":{"name":"windows"}}],"value":"--windows"}
            ]"#
        ).unwrap();
        assert_eq!(flatten_launch_arguments(&values), vec!["--always", "--linux", "yes"]);
    }

    #[test]
    fn converts_launcher_module_pairs_to_jni_equals_options() {
        let raw = vec![
            format!("-DlibraryDirectory={}", placeholder("library_directory")),
            "-p".to_string(),
            format!("{}/a.jar{}{}{}", placeholder("library_directory"), placeholder("classpath_separator"), placeholder("library_directory"), "/b.jar"),
            "--add-modules".to_string(),
            "ALL-MODULE-PATH".to_string(),
            "--add-opens".to_string(),
            "java.base/java.util.jar=cpw.mods.securejarhandler".to_string(),
        ];
        let replacements = vec![
            (placeholder("library_directory"), "/game/libraries".to_string()),
            (placeholder("classpath_separator"), ":".to_string()),
        ];
        let (options, module_paths, unsupported) = normalize_jvm_arguments(&raw, &replacements);
        assert!(unsupported.is_empty());
        assert_eq!(module_paths, vec!["/game/libraries/a.jar:/game/libraries/b.jar"]);
        assert!(options.contains(&"-DlibraryDirectory=/game/libraries".to_string()));
        assert!(options.contains(&"--module-path=/game/libraries/a.jar:/game/libraries/b.jar".to_string()));
        assert!(options.contains(&"--add-modules=ALL-MODULE-PATH".to_string()));
        assert!(options.contains(&"--add-opens=java.base/java.util.jar=cpw.mods.securejarhandler".to_string()));
    }

    #[test]
    fn detects_unresolved_launch_placeholders() {
        let value = placeholder("auth_player_name");
        let values = vec!["--username".to_string(), value.clone()];
        assert_eq!(collect_unresolved_placeholders(values.iter().map(String::as_str)), vec![value]);
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
