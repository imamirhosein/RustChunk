mod workspace;

use jni::objects::{JObject, JString};
use jni::sys::jstring;
use jni::JNIEnv;
use std::path::Path;

/// Reports the native engine target and confirms that the Rust library loaded.
#[no_mangle]
pub extern "system" fn Java_dev_rustchunk_app_MainActivity_nativeStatus(
    mut env: JNIEnv,
    _this: JObject,
) -> jstring {
    match env.new_string("RustChunk native core ready. Target: Minecraft Java 1.20.1") {
        Ok(output) => output.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

/// Initializes the private Minecraft directory layout using Rust filesystem code.
#[no_mangle]
pub extern "system" fn Java_dev_rustchunk_app_MainActivity_nativePrepareWorkspace(
    mut env: JNIEnv,
    _this: JObject,
    app_data_dir: JString,
) -> jstring {
    let message = match env.get_string(&app_data_dir) {
        Ok(java_path) => match java_path.to_str() {
            Ok(path) => workspace::prepare_workspace(Path::new(path))
                .unwrap_or_else(|error| format!("Workspace initialization failed: {}", error)),
            Err(error) => format!("Workspace initialization failed: invalid path: {}", error),
        },
        Err(error) => format!("Workspace initialization failed: JNI string error: {}", error),
    };

    match env.new_string(message) {
        Ok(output) => output.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}
