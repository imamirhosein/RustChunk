use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

const TARGET_VERSION: &str = "1.20.1";
const VERSION_MANIFEST_URL: &str =
    "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";

#[derive(Debug, Deserialize)]
struct VersionManifest {
    versions: Vec<ManifestVersion>,
}

#[derive(Debug, Deserialize)]
struct ManifestVersion {
    id: String,
    #[serde(rename = "type")]
    kind: String,
    url: String,
    #[serde(default)]
    sha1: Option<String>,
}

#[derive(Debug, Deserialize)]
struct VersionMetadata {
    id: String,
    downloads: VersionDownloads,
    #[serde(rename = "assetIndex")]
    asset_index: AssetIndexInfo,
    #[serde(default)]
    libraries: Vec<Library>,
}

#[derive(Debug, Deserialize)]
struct VersionDownloads {
    client: DownloadInfo,
}

#[derive(Debug, Deserialize)]
struct AssetIndexInfo {
    id: String,
    url: String,
    sha1: String,
}

#[derive(Debug, Deserialize)]
struct Library {
    downloads: Option<LibraryDownloads>,
    rules: Option<Vec<LibraryRule>>,
}

#[derive(Debug, Deserialize)]
struct LibraryDownloads {
    artifact: Option<DownloadInfo>,
}

#[derive(Debug, Deserialize)]
struct LibraryRule {
    action: String,
    os: Option<RuleOs>,
    features: Option<BTreeMap<String, bool>>,
}

#[derive(Debug, Deserialize)]
struct RuleOs {
    name: Option<String>,
    arch: Option<String>,
    version: Option<String>,
}

#[derive(Debug, Deserialize)]
struct DownloadInfo {
    path: Option<String>,
    url: String,
    sha1: String,
    size: u64,
}

#[derive(Debug, Deserialize)]
struct AssetIndex {
    objects: BTreeMap<String, AssetObject>,
}

#[derive(Debug, Deserialize)]
struct AssetObject {
    hash: String,
    size: u64,
}

#[derive(Debug, Serialize)]
struct DownloadEntry {
    kind: String,
    url: String,
    path: String,
    sha1: String,
    size: u64,
}

#[derive(Debug, Serialize)]
struct InstallPlan {
    minecraft_version: String,
    instance: String,
    files: Vec<DownloadEntry>,
    library_count: usize,
    asset_count: usize,
    total_bytes: u64,
}

pub fn resolve_minecraft(app_data_dir: &Path) -> Result<String, String> {
    if !app_data_dir.is_absolute() {
        return Err("App data path must be absolute".to_string());
    }

    let game_root = app_data_dir.join("minecraft");
    let metadata_dir = game_root.join("metadata");
    let asset_indexes_dir = game_root.join("assets").join("indexes");
    let instance_dir = game_root.join("instances").join("forge-1.20.1");

    fs::create_dir_all(&metadata_dir)
        .map_err(|e| format!("Cannot create metadata directory: {}", e))?;
    fs::create_dir_all(&asset_indexes_dir)
        .map_err(|e| format!("Cannot create asset index directory: {}", e))?;
    fs::create_dir_all(&instance_dir)
        .map_err(|e| format!("Cannot create instance directory: {}", e))?;

    let (manifest_text, manifest): (String, VersionManifest) =
        fetch_json(VERSION_MANIFEST_URL, "Mojang version manifest")?;
    let selected = manifest
        .versions
        .iter()
        .find(|entry| entry.id == TARGET_VERSION && entry.kind == "release")
        .ok_or_else(|| format!("Release {} was not found in Mojang's manifest", TARGET_VERSION))?;

    let (version_text, version): (String, VersionMetadata) =
        fetch_json(&selected.url, "Minecraft version metadata")?;
    if version.id != TARGET_VERSION {
        return Err(format!(
            "Mojang metadata mismatch: expected {}, received {}",
            TARGET_VERSION, version.id
        ));
    }
    verify_sha1_optional(
        version_text.as_bytes(),
        selected.sha1.as_deref(),
        "Minecraft version metadata",
    )?;

    let (asset_text, asset_index): (String, AssetIndex) =
        fetch_json(&version.asset_index.url, "Minecraft asset index")?;
    verify_sha1_optional(
        asset_text.as_bytes(),
        Some(&version.asset_index.sha1),
        "Minecraft asset index",
    )?;

    if !is_safe_component(&version.asset_index.id) {
        return Err("Mojang returned an unsafe asset index identifier".to_string());
    }

    let plan = build_install_plan(&version, &asset_index)?;
    let plan_path = instance_dir.join("install-plan.json");

    write_atomic(
        &metadata_dir.join("version_manifest_v2.json"),
        manifest_text.as_bytes(),
    )?;
    write_atomic(
        &metadata_dir.join(format!("{}.json", TARGET_VERSION)),
        version_text.as_bytes(),
    )?;
    write_atomic(
        &asset_indexes_dir.join(format!("{}.json", version.asset_index.id)),
        asset_text.as_bytes(),
    )?;

    let plan_json = serde_json::to_vec_pretty(&plan)
        .map_err(|e| format!("Could not encode install plan: {}", e))?;
    write_atomic(&plan_path, &plan_json)?;

    let library_bytes: u64 = plan
        .files
        .iter()
        .filter(|file| file.kind == "library")
        .map(|file| file.size)
        .sum();
    let asset_bytes: u64 = plan
        .files
        .iter()
        .filter(|file| file.kind == "asset")
        .map(|file| file.size)
        .sum();
    let client_bytes: u64 = plan
        .files
        .iter()
        .filter(|file| file.kind == "client")
        .map(|file| file.size)
        .sum();

    Ok(format!(
        "Minecraft 1.20.1 metadata resolved successfully\n         Release: {}\n         Libraries in plan: {} ({})\n         Assets in plan: {} ({})\n         Client file: {}\n         Total files to download: {}\n         Estimated download size: {}\n         Plan saved at: {}\n\n         Metadata and download plan are ready. Game files have NOT been downloaded yet.",
        TARGET_VERSION,
        plan.library_count,
        format_bytes(library_bytes),
        plan.asset_count,
        format_bytes(asset_bytes),
        format_bytes(client_bytes),
        plan.files.len(),
        format_bytes(plan.total_bytes),
        plan_path.display()
    ))
}

fn fetch_json<T: for<'de> Deserialize<'de>>(
    url: &str,
    description: &str,
) -> Result<(String, T), String> {
    if !url.starts_with("https://") {
        return Err(format!("Refusing non-HTTPS URL for {}", description));
    }

    let response = ureq::get(url)
        .set("User-Agent", "RustChunk/0.2 (Android launcher metadata resolver)")
        .call()
        .map_err(|e| format!("Could not fetch {}: {}", description, e))?;

    let body = response
        .into_string()
        .map_err(|e| format!("Could not read {} response: {}", description, e))?;
    let parsed = serde_json::from_str::<T>(&body)
        .map_err(|e| format!("Invalid {} JSON: {}", description, e))?;

    Ok((body, parsed))
}

fn verify_sha1_optional(
    bytes: &[u8],
    expected: Option<&str>,
    description: &str,
) -> Result<(), String> {
    let Some(expected) = expected else {
        return Ok(());
    };
    if !is_sha1(expected) {
        return Err(format!("Invalid SHA-1 value in {}", description));
    }

    let digest = Sha1::digest(bytes);
    let actual: String = digest.iter().map(|byte| format!("{:02x}", byte)).collect();
    if !actual.eq_ignore_ascii_case(expected) {
        return Err(format!("SHA-1 verification failed for {}", description));
    }
    Ok(())
}

fn verify_sha1(bytes: &[u8], expected: &str, description: &str) -> Result<(), String> {
    verify_sha1_optional(bytes, Some(expected), description)
}

fn build_install_plan(
    version: &VersionMetadata,
    asset_index: &AssetIndex,
) -> Result<InstallPlan, String> {
    if version.id != TARGET_VERSION {
        return Err(format!("Unexpected Minecraft version: {}", version.id));
    }

    let mut files = Vec::new();
    push_download(
        &mut files,
        "client",
        &version.downloads.client,
        &format!("versions/{0}/{0}.jar", TARGET_VERSION),
    )?;

    let mut library_count = 0usize;
    for library in &version.libraries {
        if !rules_allow(library.rules.as_deref()) {
            continue;
        }
        let Some(artifact) = library
            .downloads
            .as_ref()
            .and_then(|downloads| downloads.artifact.as_ref())
        else {
            continue;
        };
        let path = artifact
            .path
            .as_deref()
            .ok_or_else(|| "Library artifact is missing its destination path".to_string())?;
        push_download(&mut files, "library", artifact, path)?;
        library_count += 1;
    }

    let mut asset_count = 0usize;
    for asset in asset_index.objects.values() {
        if !is_sha1(&asset.hash) {
            return Err("Asset index contains an invalid SHA-1 hash".to_string());
        }
        let prefix = &asset.hash[..2];
        let path = format!("assets/objects/{}/{}", prefix, asset.hash);
        let url = format!(
            "https://resources.download.minecraft.net/{}/{}",
            prefix, asset.hash
        );
        files.push(DownloadEntry {
            kind: "asset".to_string(),
            url,
            path,
            sha1: asset.hash.clone(),
            size: asset.size,
        });
        asset_count += 1;
    }

    files.sort_by(|left, right| left.path.cmp(&right.path));
    let total_bytes = files.iter().try_fold(0u64, |total, file| {
        total
            .checked_add(file.size)
            .ok_or_else(|| "Total download size overflow".to_string())
    })?;

    Ok(InstallPlan {
        minecraft_version: TARGET_VERSION.to_string(),
        instance: "forge-1.20.1".to_string(),
        files,
        library_count,
        asset_count,
        total_bytes,
    })
}

fn push_download(
    files: &mut Vec<DownloadEntry>,
    kind: &str,
    info: &DownloadInfo,
    path: &str,
) -> Result<(), String> {
    let safe_path = validate_relative_path(path)?;
    if !info.url.starts_with("https://") {
        return Err(format!("Refusing non-HTTPS download URL: {}", info.url));
    }
    if !is_sha1(&info.sha1) {
        return Err(format!("Invalid SHA-1 for download destination {}", path));
    }

    files.push(DownloadEntry {
        kind: kind.to_string(),
        url: info.url.clone(),
        path: safe_path,
        sha1: info.sha1.clone(),
        size: info.size,
    });
    Ok(())
}

fn rules_allow(rules: Option<&[LibraryRule]>) -> bool {
    let Some(rules) = rules else {
        return true;
    };

    let mut allowed = false;
    for rule in rules {
        if rule_matches(rule) {
            allowed = rule.action == "allow";
        }
    }
    allowed
}

fn rule_matches(rule: &LibraryRule) -> bool {
    if let Some(os) = &rule.os {
        if os.name.as_deref().is_some_and(|name| name != "linux") {
            return false;
        }
        if os.arch.as_deref().is_some_and(|arch| {
            arch != "aarch64" && arch != "arm64"
        }) {
            return false;
        }
        // Regex-based OS version rules are not evaluated in this Android ARM64 stage.
        if os.version.is_some() {
            return false;
        }
    }

    if let Some(features) = &rule.features {
        if features.values().any(|required| *required) {
            return false;
        }
    }
    true
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

fn is_safe_component(value: &str) -> bool {
    !value.is_empty()
        && value
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || "._-".contains(character))
}

fn is_sha1(value: &str) -> bool {
    value.len() == 40 && value.chars().all(|character| character.is_ascii_hexdigit())
}

fn write_atomic(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let parent = path
        .parent()
        .ok_or_else(|| format!("Invalid destination path: {}", path.display()))?;
    fs::create_dir_all(parent)
        .map_err(|e| format!("Cannot create {}: {}", parent.display(), e))?;

    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| format!("Invalid destination filename: {}", path.display()))?;
    let temporary = parent.join(format!(".{}.tmp", file_name));

    fs::write(&temporary, bytes)
        .map_err(|e| format!("Cannot write {}: {}", temporary.display(), e))?;
    fs::rename(&temporary, path)
        .map_err(|e| format!("Cannot finalize {}: {}", path.display(), e))?;
    Ok(())
}

fn format_bytes(bytes: u64) -> String {
    const MIB: u64 = 1024 * 1024;
    format!("{:.1} MiB", bytes as f64 / MIB as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    const HASH_A: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    const HASH_B: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
    const HASH_C: &str = "cccccccccccccccccccccccccccccccccccccccc";

    fn fixture_metadata() -> VersionMetadata {
        serde_json::from_str(&format!(
            r#"{{
                "id":"1.20.1",
                "downloads":{{"client":{{
                    "url":"https://example.invalid/client.jar",
                    "sha1":"{HASH_A}",
                    "size":100
                }}}},
                "assetIndex":{{
                    "id":"1.20",
                    "url":"https://example.invalid/assets.json",
                    "sha1":"{HASH_B}"
                }},
                "libraries":[
                    {{
                        "downloads":{{"artifact":{{
                            "path":"org/example/library/1.0/library-1.0.jar",
                            "url":"https://example.invalid/library.jar",
                            "sha1":"{HASH_C}",
                            "size":20
                        }}}}
                    }},
                    {{
                        "rules":[{{"action":"allow","os":{{"name":"windows"}}}}],
                        "downloads":{{"artifact":{{
                            "path":"org/example/windows/1.0/windows-1.0.jar",
                            "url":"https://example.invalid/windows.jar",
                            "sha1":"{HASH_C}",
                            "size":30
                        }}}}
                    }}
                ]
            }}"#
        ))
        .expect("fixture metadata should parse")
    }

    #[test]
    fn creates_client_library_and_asset_download_entries() {
        let metadata = fixture_metadata();
        let assets: AssetIndex = serde_json::from_str(&format!(
            r#"{{"objects":{{"minecraft/lang/en_us.json":{{"hash":"{HASH_B}","size":10}}}}}}"#
        ))
        .expect("asset index should parse");

        let plan = build_install_plan(&metadata, &assets).expect("plan should build");

        assert_eq!(plan.minecraft_version, "1.20.1");
        assert_eq!(plan.library_count, 1);
        assert_eq!(plan.asset_count, 1);
        assert_eq!(plan.files.len(), 3);
        assert_eq!(plan.total_bytes, 130);
        assert!(plan.files.iter().any(|file| {
            file.path == "versions/1.20.1/1.20.1.jar" && file.kind == "client"
        }));
        assert!(plan.files.iter().any(|file| {
            file.path == format!("assets/objects/bb/{}", HASH_B) && file.kind == "asset"
        }));
    }

    #[test]
    fn rejects_path_traversal_in_library_artifact() {
        let mut metadata = fixture_metadata();
        metadata.libraries[0]
            .downloads
            .as_mut()
            .unwrap()
            .artifact
            .as_mut()
            .unwrap()
            .path = Some("../../outside.jar".to_string());

        let assets: AssetIndex =
            serde_json::from_str(r#"{"objects":{}}"#).expect("empty index should parse");
        assert!(build_install_plan(&metadata, &assets).is_err());
    }

    #[test]
    fn applies_minecraft_library_os_rules_for_android_arm64() {
        let linux_arm: LibraryRule = serde_json::from_str(
            r#"{"action":"allow","os":{"name":"linux","arch":"aarch64"}}"#,
        )
        .unwrap();
        let linux_x64: LibraryRule = serde_json::from_str(
            r#"{"action":"allow","os":{"name":"linux","arch":"x86_64"}}"#,
        )
        .unwrap();
        assert!(rule_matches(&linux_arm));
        assert!(!rule_matches(&linux_x64));
    }

    #[test]
    fn validates_sha1_and_relative_paths() {
        assert!(is_sha1(HASH_A));
        assert!(!is_sha1("not-a-hash"));
        assert!(validate_relative_path("assets/objects/aa/file").is_ok());
        assert!(validate_relative_path("../outside").is_err());
        assert!(validate_relative_path("assets\\outside").is_err());
    }

    #[test]
    fn verifies_sha1_digest() {
        let content = b"RustChunk";
        let digest = Sha1::digest(content);
        let expected: String = digest.iter().map(|byte| format!("{:02x}", byte)).collect();
        assert!(verify_sha1(content, &expected, "test data").is_ok());
        assert!(verify_sha1(content, HASH_A, "test data").is_err());
    }

    #[test]
    fn rejects_non_release_or_wrong_version() {
        let metadata: VersionMetadata = serde_json::from_str(
            r#"{
                "id":"1.20.2",
                "downloads":{"client":{"url":"https://example.invalid/client.jar","sha1":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","size":1}},
                "assetIndex":{"id":"1.20","url":"https://example.invalid/assets.json","sha1":"bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"},
                "libraries":[]
            }"#,
        )
        .unwrap();
        let assets: AssetIndex =
            serde_json::from_str(r#"{"objects":{}}"#).expect("empty index should parse");
        assert!(build_install_plan(&metadata, &assets).is_err());
    }

    #[test]
    fn writes_metadata_atomically_to_nested_directory() {
        let root: PathBuf = std::env::temp_dir().join(format!(
            "rustchunk-installer-test-{}",
            std::process::id()
        ));
        let target = root.join("metadata").join("version.json");
        write_atomic(&target, b"{}").expect("atomic write should succeed");
        assert_eq!(fs::read(&target).unwrap(), b"{}");
        fs::remove_dir_all(root).expect("test directory should be removable");
    }
}
