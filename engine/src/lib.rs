use jni::objects::{JObject, JString};
use jni::sys::jstring;
use jni::JNIEnv;

/// Native smoke-test endpoint. This verifies that the Android app can load
/// and call the Rust library; it is not yet a Minecraft implementation.
#[no_mangle]
pub extern "system" fn Java_dev_rustchunk_app_MainActivity_nativeStatus(
    mut env: JNIEnv,
    _this: JObject,
) -> jstring {
    let message = "RustChunk native core loaded. Target game profile: Minecraft 1.20.1";
    let output: JString = env
        .new_string(message)
        .expect("could not allocate Java status string");
    output.into_raw()
}

#[cfg(test)]
mod tests {
    #[test]
    fn target_profile_is_1201() {
        let target = (1, 20, 1);
        assert_eq!(target, (1, 20, 1));
    }
}
