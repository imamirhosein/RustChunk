use serde::Deserialize;
use sha1::{Digest, Sha1};
use std::collections::BTreeMap;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::{Mutex, OnceLock};
use std::thread;
use std::time::Duration;

const TARGET_VERSION: &str = "1.20.1";
const MAX_RETRIES: usize = 3;
const BUFFER_SIZE: usize = 64 * 1024;

static CANCEL_REQUESTED: AtomicBool = AtomicBool::new(false);
static PROGRESS: OnceLock<Mutex<DownloadProgress>> = OnceLock::new();

#[derive(Debug, Default)]
struct DownloadProgress {
    active: bool,
    bytes_done: u64,
    total_bytes: u64,
    files_done: usize,
    total_files: usize,
    current_file: String,
    message: String,
}

#[derive(Debug, Deserialize, Clone)]
struct InstallPlan {
    minecraft_version: String,
    instance: String,
    files: Vec<PlanFile>,
}

#[derive(Debug, Deserialize, Clone)]
struct PlanFile {
    kind: String,
    url: String,
    path: String,
    sha1: String,
    size: u64,
}

struct DownloadTask {
    entry: PlanFile,
    target: PathBuf,
    part: PathBuf,
}

fn progress_lock() -> &'static Mutex<DownloadProgress> {
    PROGRESS.get_or_init(|| Mutex::new(DownloadProgress::default()))
}

fn with_progress<F, T>(operation: F) -> T
where
    F: FnOnce(&mut DownloadProgress) -> T,
{
    let mut guard = progress_lock()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    operation(&mut guard)
}

pub fn download_progress() -> String {
    with_progress(|state| {
        let percent = if state.total_bytes == 0 {
            if state.active { 0.0 } else { 100.0 }
        } else {
            ((state.bytes_done as f64 / state.total_bytes as f64) * 100.0).clamp(0.0, 100.0)
        };
        format!(
            "{}\nProgress: {:.1}%\nFiles: {}/{}\nData: {} / {}\nCurrent: {}",
            state.message,
            percent,
            state.files_done,
            state.total_files,
            format_bytes(state.bytes_done),
            format_bytes(state.total_bytes),
            if state.current_file.is_empty() {
                "-"
            } else {
                state.current_file.as_str()
            }
        )
    })
}

pub fn cancel_download() -> String {
    let was_active = with_progress(|state| {
        if state.active {
            CANCEL_REQUESTED.store(true, Ordering::SeqCst);
            state.message = "Cancellation requested...".to_string();
            true
        } else {
            false
        }
    });

    if was_active {
        "Download cancellation requested. Active network reads will stop shortly.".to_string()
    } else {
        "No download is currently running.".to_string()
    }
}

pub fn download_minecraft(app_data_dir: &Path, parallelism: usize) -> Result<String, String> {
    if !app_data_dir.is_absolute() {
        return Err("App data path must be absolute".to_string());
    }
    if !(1..=8).contains(&parallelism) {
        return Err("Parallel download count must be between 1 and 8".to_string());
    }

    let already_active = with_progress(|state| {
        if state.active {
            true
        } else {
            state.active = true;
            state.bytes_done = 0;
            state.total_bytes = 0;
            state.files_done = 0;
            state.total_files = 0;
            state.current_file.clear();
            state.message = "Preparing Minecraft downloads...".to_string();
            false
        }
    });
    if already_active {
        return Err("A Minecraft download is already running".to_string());
    }

    CANCEL_REQUESTED.store(false, Ordering::SeqCst);
    let result = run_download(app_data_dir, parallelism);

    with_progress(|state| {
        state.active = false;
        state.current_file.clear();
        state.message = match &result {
            Ok(message) => message.clone(),
            Err(error) => format!("Download failed: {}", error),
        };
    });

    result
}

fn run_download(app_data_dir: &Path, parallelism: usize) -> Result<String, String> {
    let game_root = app_data_dir.join("minecraft");
    let plan_path = game_root
        .join("instances")
        .join("forge-1.20.1")
        .join("install-plan.json");

    let plan_bytes = fs::read(&plan_path)
        .map_err(|error| format!("Could not read install plan {}: {}", plan_path.display(), error))?;
    let plan: InstallPlan = serde_json::from_slice(&plan_bytes)
        .map_err(|error| format!("Install plan JSON is invalid: {}", error))?;

    if plan.minecraft_version != TARGET_VERSION || plan.instance != "forge-1.20.1" {
        return Err("Install plan does not target Minecraft 1.20.1 / forge-1.20.1".to_string());
    }

    // Asset indexes can contain multiple logical names with the same SHA-1.
    // Those names map to one content-addressed object path, so download it only once.
    let mut unique = BTreeMap::<String, PlanFile>::new();
    for entry in plan.files {
        let safe_path = validate_relative_path(&entry.path)?;
        if !entry.url.starts_with("https://") {
            return Err(format!("Refusing non-HTTPS URL for {}", safe_path));
        }
        if !is_sha1(&entry.sha1) {
            return Err(format!("Invalid SHA-1 for {}", safe_path));
        }
        let mut normalized = entry.clone();
        normalized.path = safe_path.clone();

        if let Some(existing) = unique.get(&safe_path) {
            if existing.sha1 != normalized.sha1 || existing.size != normalized.size {
                return Err(format!("Conflicting plan entries for {}", safe_path));
            }
        } else {
            unique.insert(safe_path, normalized);
        }
    }

    let entries: Vec<PlanFile> = unique.into_values().collect();
    let total_bytes = entries.iter().try_fold(0u64, |total, entry| {
        total
            .checked_add(entry.size)
            .ok_or_else(|| "Total download size overflow".to_string())
    })?;

    let mut tasks = Vec::new();
    let mut already_done_bytes = 0u64;
    let mut already_done_files = 0usize;

    for entry in entries {
        if CANCEL_REQUESTED.load(Ordering::SeqCst) {
            return Ok("Download cancelled before file transfer began.".to_string());
        }

        let target = game_root.join(&entry.path);
        let part = part_path(&target);
        let parent = target
            .parent()
            .ok_or_else(|| format!("Invalid target path: {}", target.display()))?;
        fs::create_dir_all(parent)
            .map_err(|error| format!("Cannot create {}: {}", parent.display(), error))?;

        if target.is_file() {
            let existing_size = fs::metadata(&target)
                .map_err(|error| format!("Cannot inspect {}: {}", target.display(), error))?
                .len();
            if existing_size == entry.size && sha1_matches(&target, &entry.sha1)? {
                already_done_bytes = already_done_bytes.saturating_add(entry.size);
                already_done_files += 1;
                continue;
            }
            fs::remove_file(&target)
                .map_err(|error| format!("Cannot remove invalid file {}: {}", target.display(), error))?;
        }

        let partial_size = if part.is_file() {
            fs::metadata(&part)
                .map_err(|error| format!("Cannot inspect partial file {}: {}", part.display(), error))?
                .len()
        } else {
            0
        };

        if partial_size > entry.size {
            fs::remove_file(&part)
                .map_err(|error| format!("Cannot remove oversized partial file {}: {}", part.display(), error))?;
        } else if partial_size == entry.size && part.is_file() {
            if sha1_matches(&part, &entry.sha1)? {
                fs::rename(&part, &target)
                    .map_err(|error| format!("Cannot finalize {}: {}", target.display(), error))?;
                already_done_bytes = already_done_bytes.saturating_add(entry.size);
                already_done_files += 1;
                continue;
            }
            fs::remove_file(&part)
                .map_err(|error| format!("Cannot remove invalid completed partial {}: {}", part.display(), error))?;
        } else {
            already_done_bytes = already_done_bytes.saturating_add(partial_size);
        }

        tasks.push(DownloadTask {
            entry,
            target,
            part,
        });
    }

    with_progress(|state| {
        state.bytes_done = already_done_bytes.min(total_bytes);
        state.total_bytes = total_bytes;
        state.files_done = already_done_files;
        state.total_files = already_done_files + tasks.len();
        state.message = format!(
            "Downloading Minecraft 1.20.1 files with {} parallel workers...",
            parallelism
        );
    });

    if tasks.is_empty() {
        return Ok(format!(
            "Minecraft 1.20.1 files are already complete.\nVerified files: {}\nTotal data: {}",
            already_done_files,
            format_bytes(total_bytes)
        ));
    }

    let queue = AtomicUsize::new(0);
    let first_error = Mutex::new(None::<String>);
    let worker_count = parallelism.min(tasks.len());

    thread::scope(|scope| {
        for _ in 0..worker_count {
            scope.spawn(|| {
                let agent = ureq::AgentBuilder::new()
                    .timeout_connect(Duration::from_secs(20))
                    .timeout_read(Duration::from_secs(30))
                    .timeout_write(Duration::from_secs(30))
                    .build();

                loop {
                    if CANCEL_REQUESTED.load(Ordering::SeqCst) {
                        break;
                    }
                    let index = queue.fetch_add(1, Ordering::SeqCst);
                    if index >= tasks.len() {
                        break;
                    }

                    let task = &tasks[index];
                    with_progress(|state| {
                        state.current_file = task.entry.path.clone();
                    });

                    if let Err(error) = download_one(task, &agent) {
                        if !CANCEL_REQUESTED.load(Ordering::SeqCst) {
                            let mut first = first_error
                                .lock()
                                .unwrap_or_else(|poisoned| poisoned.into_inner());
                            if first.is_none() {
                                *first = Some(format!("{}: {}", task.entry.path, error));
                            }
                            CANCEL_REQUESTED.store(true, Ordering::SeqCst);
                        }
                        break;
                    }
                }
            });
        }
    });

    let error = first_error
        .into_inner()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    if let Some(error) = error {
        return Err(error);
    }

    let (files_done, bytes_done) = with_progress(|state| (state.files_done, state.bytes_done));
    if CANCEL_REQUESTED.load(Ordering::SeqCst) && files_done < with_progress(|state| state.total_files) {
        return Ok(format!(
            "Download cancelled.\nCompleted files: {}\nData received or resumed: {} / {}.\nPartial files were kept so the download can resume later.",
            files_done,
            format_bytes(bytes_done),
            format_bytes(total_bytes)
        ));
    }

    Ok(format!(
        "Minecraft 1.20.1 base files downloaded and SHA-1 verified successfully.\nVerified files: {}\nTotal data: {}\nLocation: {}\n\nForge, Java runtime, native extraction, and game launching are not installed in this stage.",
        files_done,
        format_bytes(total_bytes),
        game_root.display()
    ))
}

fn download_one(task: &DownloadTask, agent: &ureq::Agent) -> Result<(), String> {
    'attempt: for attempt in 1..=MAX_RETRIES {
        if CANCEL_REQUESTED.load(Ordering::SeqCst) {
            return Ok(());
        }

        let offset = if task.part.is_file() {
            fs::metadata(&task.part)
                .map_err(|error| format!("Cannot inspect partial file: {}", error))?
                .len()
        } else {
            0
        };

        if offset > task.entry.size {
            fs::remove_file(&task.part)
                .map_err(|error| format!("Cannot reset oversized partial file: {}", error))?;
            subtract_progress(offset);
        }

        let current_offset = if offset > task.entry.size { 0 } else { offset };
        let mut request = agent
            .get(&task.entry.url)
            .set("User-Agent", "RustChunk/0.3 Android ARM64 downloader")
            .set("Accept-Encoding", "identity");
        if current_offset > 0 {
            request = request.set("Range", &format!("bytes={}-", current_offset));
        }

        let response = match request.call() {
            Ok(response) => response,
            Err(ureq::Error::Status(code, _)) => {
                if attempt == MAX_RETRIES {
                    return Err(format!("HTTP status {}", code));
                }
                thread::sleep(Duration::from_millis(400 * attempt as u64));
                continue;
            }
            Err(error) => {
                if attempt == MAX_RETRIES {
                    return Err(format!("Network request failed: {}", error));
                }
                thread::sleep(Duration::from_millis(400 * attempt as u64));
                continue;
            }
        };

        let status = response.status();
        if status != 200 && status != 206 {
            return Err(format!("Unexpected HTTP status {}", status));
        }

        let append = status == 206 && current_offset > 0;
        if status == 206 {
            let expected_prefix = format!("bytes {}-", current_offset);
            let content_range = response.header("Content-Range").unwrap_or("");
            if !content_range.starts_with(&expected_prefix) {
                return Err(format!(
                    "Invalid Content-Range while resuming (expected {}, received {})",
                    expected_prefix, content_range
                ));
            }
        }

        if current_offset > 0 && status == 200 {
            subtract_progress(current_offset);
        }

        let mut options = OpenOptions::new();
        options.create(true).write(true);
        if append {
            options.append(true);
        } else {
            options.truncate(true);
        }
        let mut output = options
            .open(&task.part)
            .map_err(|error| format!("Cannot open partial file: {}", error))?;
        let mut input = response.into_reader();
        let mut buffer = [0u8; BUFFER_SIZE];

        loop {
            if CANCEL_REQUESTED.load(Ordering::SeqCst) {
                output.flush().ok();
                return Ok(());
            }
            let count = match input.read(&mut buffer) {
                Ok(0) => break,
                Ok(count) => count,
                Err(error) => {
                    output.flush().ok();
                    if attempt == MAX_RETRIES {
                        return Err(format!("Download stream failed: {}", error));
                    }
                    drop(output);
                    thread::sleep(Duration::from_millis(400 * attempt as u64));
                    continue 'attempt;
                }
            };
            output
                .write_all(&buffer[..count])
                .map_err(|error| format!("Cannot write partial file: {}", error))?;
            add_progress(count as u64);
        }

        output
            .flush()
            .map_err(|error| format!("Cannot flush partial file: {}", error))?;
        output
            .sync_all()
            .map_err(|error| format!("Cannot sync partial file: {}", error))?;
        drop(output);

        if CANCEL_REQUESTED.load(Ordering::SeqCst) {
            return Ok(());
        }

        let received_size = fs::metadata(&task.part)
            .map_err(|error| format!("Cannot inspect downloaded file: {}", error))?
            .len();
        if received_size != task.entry.size {
            if received_size > task.entry.size {
                fs::remove_file(&task.part)
                    .map_err(|error| format!("Cannot reset oversized download: {}", error))?;
                subtract_progress(received_size);
            }
            if attempt == MAX_RETRIES {
                return Err(format!(
                    "Size mismatch: expected {} bytes, received {}",
                    task.entry.size, received_size
                ));
            }
            thread::sleep(Duration::from_millis(400 * attempt as u64));
            continue;
        }

        if !sha1_matches(&task.part, &task.entry.sha1)? {
            fs::remove_file(&task.part)
                .map_err(|error| format!("Cannot remove SHA-1-invalid download: {}", error))?;
            subtract_progress(received_size);
            if attempt == MAX_RETRIES {
                return Err("SHA-1 verification failed after retries".to_string());
            }
            thread::sleep(Duration::from_millis(400 * attempt as u64));
            continue;
        }

        fs::rename(&task.part, &task.target)
            .map_err(|error| format!("Cannot finalize downloaded file: {}", error))?;
        with_progress(|state| {
            state.files_done = state.files_done.saturating_add(1);
        });
        return Ok(());
    }

    Err("Download retries exhausted".to_string())
}

fn add_progress(bytes: u64) {
    with_progress(|state| {
        state.bytes_done = state.bytes_done.saturating_add(bytes).min(state.total_bytes);
    });
}

fn subtract_progress(bytes: u64) {
    with_progress(|state| {
        state.bytes_done = state.bytes_done.saturating_sub(bytes);
    });
}

fn part_path(target: &Path) -> PathBuf {
    let mut name = target.as_os_str().to_os_string();
    name.push(".part");
    PathBuf::from(name)
}

fn sha1_matches(path: &Path, expected: &str) -> Result<bool, String> {
    let mut file = File::open(path)
        .map_err(|error| format!("Cannot read {} for SHA-1: {}", path.display(), error))?;
    let mut digest = Sha1::new();
    let mut buffer = [0u8; BUFFER_SIZE];
    loop {
        let count = file
            .read(&mut buffer)
            .map_err(|error| format!("Cannot hash {}: {}", path.display(), error))?;
        if count == 0 {
            break;
        }
        digest.update(&buffer[..count]);
    }
    let actual = format!("{:x}", digest.finalize());
    Ok(actual.eq_ignore_ascii_case(expected))
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

fn is_sha1(value: &str) -> bool {
    value.len() == 40 && value.chars().all(|character| character.is_ascii_hexdigit())
}

fn format_bytes(bytes: u64) -> String {
    const MIB: u64 = 1024 * 1024;
    format!("{:.1} MiB", bytes as f64 / MIB as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_unsafe_paths_and_non_https_urls() {
        assert!(validate_relative_path("assets/objects/aa/file").is_ok());
        assert!(validate_relative_path("../../private").is_err());
        assert!(validate_relative_path("assets\\private").is_err());
        assert!(!"http://example.com/file".starts_with("https://"));
    }

    #[test]
    fn builds_partial_path_without_replacing_extension() {
        let path = PathBuf::from("/tmp/example.jar");
        assert_eq!(part_path(&path), PathBuf::from("/tmp/example.jar.part"));
    }

    #[test]
    fn validates_sha1_shape() {
        assert!(is_sha1("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"));
        assert!(!is_sha1("abc"));
    }

    #[test]
    fn sha1_verification_reads_file_contents() {
        let path = std::env::temp_dir().join(format!(
            "rustchunk-downloader-test-{}.dat",
            std::process::id()
        ));
        fs::write(&path, b"RustChunk").unwrap();
        let digest = Sha1::digest(b"RustChunk");
        let expected = format!("{:x}", digest);
        assert!(sha1_matches(&path, &expected).unwrap());
        assert!(!sha1_matches(&path, "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap());
        fs::remove_file(path).unwrap();
    }
}
