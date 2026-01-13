#!/usr/bin/env bash
# SPDX-FileCopyrightText: 2023 - 2026 Ali Sajid Imami
#
# SPDX-License-Identifier: Apache-2.0
# SPDX-License-Identifier: MIT

set -euo pipefail
set -x

# Set the necessary variables

# Check the files in the source folder
tree artifacts

# Check the files in the dist folder
mkdir -p dist

# Copy binaries with the rust triple to the dist folder
# Expected structure: artifacts/<bin-target>/<bin-target>
find artifacts -mindepth 2 -maxdepth 2 -type f | while IFS= read -r filepath; do
    artifact_name=$(basename "$filepath")
    dir_name=$(basename "$(dirname "$filepath")")
    # Only copy if the filename matches the directory name (expected structure)
    if [ "$artifact_name" = "$dir_name" ]; then
        cp -v "$filepath" "dist/$artifact_name"
    fi
done

# Create the checksums
shasum -a 256 dist/* | sed 's/dist\///' | tee dist/SHA256SUMS.txt

# Sign the files
for file in dist/*; do
    gpg --armor --output "$file.asc" --detach-sig "$file"
done

tree -a dist/

set +x
