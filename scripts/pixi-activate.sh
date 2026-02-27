#!/bin/sh
# Activation script for pixi environment.
# Adds the tpchgen-cli release binary to PATH after building it if needed.

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
RELEASE_BIN="$REPO_ROOT/target/release"

# Build the release binary if it doesn't exist yet
if [ ! -f "$RELEASE_BIN/tpchgen-cli" ]; then
    echo "tpchgen-cli not found, building release binary..."
    cargo build --release --bin tpchgen-cli --manifest-path "$REPO_ROOT/Cargo.toml"
fi

export PATH="$RELEASE_BIN:$PATH"
