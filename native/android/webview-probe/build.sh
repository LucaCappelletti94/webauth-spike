#!/bin/sh
# Build a minimal WebView APK by hand (no Gradle) to measure A5: whether an Android
# WebView exposes WebAuthn. Requires the Android SDK (build-tools and a platform).
set -eu

SDK="${ANDROID_HOME:-$HOME/Android/sdk}"
BT="$SDK/build-tools/35.0.1"
PLAT="$SDK/platforms/android-35/android.jar"
KS="$HOME/.android/debug.keystore"

cd "$(dirname "$0")"
rm -rf build
mkdir -p build/gen build/obj

# Package the manifest into a base APK (no resources).
"$BT/aapt2" link -o build/base.apk -I "$PLAT" \
  --manifest AndroidManifest.xml --java build/gen \
  --min-sdk-version 24 --target-sdk-version 34

# Compile and dex the activity.
javac -source 17 -target 17 -classpath "$PLAT" -d build/obj MainActivity.java
"$BT/d8" --min-api 24 --output build build/obj/com/connetto/webviewprobe/*.class

# Add the dex, align, sign.
(cd build && zip -qj base.apk classes.dex)
"$BT/zipalign" -f 4 build/base.apk build/webviewprobe-aligned.apk
"$BT/apksigner" sign \
  --ks "$KS" --ks-pass pass:android --key-pass pass:android --ks-key-alias androiddebugkey \
  build/webviewprobe-aligned.apk

echo "built build/webviewprobe-aligned.apk"
