#!/bin/sh
# Serve the probe over http://localhost, which WebAuthn accepts as a trustworthy origin.
# NEVER open the pages as file:// URLs, and never over a raw IP address: both are refused
# in a way that looks like missing features rather than a refusal.
#
# Usage: ./serve.sh [port]   (default 8000)
# Then open http://localhost:8000/browser/ for the JavaScript leg
#      and  http://localhost:8000/browser/rust/ for the Rust leg.
exec python3 -m http.server "${1:-8000}"
