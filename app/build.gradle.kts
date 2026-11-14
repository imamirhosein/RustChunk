plugins {
    id("com.android.application")
    id("org.jetbrains.kotlin.android")
}

android {
    namespace = "dev.rustchunk.app"
    compileSdk = 35

    defaultConfig {
        applicationId = "dev.rustchunk.app"
        minSdk = 26
        targetSdk = 35
        versionCode = 2
        versionName = "0.2.0-stage2"

        ndk {
            abiFilters += "arm64-v8a"
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
