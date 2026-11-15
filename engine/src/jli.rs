//! Android Java runtime smoke test using the OpenJDK JLI entry point.
//! This avoids directly executing the app-private bin/java ELF binary.

use std::collections::BTreeSet;
use std::ffi::{CStr, CString};
use std::fs;
use std::os::raw::{c_char, c_int, c_void};
use std::path::{Path, PathBuf};
use std::ptr;
use std::sync::{Mutex, OnceLock};

const RTLD_LAZY: c_int = 1;
const RTLD_NOW: c_int = 2;
const RTLD_GLOBAL: c_int = 0x100;
const JNI_FALSE: u8 = 0;
const JNI_TRUE: u8 = 1;
const MAX_SCAN_DIRECTORIES: usize = 25_000;
const MAX_SCAN_DEPTH: usize = 14;

#[link(name = "dl")]
unsafe extern "C" {
    fn dlopen(filename: *const c_char, flags: c_int) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    fn dlerror() -> *const c_char;
}

type JliLaunch = unsafe extern "C" fn(
    c_int, *mut *mut c_char, c_int, *const *const c_char,
    c_int, *const *const c_char, *const c_char, *const c_char,
    *const c_char, *const c_char, u8, u8, u8, c_int
) -> c_int;

type AndroidUpdateLdLibraryPath = unsafe extern "C" fn(*mut c_char);

#[derive(Debug, Clone)]
struct RuntimeCandidate {
    home: PathBuf,
    libjli: PathBuf,
    libjvm: PathBuf,
    java_executable: Option<PathBuf>,
    version: String,
    library_directories: Vec<PathBuf>,
}

static JLI_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

pub fn test_java_runtime(app_data_dir: &Path, app_native_library_dir: &Path) -> Result<String, String> {
    if !app_data_dir.is_absolute() || !app_native_library_dir.is_absolute() {
        return Err("RustChunk and Android native library paths must be absolute.".to_string());
    }
    let _guard = JLI_LOCK.get_or_init(|| Mutex::new(())).lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());

    let runtime = find_java17_runtime(app_data_dir)?;
    let home = runtime.home.canonicalize()
        .map_err(|error| format!("Cannot resolve Java home {}: {}", runtime.home.display(), error))?;
    let cache_dir = app_data_dir.join("cache").join("jli");
    fs::create_dir_all(&cache_dir)
        .map_err(|error| format!("Cannot create JLI temporary directory {}: {}", cache_dir.display(), error))?;

    let mut directories = BTreeSet::<PathBuf>::new();
    directories.insert(home.join("lib"));
    directories.insert(home.join("lib").join("server"));
    directories.insert(home.join("lib").join("jli"));
    directories.extend(runtime.library_directories.iter().cloned());
    directories.insert(app_native_library_dir.to_path_buf());

    let previous_ld = std::env::var("LD_LIBRARY_PATH").unwrap_or_default();
    let mut ld_parts: Vec<String> = directories.iter()
        .filter(|path| path.is_dir())
        .map(|path| path.to_string_lossy().into_owned())
        .collect();
    if !previous_ld.trim().is_empty() {
        ld_parts.extend(previous_ld.split(':').filter(|part| !part.is_empty()).map(str::to_string));
    }
    let ld_path = ld_parts.join(":");

    std::env::set_var("JAVA_HOME", &home);
    std::env::set_var("HOME", app_data_dir);
    std::env::set_var("TMPDIR", &cache_dir);
    std::env::set_var("LD_LIBRARY_PATH", &ld_path);
    let previous_path = std::env::var("PATH").unwrap_or_default();
    std::env::set_var("PATH", format!("{}:{}", home.join("bin").to_string_lossy(), previous_path));

    let mut report = String::new();
    report.push_str("RustChunk Android native Java runtime test (JLI)\n");
    report.push_str(&format!("Java home: {}\n", home.display()));
    report.push_str(&format!("Java release: {}\n", runtime.version));
    report.push_str(&format!("Java executable (not directly executed): {}\n",
        runtime.java_executable.as_ref().map(|p| p.display().to_string()).unwrap_or_else(|| "not found".to_string())));
    report.push_str(&format!("libjli.so: {}\n", runtime.libjli.display()));
    report.push_str(&format!("libjvm.so: {}\n", runtime.libjvm.display()));
    report.push_str(&format!("Android native library directory: {}\n", app_native_library_dir.display()));
    report.push_str(&format!("Runtime library directories: {}\n", directories.iter().filter(|p| p.is_dir()).count()));

    if let Some(update_path) = resolve_android_update_library_path() {
        let path_c = CString::new(ld_path.as_bytes())
            .map_err(|_| "LD_LIBRARY_PATH contains an embedded NUL byte.".to_string())?;
        unsafe { update_path(path_c.as_ptr() as *mut c_char); }
        report.push_str("Android linker search path updated through native loader bridge.\n");
    } else {
        report.push_str("NOTICE: Android loader path bridge unavailable; absolute library paths will be used.\n");
    }

    let jli_handle = open_library(&runtime.libjli)
        .map_err(|error| format!("Cannot load libjli.so: {}\n{}", error, report))?;
    let _jvm_handle = open_library(&runtime.libjvm)
        .map_err(|error| format!("Cannot load libjvm.so: {}\n{}", error, report))?;

    for name in ["libverify.so", "libjava.so", "libnet.so", "libnio.so"] {
        if let Some(path) = find_named_library(&home, name) {
            match open_library(&path) {
                Ok(_) => report.push_str(&format!("Preloaded {}\n", path.display())),
                Err(error) => report.push_str(&format!("NOTICE: Optional {} preload failed: {}\n", name, error)),
            }
        }
    }

    let symbol_name = CString::new("JLI_Launch").unwrap();
    unsafe { dlerror(); }
    let symbol = unsafe { dlsym(jli_handle, symbol_name.as_ptr()) };
    if symbol.is_null() {
        return Err(format!("JLI_Launch is not exported by {}: {}\n{}",
            runtime.libjli.display(), last_dl_error(), report));
    }
    let launch: JliLaunch = unsafe { std::mem::transmute(symbol) };

    let version_c = CString::new(runtime.version.as_str())
        .map_err(|_| "Java release version contains an embedded NUL byte.".to_string())?;
    let major_c = CString::new("17").unwrap();
    let program_c = CString::new("java").unwrap();
    let launcher_c = CString::new("openjdk").unwrap();
    // JLI_Launch accepts writable C argv strings. Keep each NUL-terminated
    // buffer alive and mutable for the entire native call.
    let mut args = vec![
        CString::new("java").unwrap().into_bytes_with_nul(),
        CString::new(format!("-Djava.home={}", home.to_string_lossy())).unwrap().into_bytes_with_nul(),
        CString::new(format!("-Djava.io.tmpdir={}", cache_dir.to_string_lossy())).unwrap().into_bytes_with_nul(),
        CString::new("-Djava.awt.headless=true").unwrap().into_bytes_with_nul(),
        CString::new("-version").unwrap().into_bytes_with_nul(),
    ];
    let argc = args.len() as c_int;
    let mut argv: Vec<*mut c_char> = args
        .iter_mut()
        .map(|arg| arg.as_mut_ptr().cast::<c_char>())
        .collect();
    argv.push(ptr::null_mut());

    report.push_str("Calling OpenJDK JLI_Launch: java -version\n");
    let exit_code = unsafe {
        launch(argc, argv.as_mut_ptr(), 0, ptr::null(), 0, ptr::null(),
            version_c.as_ptr(), major_c.as_ptr(), program_c.as_ptr(),
            launcher_c.as_ptr(), JNI_FALSE, JNI_TRUE, JNI_FALSE, 0)
    };
    report.push_str(&format!("JLI_Launch exit code: {}\n", exit_code));
    if exit_code == 0 {
        report.push_str("RESULT: JAVA 17 NATIVE JLI STARTUP OK\n");
    } else {
        report.push_str("RESULT: JAVA 17 NATIVE JLI STARTUP FAILED\n");
    }
    report.push_str("This validates Java VM startup only. Forge, Minecraft graphics, mods, and Aternos networking have not been tested.\n");
    Ok(report)
}

fn find_java17_runtime(app_data_dir: &Path) -> Result<RuntimeCandidate, String> {
    let roots = [
        app_data_dir.join("games/runtimes"),
        app_data_dir.join("runtime"),
        app_data_dir.join("runtimes"),
        app_data_dir.join("minecraft/runtime"),
        app_data_dir.join("minecraft/runtimes"),
    ];
    let mut files = Vec::new();
    let mut visited = 0usize;
    for root in &roots {
        if root.is_dir() { scan_runtime_tree(root, &mut files, &mut visited); }
    }

    let jli_paths: Vec<PathBuf> = files.iter()
        .filter(|p| p.file_name().and_then(|s| s.to_str()) == Some("libjli.so"))
        .cloned().collect();
    let jvm_paths: Vec<PathBuf> = files.iter()
        .filter(|p| p.file_name().and_then(|s| s.to_str()) == Some("libjvm.so"))
        .cloned().collect();
    let java_paths: Vec<PathBuf> = files.iter()
        .filter(|p| p.file_name().and_then(|s| s.to_str()) == Some("java")
            && p.parent().and_then(Path::file_name).and_then(|s| s.to_str()) == Some("bin"))
        .cloned().collect();

    let mut candidates = Vec::new();
    for libjli in jli_paths {
        let Some(home) = libjli.ancestors()
            .find(|ancestor| ancestor.join("release").is_file())
            .map(Path::to_path_buf) else { continue };
        let release = fs::read_to_string(home.join("release")).unwrap_or_default();
        let Some(version) = parse_java_version(&release) else { continue };
        if java_major(&version) != Some(17) { continue; }

        let libjvm = jvm_paths.iter().find(|p| p.starts_with(&home)).cloned();
        let Some(libjvm) = libjvm else { continue };
        if !is_aarch64_elf(&libjli) || !is_aarch64_elf(&libjvm) { continue; }
        let java_executable = java_paths.iter().find(|p| p.starts_with(&home)).cloned();
        let library_directories: BTreeSet<PathBuf> = files.iter()
            .filter(|p| p.starts_with(&home) && p.extension().and_then(|e| e.to_str()) == Some("so"))
            .filter_map(|p| p.parent().map(Path::to_path_buf))
            .collect();
        candidates.push(RuntimeCandidate {
            home, libjli, libjvm, java_executable, version,
            library_directories: library_directories.into_iter().collect(),
        });
    }

    candidates.sort_by(|a,b| {
        let ap = a.home.ends_with("Internal-17");
        let bp = b.home.ends_with("Internal-17");
        bp.cmp(&ap).then_with(|| a.home.cmp(&b.home))
    });
    candidates.into_iter().next().ok_or_else(|| {
        format!("No ARM64 Java 17 runtime found. Required: release with JAVA_VERSION 17, libjli.so, and libjvm.so. Scanned {} files; roots: {}",
            files.len(), roots.iter().map(|p| format!("{} ({})", p.display(), if p.is_dir(){"present"}else{"missing"})).collect::<Vec<_>>().join("; "))
    })
}

fn scan_runtime_tree(root: &Path, files: &mut Vec<PathBuf>, visited_total: &mut usize) {
    let mut pending = vec![(root.to_path_buf(), 0usize)];
    let mut seen = BTreeSet::<PathBuf>::new();
    while let Some((directory, depth)) = pending.pop() {
        if *visited_total >= MAX_SCAN_DIRECTORIES || depth > MAX_SCAN_DEPTH { break; }
        let Ok(canonical) = directory.canonicalize() else { continue };
        if !seen.insert(canonical) { continue; }
        *visited_total += 1;
        let Ok(entries) = fs::read_dir(&directory) else { continue };
        for entry in entries.flatten() {
            let path = entry.path();
            let Ok(kind) = entry.file_type() else { continue };
            if kind.is_dir() {
                pending.push((path, depth + 1));
            } else if (kind.is_file() || kind.is_symlink()) && path.is_file() {
                let name = path.file_name().and_then(|v| v.to_str()).unwrap_or("");
                if path.extension().and_then(|v| v.to_str()) == Some("so") || name == "release" || name == "java" {
                    files.push(path);
                }
            }
        }
    }
}

fn parse_java_version(release: &str) -> Option<String> {
    release.lines().find_map(|line| {
        let (key, value) = line.split_once('=')?;
        if key.trim() != "JAVA_VERSION" { return None; }
        let value = value.trim().trim_matches('"').trim();
        if value.is_empty() { None } else { Some(value.to_string()) }
    })
}

fn java_major(version: &str) -> Option<u32> {
    let first = version.split('.').next()?;
    if first == "1" { version.split('.').nth(1)?.parse().ok() }
    else { first.split(['-', '+']).next()?.parse().ok() }
}

fn is_aarch64_elf(path: &Path) -> bool {
    let Ok(bytes) = fs::read(path) else { return false };
    bytes.len() >= 20 && bytes[0..4] == [0x7f, b'E', b'L', b'F']
        && bytes[4] == 2 && bytes[5] == 1
        && u16::from_le_bytes([bytes[18], bytes[19]]) == 183
}

fn resolve_android_update_library_path() -> Option<AndroidUpdateLdLibraryPath> {
    let library = CString::new("libdl.so").unwrap();
    let handle = unsafe { dlopen(library.as_ptr(), RTLD_LAZY | RTLD_GLOBAL) };
    if handle.is_null() { return None; }
    for name in ["android_update_LD_LIBRARY_PATH", "__loader_android_update_LD_LIBRARY_PATH"] {
        let name = CString::new(name).unwrap();
        let symbol = unsafe { dlsym(handle, name.as_ptr()) };
        if !symbol.is_null() { return Some(unsafe { std::mem::transmute(symbol) }); }
    }
    None
}

fn open_library(path: &Path) -> Result<*mut c_void, String> {
    let name = CString::new(path.to_string_lossy().as_bytes())
        .map_err(|_| format!("Library path contains NUL: {}", path.display()))?;
    unsafe { dlerror(); }
    let handle = unsafe { dlopen(name.as_ptr(), RTLD_NOW | RTLD_GLOBAL) };
    if handle.is_null() { Err(last_dl_error()) } else { Ok(handle) }
}

fn last_dl_error() -> String {
    let error = unsafe { dlerror() };
    if error.is_null() { "no dynamic-linker diagnostic".to_string() }
    else { unsafe { CStr::from_ptr(error) }.to_string_lossy().into_owned() }
}

fn find_named_library(home: &Path, name: &str) -> Option<PathBuf> {
    let direct = [home.join("lib/server").join(name), home.join("lib").join(name), home.join("lib/jli").join(name)];
    if let Some(path) = direct.into_iter().find(|p| p.is_file()) { return Some(path); }
    let mut pending = vec![(home.to_path_buf(), 0usize)];
    let mut seen = BTreeSet::<PathBuf>::new();
    let mut visited = 0usize;
    while let Some((dir, depth)) = pending.pop() {
        if visited >= MAX_SCAN_DIRECTORIES || depth > MAX_SCAN_DEPTH { break; }
        let Ok(canonical) = dir.canonicalize() else { continue };
        if !seen.insert(canonical) { continue; }
        visited += 1;
        let Ok(entries) = fs::read_dir(dir) else { continue };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() { pending.push((path, depth + 1)); }
            else if path.file_name().and_then(|v| v.to_str()) == Some(name) { return Some(path); }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_java_17_release_metadata() {
        assert_eq!(parse_java_version("JAVA_VERSION=\"17.0.13\"\nIMPLEMENTOR=\"Example\""), Some("17.0.13".to_string()));
        assert_eq!(java_major("17.0.13+11"), Some(17));
        assert_eq!(java_major("1.8.0_402"), Some(8));
        assert_eq!(java_major("21.0.1"), Some(21));
    }

    #[test]
    fn ignores_release_files_without_java_version() {
        assert_eq!(parse_java_version("IMPLEMENTOR=\"Example\""), None);
        assert_ne!(java_major("21.0.1"), Some(17));
    }
}
