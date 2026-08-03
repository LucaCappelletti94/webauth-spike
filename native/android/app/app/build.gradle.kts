plugins {
    id("com.android.application")
}

android {
    namespace = "com.connetto.probe"
    compileSdk = 34

    defaultConfig {
        applicationId = "com.connetto.probe"
        // API 23 is the floor for setUserAuthenticationRequired. 24 keeps things simple.
        minSdk = 24
        targetSdk = 34
        versionCode = 1
        versionName = "0.1"
    }

    // The Rust cdylib is built separately (see README) and dropped into these directories,
    // one per ABI: src/main/jniLibs/<abi>/libwebauth_probe_android.so
    sourceSets["main"].jniLibs.srcDirs("src/main/jniLibs")

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }
}
