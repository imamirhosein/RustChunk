plugins {
    id("com.android.application")
    id("org.jetbrains.kotlin.android")
}

val rustChunkDebugKeystore = rootProject.file(".ci-signing/rustchunk-debug.keystore")

android {
    namespace = "dev.rustchunk.app"
    compileSdk = 35

    defaultConfig {
        applicationId = "dev.rustchunk.app"
        minSdk = 26
        targetSdk = 35
        versionCode = 15
        versionName = "0.16.0-stage16-exit-diagnostics"

        ndk {
            abiFilters += "arm64-v8a"
        }
    }

    signingConfigs {
        create("rustchunkDebug") {
            storeFile = rustChunkDebugKeystore
            storePassword = "android"
            keyAlias = "rustchunkdebug"
            keyPassword = "android"
        }
    }

    buildTypes {
        getByName("debug") {
            signingConfig = signingConfigs.getByName("rustchunkDebug")
        }
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }

    kotlinOptions {
        jvmTarget = "17"
    }

}

dependencies {
    // Java 17 runtime archives are read from the installed Zalith APK and extracted with symlink support.
    implementation("org.apache.commons:commons-compress:1.28.0")
    implementation("org.tukaani:xz:1.12")
}
