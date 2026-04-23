#!/bin/bash
set -e

SOURCE_ROOT="$1"
BUILDTYPE="$2"
OUTPUT="$3"

CARGO_MANIFEST="$SOURCE_ROOT/Cargo.toml"

if [[ "$BUILDTYPE" == "release" ]]; then
    cargo build --manifest-path "$CARGO_MANIFEST" --release
    cp "$SOURCE_ROOT/target/release/scratchpad" "$OUTPUT"
else
    cargo build --manifest-path "$CARGO_MANIFEST"
    cp "$SOURCE_ROOT/target/debug/scratchpad" "$OUTPUT"
fi
