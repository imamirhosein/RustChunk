mod downloader;
mod installer;
mod workspace;

use jni::objects::{JObject, JString};
use jni::sys::{jint, jstring};
use jni::JNIEnv;
use std::path::Path;

fn java_string_to_rust(env: &mut JNIEnv, value: &JString) -> Result<String, String> {
    env.get_string(value)
        .map(|text| text.to_string_lossy().into_owned())
        .map_err(|error| format!("JNI string error: {}", error))
}

fn rust_string_to_java(env: &mut JNIEnv, value: String) -> jstring {
    match env.new_string(value) {
        Ok(output) => output.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "system" fn Java_dev_rustchunk_app_MainActivity_nativeStatus(
    mut env: JNIEnv,
    _this: JObject,
) -> jstring {
    rust_string_to_java(
        &mut env,
        "RustChunk native core ready. Target: Minecraft Java 1.20.1".to_string(),
    )
}

#[no_mangle]
pub extern "system" fn Java_dev_rustchunk_app_MainActivity_nativePrepareWorkspace(
    mut env: JNIEnv,
    _this: JObject,
    app_data_dir: JString,
) -> jstring {
    let result = java_string_to_rust(&mut env, &app_data_dir)
        .and_then(|path| workspace::prepare_workspace(Path::new(&path)));
    rust_string_to_java(
        &mut env,
        result.unwrap_or_else(|error| format!("Workspace initialization failed: {}", error)),
    )
}

#[no_mangle]
pub extern "system" fn Java_dev_rustchunk_app_MainActivity_nativeResolveMinecraft(
    mut env: JNIEnv,
    _this: JObject,
    app_data_dir: JString,
) -> jstring {
    let result = java_string_to_rust(&mut env, &app_data_dir)
        .and_then(|path| installer::resolve_minecraft(Path::new(&path)));
    rust_string_to_java(
        &mut env,
        result.unwrap_or_else(|error| format!("Metadata resolution failed: {}", error)),
    )
}

#[no_mangle]
pub extern "system" fn Java_dev_rustchunk_app_MainActivity_nativeDownloadMinecraft(
    mut env: JNIEnv,
    _this: JObject,
    app_data_dir: JString,
    parallelism: jint,
) -> jstring {
    let result = java_string_to_rust(&mut env, &app_data_dir).and_then(|path| {
        downloader::download_minecraft(Path::new(&path), parallelism.max(1) as usize)
    });
    rust_string_to_java(
        &mut env,
        result.unwrap_or_else(|error| format!("Download failed: {}", error)),
    )
}

#[no_mangle]
pub extern "system" fn Java_dev_rustchunk_app_MainActivity_nativeDownloadProgress(
    mut env: JNIEnv,
    _this: JObject,
) -> jstring {
    rust_string_to_java(&mut env, downloader::download_progress())
}

#[no_mangle]
pub extern "system" fn Java_dev_rustchunk_app_MainActivity_nativeCancelDownload(
    mut env: JNIEnv,
    _this: JObject,
) -> jstring {
    rust_string_to_java(&mut env, downloader::cancel_download())
}
