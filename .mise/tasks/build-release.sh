#!/usr/bin/env bash

# SPDX-FileCopyrightText: 2023 - 2026 Ali Sajid Imami
#
# SPDX-License-Identifier: Apache-2.0
# SPDX-License-Identifier: MIT

#MISE description="Generate and sign artifacts for a new release"
#MISE alias="br"

set -euo pipefail

# Inputs
BINARIES=("$@") # List of binaries to build
TARGET=${TARGET:?}
CROSS=${CROSS:-false}
DIST_DIR=dist

# Install dependencies for Linux when not cross-compiling
if [[ "$(uname)" == "Linux" && "$CROSS" == "false" ]]; then
    sudo apt update
    sudo apt install -y musl-tools mingw-w64
fi

# Setup cargo command
CARGO="cargo"
if [[ "$CROSS" == "true" ]]; then
    CARGO="cross"
fi

# Create the dist directory
mkdir -p "$DIST_DIR"

# Build and package binaries
for BIN in "${BINARIES[@]}"; do
    echo "Building $BIN for $TARGET..."
    $CARGO build --bin "$BIN" --release --target "$TARGET" --verbose

    # Handle Windows and non-Windows packaging
    if [[ "$TARGET" == *windows-gnu ]]; then
        cp "target/$TARGET/release/$BIN.exe" "$DIST_DIR/$BIN-$TARGET.exe"
    else
        cp "target/$TARGET/release/$BIN" "$DIST_DIR/$BIN-$TARGET"
    fi

    # Generate checksum
    echo "Generating checksum for $BIN..."
    shasum --algorithm 256 "$DIST_DIR/$BIN-$TARGET" >"$DIST_DIR/$BIN-$TARGET-SHA256SUM.txt"
done
