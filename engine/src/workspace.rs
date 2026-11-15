use std::fs;
use std::path::{Path, PathBuf};

const SHARED_DIRECTORIES: &[&str] = &[
    "",
    "assets",
    "libraries",
    "versions",
    "runtime",
    "profiles",
    "instances",
    "logs",
];

const INSTANCE_DIRECTORIES: &[&str] = &[
    "mods",
    "config",
    "defaultconfigs",
    "resourcepacks",
    "shaderpacks",
    "saves",
    "screenshots",
    "logs",
    "crash-reports",
];

const DEFAULT_PROFILE: &str = "# RustChunk instance profile
profile_id=forge-1.20.1
minecraft_version=1.20.1
mod_loader=forge
mod_loader_version=UNSET
java_major=17
";

/// Creates the on-device Minecraft workspace without overwriting user data.
/// `app_data_dir` must be the app-private files directory supplied by Android.
pub fn prepare_workspace(app_data_dir: &Path) -> Result<String, String> {
    if !app_data_dir.is_absolute() {
        return Err("Android app data path must be absolute".to_string());
    }

    let game_root = app_data_dir.join("minecraft");
    let instance_root = game_root.join("instances").join("forge-1.20.1");
    let mut ready_count = 0usize;

    for relative in SHARED_DIRECTORIES {
        let directory = if relative.is_empty() {
            game_root.clone()
        } else {
            game_root.join(relative)
        };
        fs::create_dir_all(&directory).map_err(|error| {
            format!("Could not create {}: {}", directory.display(), error)
        })?;
        ready_count += 1;
    }

    for relative in INSTANCE_DIRECTORIES {
        let directory = instance_root.join(relative);
        fs::create_dir_all(&directory).map_err(|error| {
            format!("Could not create {}: {}", directory.display(), error)
        })?;
        ready_count += 1;
    }

    let profile_file = instance_root.join("profile.properties");
    if !profile_file.exists() {
        fs::write(&profile_file, DEFAULT_PROFILE).map_err(|error| {
            format!("Could not write {}: {}", profile_file.display(), error)
        })?;
    }

    Ok(format!(
        "Minecraft workspace ready\nGame folder: {}\nForge instance: {}\nDirectories ready: {}\nProfile file: {}",
        game_root.display(),
        instance_root.display(),
        ready_count,
        profile_file.display()
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temporary_directory() -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock must be after Unix epoch")
            .as_nanos();
        std::env::temp_dir().join(format!(
            "rustchunk-workspace-test-{}-{}",
            std::process::id(),
            nonce
        ))
    }

    #[test]
    fn creates_shared_and_forge_instance_directories() {
        let temp = temporary_directory();
        let result = prepare_workspace(&temp).expect("workspace should be created");
        let game_root = temp.join("minecraft");
        let instance = game_root.join("instances/forge-1.20.1");

        assert!(game_root.join("assets").is_dir());
        assert!(game_root.join("libraries").is_dir());
        assert!(game_root.join("versions").is_dir());
        assert!(instance.join("mods").is_dir());
        assert!(instance.join("config").is_dir());
        assert!(instance.join("saves").is_dir());
        assert!(instance.join("logs").is_dir());
        assert!(result.contains("Minecraft workspace ready"));

        fs::remove_dir_all(temp).expect("temporary workspace should be removable");
    }

    #[test]
    fn preserves_existing_profile_configuration() {
        let temp = temporary_directory();
        prepare_workspace(&temp).expect("workspace should be created");

        let profile = temp
            .join("minecraft/instances/forge-1.20.1/profile.properties");
        fs::write(&profile, "custom_setting=preserve-me\n")
            .expect("custom profile should be writable");

        prepare_workspace(&temp).expect("workspace should be reusable");
        let contents = fs::read_to_string(&profile).expect("profile should be readable");
        assert_eq!(contents, "custom_setting=preserve-me\n");

        fs::remove_dir_all(temp).expect("temporary workspace should be removable");
    }

    #[test]
    fn rejects_relative_app_data_path() {
        let result = prepare_workspace(Path::new("relative/path"));
        assert!(result.is_err());
    }
}
