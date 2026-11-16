//! Android Java runtime smoke test using the JNI Invocation API.
//! The VM is created directly from libjvm.so; app-private bin/java is never executed.

use std::collections::BTreeSet;
use std::ffi::{CStr, CString};
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::os::raw::{c_char, c_int, c_void};
use std::path::{Path, PathBuf};
use std::ptr;
use std::sync::{Mutex, OnceLock};

use jni::objects::{JString, JValue};
use jni::JNIEnv;

const RTLD_LAZY: c_int = 1;
const RTLD_NOW: c_int = 2;
const RTLD_GLOBAL: c_int = 0x100;
const JNI_FALSE: jni::sys::jboolean = jni::sys::JNI_FALSE;
const MAX_SCAN_DIRECTORIES: usize = 25_000;
const MAX_SCAN_DEPTH: usize = 14;
const O_WRONLY: c_int = 1;
const O_CREAT: c_int = 64;
const O_TRUNC: c_int = 512;

#[link(name = "dl")]
unsafe extern "C" {
    fn dlopen(filename: *const c_char, flags: c_int) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    fn dlerror() -> *const c_char;
}

#[link(name = "c")]
unsafe extern "C" {
    // Android/Bionic uses the same signal(2) ABI as POSIX: SIG_DFL=0, SIG_IGN=1.
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn dup(fd: c_int) -> c_int;
    fn dup2(old_fd: c_int, new_fd: c_int) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn fflush(stream: *mut c_void) -> c_int;
}

type JniCreateJavaVm = unsafe extern "system" fn(
    pvm: *mut *mut jni::sys::JavaVM,
    penv: *mut *mut c_void,
    args: *mut c_void,
) -> jni::sys::jint;

type AndroidUpdateLdLibraryPath = unsafe extern "C" fn(*mut c_char);

#[derive(Debug, Clone)]
struct RuntimeCandidate {
    home: PathBuf,
    libjvm: PathBuf,
    version: String,
    library_directories: Vec<PathBuf>,
    native_libraries: Vec<PathBuf>,
}

static JVM_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

struct StdioCapture {
    saved_stdout: c_int,
    saved_stderr: c_int,
    log_fd: c_int,
}

impl StdioCapture {
    fn begin(log_path: &Path) -> Result<Self, String> {
        let path = CString::new(log_path.to_string_lossy().as_bytes())
            .map_err(|_| "JNI console log path contains a NUL byte.".to_string())?;
        let log_fd = unsafe { open(path.as_ptr(), O_WRONLY | O_CREAT | O_TRUNC, 0o600) };
        if log_fd < 0 {
            return Err(format!("Cannot open JNI console log {}.", log_path.display()));
        }

        let saved_stdout = unsafe { dup(1) };
        let saved_stderr = unsafe { dup(2) };
        if saved_stdout < 0 || saved_stderr < 0 {
            unsafe {
                if saved_stdout >= 0 { close(saved_stdout); }
                if saved_stderr >= 0 { close(saved_stderr); }
                close(log_fd);
            }
            return Err("Cannot preserve stdout/stderr before native Java startup.".to_string());
        }

        if unsafe { dup2(log_fd, 1) } < 0 || unsafe { dup2(log_fd, 2) } < 0 {
            unsafe {
                dup2(saved_stdout, 1);
                dup2(saved_stderr, 2);
                close(saved_stdout);
                close(saved_stderr);
                close(log_fd);
            }
            return Err("Cannot redirect native Java diagnostics to the JNI log.".to_string());
        }
        Ok(Self { saved_stdout, saved_stderr, log_fd })
    }
}

impl Drop for StdioCapture {
    fn drop(&mut self) {
        unsafe {
            fflush(ptr::null_mut());
            dup2(self.saved_stdout, 1);
            dup2(self.saved_stderr, 2);
            close(self.saved_stdout);
            close(self.saved_stderr);
            close(self.log_fd);
        }
    }
}

fn prepare_jvm_signal_handlers() -> (usize, usize) {
    // Match the Android OpenJDK launcher convention: use sigaction, not
    // signal(), and temporarily ignore SIGSEGV until HotSpot installs its
    // own crash/null-check handler.
    let mut reset = 0usize;
    let mut ignored_segv = false;
    for signal_number in 1..65 {
        let mut action: libc::sigaction = unsafe { std::mem::zeroed() };
        action.sa_sigaction = if signal_number == libc::SIGSEGV {
            libc::SIG_IGN
        } else {
            libc::SIG_DFL
        };
        let result = unsafe { libc::sigaction(signal_number, &action, ptr::null_mut()) };
        if result == 0 {
            reset += 1;
            if signal_number == libc::SIGSEGV { ignored_segv = true; }
        }
    }
    (reset, if ignored_segv { 1 } else { 0 })
}

pub fn test_java_runtime(app_data_dir: &Path, app_native_library_dir: &Path) -> Result<String, String> {
    if !app_data_dir.is_absolute() || !app_native_library_dir.is_absolute() {
        return Err("RustChunk and Android native library paths must be absolute.".to_string());
    }

    let trace_file = app_data_dir.join("runtime").join("native-jni-trace.txt");
    if let Some(parent) = trace_file.parent() {
        fs::create_dir_all(parent).map_err(|error| format!("Cannot create JNI trace directory: {}", error))?;
    }
    fs::write(&trace_file, "JNI TRACE: START\n").map_err(|error| format!("Cannot create JNI trace: {}", error))?;
    trace_checkpoint(&trace_file, &format!("app data: {}", app_data_dir.display()));
    trace_checkpoint(&trace_file, &format!("native library dir: {}", app_native_library_dir.display()));

    let _guard = JVM_LOCK.get_or_init(|| Mutex::new(())).lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    trace_checkpoint(&trace_file, "JVM mutex acquired; beginning Java runtime discovery");

    let runtime = find_java17_runtime(app_data_dir, &trace_file)?;
    trace_checkpoint(&trace_file, &format!("Java 17 runtime discovered: home={}", runtime.home.display()));
    let home = runtime.home.canonicalize()
        .map_err(|error| format!("Cannot resolve Java home {}: {}", runtime.home.display(), error))?;
    trace_checkpoint(&trace_file, &format!("canonical Java home: {}", home.display()));

    let cache_dir = app_data_dir.join("cache").join("jni");
    fs::create_dir_all(&cache_dir)
        .map_err(|error| format!("Cannot create JNI temporary directory {}: {}", cache_dir.display(), error))?;
    trace_checkpoint(&trace_file, &format!("JNI cache directory ready: {}", cache_dir.display()));

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

    let mut report = String::new();
    report.push_str("RustChunk Android embedded Java runtime test (JNI Invocation API)\n");
    report.push_str(&format!("Java home: {}\n", home.display()));
    report.push_str(&format!("Java release metadata: {}\n", runtime.version));
    report.push_str(&format!("libjvm.so: {}\n", runtime.libjvm.display()));
    report.push_str(&format!("Android native library directory: {}\n", app_native_library_dir.display()));
    report.push_str(&format!("Runtime library directories: {}\n", directories.iter().filter(|p| p.is_dir()).count()));
    report.push_str("Startup method: JNI_CreateJavaVM; app-private bin/java is not executed.\n");

    trace_checkpoint(&trace_file, "resolving Android linker path bridge");
    if let Some(update_path) = resolve_android_update_library_path() {
        trace_checkpoint(&trace_file, "Android linker path bridge found; applying search path");
        let path_c = CString::new(ld_path.as_bytes())
            .map_err(|_| "LD_LIBRARY_PATH contains an embedded NUL byte.".to_string())?;
        unsafe { update_path(path_c.as_ptr() as *mut c_char); }
        report.push_str("Android linker search path updated through native loader bridge.\n");
    } else {
        report.push_str("NOTICE: Android loader path bridge unavailable; absolute library paths will be used.\n");
    }

    // Retain all handles for the full VM lifetime. HotSpot can resolve runtime
    // symbols lazily after JNI_CreateJavaVM returns.
    trace_checkpoint(&trace_file, "preloading OpenJDK runtime libraries");
    let _preloaded_runtime_handles = preload_runtime_libraries(&runtime, &trace_file)?;
    trace_checkpoint(&trace_file, "runtime native library preload complete");

    trace_checkpoint(&trace_file, "preparing JVM signal handlers in isolated :jvm process");
    let (signals_reset, segv_ignored) = prepare_jvm_signal_handlers();
    trace_checkpoint(&trace_file, &format!(
        "signal handlers prepared: {} reset; SIGSEGV ignored={}",
        signals_reset, segv_ignored == 1
    ));
    report.push_str(&format!(
        "Android signal handlers prepared: {} reset; SIGSEGV ignored: {}\n",
        signals_reset, segv_ignored == 1
    ));

    trace_checkpoint(&trace_file, &format!("opening libjvm for JNI_CreateJavaVM: {}", runtime.libjvm.display()));
    let jvm_handle = open_library(&runtime.libjvm)
        .map_err(|error| format!("Cannot load libjvm.so: {}\n{}", error, report))?;

    let symbol_name = CString::new("JNI_CreateJavaVM").unwrap();
    unsafe { dlerror(); }
    let symbol = unsafe { dlsym(jvm_handle, symbol_name.as_ptr()) };
    if symbol.is_null() {
        return Err(format!(
            "JNI_CreateJavaVM is not exported by {}: {}\n{}",
            runtime.libjvm.display(), last_dl_error(), report
        ));
    }
    let create_vm: JniCreateJavaVm = unsafe { std::mem::transmute(symbol) };
    trace_checkpoint(&trace_file, "JNI_CreateJavaVM symbol resolved successfully");

    let library_path = directories.iter()
        .filter(|path| path.is_dir())
        .map(|path| path.to_string_lossy().into_owned())
        .collect::<Vec<_>>()
        .join(":");

    let option_values = vec![
        format!("-Djava.home={}", home.to_string_lossy()),
        format!("-Djava.io.tmpdir={}", cache_dir.to_string_lossy()),
        format!("-Duser.home={}", app_data_dir.to_string_lossy()),
        format!("-Djava.library.path={}", library_path),
        "-Djava.awt.headless=true".to_string(),
        "-Xms32m".to_string(),
        "-Xmx512m".to_string(),
    ];
    let c_options: Vec<CString> = option_values.iter()
        .map(|value| CString::new(value.as_str())
            .map_err(|_| format!("JVM option contains an embedded NUL byte: {}", value)))
        .collect::<Result<_, _>>()?;
    let mut vm_options: Vec<jni::sys::JavaVMOption> = c_options.iter()
        .map(|value| jni::sys::JavaVMOption {
            optionString: value.as_ptr() as *mut c_char,
            extraInfo: ptr::null_mut(),
        })
        .collect();

    let mut vm_args = jni::sys::JavaVMInitArgs {
        version: jni::sys::JNI_VERSION_1_8,
        nOptions: vm_options.len() as jni::sys::jint,
        options: vm_options.as_mut_ptr(),
        ignoreUnrecognized: JNI_FALSE,
    };

    trace_checkpoint(&trace_file, &format!("JNI init args prepared: version=0x00010008; options={}", option_values.join(" || ")));
    let console_log = cache_dir.join("native-jni-console.log");
    report.push_str(&format!("JNI console log: {}\n", console_log.display()));
    report.push_str("Calling JNI_CreateJavaVM from libjvm.so.\n");

    let capture = StdioCapture::begin(&console_log)
        .map_err(|error| format!("{}\n{}", error, report))?;
    trace_checkpoint(&trace_file, "stdio capture ready; entering JNI_CreateJavaVM");
    let mut raw_vm: *mut jni::sys::JavaVM = ptr::null_mut();
    let mut raw_env: *mut c_void = ptr::null_mut();
    let create_status = unsafe {
        create_vm(
            &mut raw_vm,
            &mut raw_env,
            &mut vm_args as *mut jni::sys::JavaVMInitArgs as *mut c_void,
        )
    };
    drop(capture);

    trace_checkpoint(&trace_file, &format!(
        "JNI_CreateJavaVM returned: status={}, vm_is_null={}, env_is_null={}",
        create_status, raw_vm.is_null(), raw_env.is_null()
    ));

    let console_output = fs::read_to_string(&console_log).unwrap_or_default();
    if !console_output.trim().is_empty() {
        report.push_str("\nJNI console output:\n");
        report.push_str(&console_output);
        if !console_output.ends_with('\n') { report.push('\n'); }
    }

    if create_status != jni::sys::JNI_OK {
        report.push_str(&format!("JNI_CreateJavaVM status: {}\n", create_status));
        report.push_str("RESULT: JAVA 17 JNI STARTUP FAILED\n");
        report.push_str("JNI_CreateJavaVM was called but did not initialize a usable Java VM.\n");
        report.push_str("Forge, Minecraft graphics, mods, and Aternos networking have not been tested.\n");
        return Err(report);
    }
    if raw_vm.is_null() || raw_env.is_null() {
        report.push_str("JNI_CreateJavaVM returned JNI_OK but provided a null VM or JNIEnv pointer.\n");
        report.push_str("RESULT: JAVA 17 JNI STARTUP FAILED\n");
        return Err(report);
    }

    trace_checkpoint(&trace_file, "wrapping the JNI environment returned by JNI_CreateJavaVM");
    let env_result = unsafe { JNIEnv::from_raw(raw_env as *mut jni::sys::JNIEnv) };
    let mut env = match env_result {
        Ok(env) => env,
        Err(error) => {
            let detach_result = detach_current_jni_thread(raw_vm);
            trace_checkpoint(&trace_file, &format!("thread detach after JNIEnv wrapping failure: {:?}", detach_result));
            return Err(format!("Cannot wrap the JNIEnv returned by JNI_CreateJavaVM: {}\n{}", error, report));
        }
    };

    trace_checkpoint(&trace_file, "JNI environment created; reading Java system properties");
    let properties_result = (|| -> Result<(String, String, String), String> {
        let version = read_java_property(&mut env, "java.version")?;
        let java_home = read_java_property(&mut env, "java.home")?;
        let architecture = read_java_property(&mut env, "os.arch")?;
        Ok((version, java_home, architecture))
    })();

    drop(env);
    let detach_result = detach_current_jni_thread(raw_vm);
    trace_checkpoint(&trace_file, &format!("created-VM thread detach result: {:?}", detach_result));

    let (java_version, reported_home, architecture) = properties_result?;
    report.push_str(&format!("Java system property java.version: {}\n", java_version));
    report.push_str(&format!("Java system property java.home: {}\n", reported_home));
    report.push_str(&format!("Java system property os.arch: {}\n", architecture));
    match detach_result {
        Ok(()) => report.push_str("JNI-created thread detached cleanly.\n"),
        Err(error) => report.push_str(&format!("WARNING: JNI-created thread detach failed: {}\n", error)),
    }

    if java_major(&java_version) != Some(17) {
        report.push_str("RESULT: JAVA 17 JNI STARTUP FAILED (unexpected Java major version)\n");
        return Err(report);
    }
    if architecture != "aarch64" && architecture != "arm64" {
        report.push_str("RESULT: JAVA 17 JNI STARTUP FAILED (unexpected architecture)\n");
        return Err(report);
    }

    report.push_str("RESULT: JAVA 17 JNI STARTUP OK\n");
    report.push_str("JNI_CreateJavaVM initialized HotSpot without executing bin/java.\n");
    report.push_str("This validates Java VM startup only. Forge, Minecraft graphics, mods, and Aternos networking have not been tested.\n");
    trace_checkpoint(&trace_file, "RESULT: JAVA 17 JNI STARTUP OK");
    Ok(report)
}

fn read_java_property(env: &mut JNIEnv<'_>, name: &str) -> Result<String, String> {
    let key = env.new_string(name)
        .map_err(|error| format!("Cannot create Java property key '{}': {}", name, error))?;
    let key_object = jni::objects::JObject::from(key);
    let result = env.call_static_method(
        "java/lang/System",
        "getProperty",
        "(Ljava/lang/String;)Ljava/lang/String;",
        &[JValue::Object(&key_object)],
    ).map_err(|error| format!("System.getProperty('{}') failed: {}", name, error))?;
    let value = result.l()
        .map_err(|error| format!("System.getProperty('{}') returned a non-object: {}", name, error))?;
    if value.is_null() {
        return Err(format!("System.getProperty('{}') returned null.", name));
    }
    let value = JString::from(value);
    env.get_string(&value)
        .map(|text| text.to_string_lossy().into_owned())
        .map_err(|error| format!("Cannot read Java system property '{}': {}", name, error))
}

fn detach_current_jni_thread(vm: *mut jni::sys::JavaVM) -> Result<(), String> {
    if vm.is_null() {
        return Err("JavaVM pointer is null; cannot detach the JNI-created thread.".to_string());
    }
    let interface = unsafe { *vm };
    if interface.is_null() {
        return Err("JavaVM invocation table is null; cannot detach the JNI-created thread.".to_string());
    }
    // DetachCurrentThread is part of the base JNI invocation table.
    let detach = unsafe { (*interface).DetachCurrentThread }
        .ok_or_else(|| "JavaVM does not expose DetachCurrentThread.".to_string())?;
    let status = unsafe { detach(vm) };
    if status == jni::sys::JNI_OK {
        Ok(())
    } else {
        Err(format!("DetachCurrentThread returned JNI status {}.", status))
    }
}

fn trace_checkpoint(path: &Path, message: &str) {
    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(path) {
        let _ = writeln!(file, "{}", message);
        let _ = file.sync_data();
    }
}

fn preload_runtime_libraries(
    runtime: &RuntimeCandidate,
    trace_file: &Path,
) -> Result<Vec<*mut c_void>, String> {
    let mut ordered = Vec::<PathBuf>::new();
    ordered.push(runtime.libjvm.clone());

    // These are the core libraries preloaded by the established Android
    // OpenJDK launcher before it calls JNI_CreateJavaVM.
    for name in [
        "libverify.so", "libjava.so", "libnet.so", "libnio.so",
        "libawt.so", "libawt_headless.so", "libfreetype.so", "libfontmanager.so",
    ] {
        if let Some(path) = runtime.native_libraries.iter()
            .find(|path| path.file_name().and_then(|v| v.to_str()) == Some(name)) {
            ordered.push(path.clone());
        }
    }

    // Match the launcher's recursive preload of the runtime's Android
    // architecture directory. Optional libraries may legitimately fail to
    // load; record them and continue. libjvm is mandatory.
    ordered.extend(runtime.native_libraries.iter().cloned());

    let mut seen = BTreeSet::<PathBuf>::new();
    let mut handles = Vec::new();
    for path in ordered {
        let identity = path.canonicalize().unwrap_or_else(|_| path.clone());
        if !seen.insert(identity) { continue; }
        let name = path.file_name().and_then(|v| v.to_str()).unwrap_or("unknown");
        trace_checkpoint(trace_file, &format!("preloading runtime library: {}", path.display()));
        match open_library(&path) {
            Ok(handle) => {
                trace_checkpoint(trace_file, &format!("runtime library loaded: {}", name));
                handles.push(handle);
            }
            Err(error) if path == runtime.libjvm => {
                trace_checkpoint(trace_file, &format!("FATAL: libjvm preload failed: {}", error));
                return Err(format!("Cannot preload required libjvm.so at {}: {}", path.display(), error));
            }
            Err(error) => {
                trace_checkpoint(trace_file, &format!("optional runtime library failed: {}: {}", name, error));
            }
        }
    }
    trace_checkpoint(trace_file, &format!("runtime libraries successfully preloaded: {}", handles.len()));
    Ok(handles)
}

fn find_java17_runtime(app_data_dir: &Path, trace_file: &Path) -> Result<RuntimeCandidate, String> {
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
        if root.is_dir() {
            trace_checkpoint(trace_file, &format!("runtime scan starting: {}", root.display()));
            let before = visited;
            scan_runtime_tree(root, &mut files, &mut visited, trace_file);
            trace_checkpoint(trace_file, &format!("runtime scan finished: {} new directories; {} candidate files", visited.saturating_sub(before), files.len()));
            let has_jvm = files.iter().any(|p| p.file_name().and_then(|s| s.to_str()) == Some("libjvm.so"));
            if has_jvm {
                trace_checkpoint(trace_file, "libjvm.so found; skipping other search roots");
                break;
            }
        } else {
            trace_checkpoint(trace_file, &format!("runtime scan root missing: {}", root.display()));
        }
    }
    trace_checkpoint(trace_file, &format!("runtime discovery scan complete: {} directories visited; {} files indexed", visited, files.len()));

    let jvm_paths: Vec<PathBuf> = files.iter()
        .filter(|p| p.file_name().and_then(|s| s.to_str()) == Some("libjvm.so"))
        .cloned().collect();

    let mut candidates = Vec::new();
    for libjvm in jvm_paths {
        let Some(home) = libjvm.ancestors()
            .find(|ancestor| ancestor.join("release").is_file())
            .map(Path::to_path_buf) else { continue };
        let release = fs::read_to_string(home.join("release")).unwrap_or_default();
        let Some(version) = parse_java_version(&release) else { continue };
        if java_major(&version) != Some(17) { continue; }
        if !is_aarch64_elf(&libjvm) { continue; }
        let native_libraries: Vec<PathBuf> = files.iter()
            .filter(|p| p.starts_with(&home) && p.extension().and_then(|e| e.to_str()) == Some("so"))
            .cloned()
            .collect();
        let library_directories: BTreeSet<PathBuf> = native_libraries.iter()
            .filter_map(|p| p.parent().map(Path::to_path_buf))
            .collect();
        candidates.push(RuntimeCandidate {
            home, libjvm, version,
            library_directories: library_directories.into_iter().collect(),
            native_libraries,
        });
    }

    candidates.sort_by(|a,b| {
        let ap = a.home.ends_with("Internal-17");
        let bp = b.home.ends_with("Internal-17");
        bp.cmp(&ap).then_with(|| a.home.cmp(&b.home))
    });
    candidates.into_iter().next().ok_or_else(|| {
        format!("No ARM64 Java 17 runtime found. Required: release with JAVA_VERSION 17 and libjvm.so. Scanned {} files; roots: {}",
            files.len(), roots.iter().map(|p| format!("{} ({})", p.display(), if p.is_dir(){"present"}else{"missing"})).collect::<Vec<_>>().join("; "))
    })
}

fn scan_runtime_tree(root: &Path, files: &mut Vec<PathBuf>, visited_total: &mut usize, trace_file: &Path) {
    let mut pending = vec![(root.to_path_buf(), 0usize)];
    let mut seen = BTreeSet::<PathBuf>::new();
    while let Some((directory, depth)) = pending.pop() {
        if *visited_total >= MAX_SCAN_DIRECTORIES || depth > MAX_SCAN_DEPTH { break; }
        let Ok(canonical) = directory.canonicalize() else { continue };
        if !seen.insert(canonical) { continue; }
        *visited_total += 1;
        if *visited_total % 500 == 0 {
            trace_checkpoint(trace_file, &format!("runtime scan progress: {} directories visited", *visited_total));
        }
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
