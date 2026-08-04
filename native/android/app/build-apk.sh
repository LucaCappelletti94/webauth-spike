#!/bin/sh
# Build the A6 native-gate app APK by hand (no Gradle): compile the Rust cdylib for arm64
# with the NDK, then assemble and sign an APK bundling the .so. Requires the Android SDK
# (build-tools, a platform, an NDK) and a debug keystore.
set -eu

SDK="${ANDROID_HOME:-$HOME/Android/sdk}"
BT="$SDK/build-tools/35.0.1"
PLAT="$SDK/platforms/android-35/android.jar"
KS="$HOME/.android/debug.keystore"
NDK="$SDK/ndk/28.2.13676358"
TC="$NDK/toolchains/llvm/prebuilt/linux-x86_64/bin"

HERE="$(cd "$(dirname "$0")" && pwd)"
RUST="$HERE/../rust"
SRC="$HERE/app/src/main/java/com/connetto/probe"

# 1. Rust cdylib for arm64.
( cd "$RUST" && \
  CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER="$TC/aarch64-linux-android24-clang" \
  CC_aarch64_linux_android="$TC/aarch64-linux-android24-clang" \
  AR_aarch64_linux_android="$TC/llvm-ar" \
  cargo build --release --target aarch64-linux-android )
SO="$RUST/target/aarch64-linux-android/release/libwebauth_probe_android.so"

# 2. Assemble APK.
cd "$HERE"
rm -rf build
mkdir -p build/gen build/obj build/lib/arm64-v8a
cp "$SO" build/lib/arm64-v8a/libwebauth_probe_android.so

cat > build/AndroidManifest.xml <<'EOF'
<?xml version="1.0" encoding="utf-8"?>
<manifest xmlns:android="http://schemas.android.com/apk/res/android" package="com.connetto.probe">
    <uses-permission android:name="android.permission.USE_BIOMETRIC" />
    <application android:allowBackup="false" android:label="webauth probe" android:extractNativeLibs="true">
        <activity android:name=".MainActivity" android:exported="true">
            <intent-filter>
                <action android:name="android.intent.action.MAIN" />
                <category android:name="android.intent.category.LAUNCHER" />
            </intent-filter>
        </activity>
    </application>
</manifest>
EOF

"$BT/aapt2" link -o build/base.apk -I "$PLAT" \
  --manifest build/AndroidManifest.xml --java build/gen \
  --min-sdk-version 24 --target-sdk-version 34
javac -source 17 -target 17 -classpath "$PLAT" -d build/obj \
  "$SRC/ProbeBridge.java" "$SRC/MainActivity.java"
"$BT/d8" --min-api 24 --output build build/obj/com/connetto/probe/*.class
( cd build && zip -qj base.apk classes.dex && zip -q base.apk lib/arm64-v8a/libwebauth_probe_android.so )
"$BT/zipalign" -f 4 build/base.apk build/probe-aligned.apk
"$BT/apksigner" sign \
  --ks "$KS" --ks-pass pass:android --key-pass pass:android --ks-key-alias androiddebugkey \
  build/probe-aligned.apk

echo "built build/probe-aligned.apk"
