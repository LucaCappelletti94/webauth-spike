#!/bin/sh
# Package a probe binary into a signed .app so it can reach the data protection keychain.
#
# The keychain-item probes (security_framework_probe for N1 and N2, apple_keyring_probe for
# N3) use kSecAttrAccessControl, which routes through the data protection keychain. macOS only
# grants that from a binary carrying the keychain-access-groups entitlement, and it only honors
# that restricted entitlement when an embedded provisioning profile authorizes it. A bare
# signed CLI is killed at exec by AMFI. So the binary has to live in a .app with a profile.
#
# Generate the profile once (see README): a minimal Xcode macOS app with bundle id
# com.connetto.probe, team 7W8527FJJE, and the Keychain Sharing capability, built for My Mac,
# which registers this Mac and mints a Development profile. Then run this script.
#
# Usage:
#   SIGN_IDENTITY="Apple Development: you (CERTID)" \
#     ./package-app.sh <bin-name> <path-to.provisionprofile>
#
# Then run:  target/app/<bin-name>.app/Contents/MacOS/<bin-name>
set -eu

BIN="$1"
PROFILE="$2"
IDENTITY="${SIGN_IDENTITY:?set SIGN_IDENTITY to your Apple Development identity, see security find-identity -v -p codesigning}"

OUT="target/app/$BIN.app"
rm -rf "$OUT"
mkdir -p "$OUT/Contents/MacOS"
cp "target/debug/$BIN" "$OUT/Contents/MacOS/$BIN"
cp "$PROFILE" "$OUT/Contents/embedded.provisionprofile"

cat > "$OUT/Contents/Info.plist" <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0"><dict>
<key>CFBundleExecutable</key><string>$BIN</string>
<key>CFBundleIdentifier</key><string>com.connetto.probe</string>
<key>CFBundleName</key><string>$BIN</string>
<key>CFBundlePackageType</key><string>APPL</string>
<key>CFBundleVersion</key><string>1</string>
<key>CFBundleShortVersionString</key><string>1.0</string>
</dict></plist>
EOF

codesign --force --sign "$IDENTITY" --entitlements entitlements.plist "$OUT"
echo "signed $OUT"
echo "run: $OUT/Contents/MacOS/$BIN"
