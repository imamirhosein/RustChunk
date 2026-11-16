mod downloader;
mod forge;
mod installer;
mod jli;
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
pub extern "system" fn Java_dev_rustchunk_app_MainActivity_nativeDownloadForgeLibraries(
    mut env: JNIEnv,
    _this: JObject,
    app_data_dir: JString,
    profile_id: JString,
    parallelism: jint,
) -> jstring {
    let result = java_string_to_rust(&mut env, &app_data_dir).and_then(|path| {
        java_string_to_rust(&mut env, &profile_id).and_then(|id| {
            let data_dir = Path::new(&path);
            let plan_report = forge::prepare_forge_dependency_plan(data_dir, &id)?;
            let download_report = downloader::download_forge_libraries(
                data_dir, &id, parallelism.clamp(1, 8) as usize
            )?;
            Ok(format!("{}\n\n{}", plan_report, download_report))
        })
    });
    rust_string_to_java(
        &mut env,
        result.unwrap_or_else(|error| format!("Forge dependency download failed: {}", error)),
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


#[no_mangle]
pub extern "system" fn Java_dev_rustchunk_app_MainActivity_nativeListMinecraftProfiles(
    mut env: JNIEnv,
    _this: JObject,
    app_data_dir: JString,
) -> jstring {
    let result = java_string_to_rust(&mut env, &app_data_dir)
        .and_then(|path| forge::list_profiles(Path::new(&path)));
    rust_string_to_java(
        &mut env,
        result.unwrap_or_else(|error| format!("Profile scan failed: {}", error)),
    )
}

#[no_mangle]
pub extern "system" fn Java_dev_rustchunk_app_MainActivity_nativePrepareForgeLaunchProfile(
    mut env: JNIEnv,
    _this: JObject,
    app_data_dir: JString,
    profile_id: JString,
) -> jstring {
    let result = java_string_to_rust(&mut env, &app_data_dir).and_then(|path| {
        java_string_to_rust(&mut env, &profile_id)
            .and_then(|id| forge::prepare_forge_profile(Path::new(&path), &id))
    });
    rust_string_to_java(
        &mut env,
        result.unwrap_or_else(|error| format!("Forge profile preparation failed: {}", error)),
    )
}


#[no_mangle]
pub extern "system" fn Java_dev_rustchunk_app_MainActivity_nativePrepareJniForgeLaunchPlan(
    mut env: JNIEnv,
    _this: JObject,
    app_data_dir: JString,
    profile_id: JString,
    offline_username: JString,
    native_library_dir: JString,
) -> jstring {
    let result = java_string_to_rust(&mut env, &app_data_dir).and_then(|data| {
        java_string_to_rust(&mut env, &profile_id).and_then(|profile| {
            java_string_to_rust(&mut env, &offline_username).and_then(|username| {
                java_string_to_rust(&mut env, &native_library_dir).and_then(|native_dir| {
                    forge::prepare_jni_forge_launch_plan(
                        Path::new(&data),
                        &profile,
                        &username,
                        Path::new(&native_dir),
                    )
                })
            })
        })
    });
    rust_string_to_java(
        &mut env,
        result.unwrap_or_else(|error| format!("JNI Forge launch-plan preparation failed: {}", error)),
    )
}

#[no_mangle]
pub extern "system" fn Java_dev_rustchunk_app_JavaRuntimeTestService_nativeTestJavaRuntimeNative(
    mut env: JNIEnv,
    _this: JObject,
    app_data_dir: JString,
    native_library_dir: JString,
) -> jstring {
    let result = java_string_to_rust(&mut env, &app_data_dir).and_then(|data| {
        java_string_to_rust(&mut env, &native_library_dir)
            .and_then(|native_dir| jli::test_java_runtime(Path::new(&data), Path::new(&native_dir)))
    });
    rust_string_to_java(
        &mut env,
        result.unwrap_or_else(|error| format!("Native Java JNI test failed: {}", error)),
    )
}
