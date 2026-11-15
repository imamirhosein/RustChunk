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
        versionCode = 9
        versionName = "0.10.0-stage10-java-runtime-test"

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
