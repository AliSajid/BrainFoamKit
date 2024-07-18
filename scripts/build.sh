#!/usr/bin/env bash
# SPDX-FileCopyrightText: 2024 Ali Sajid Imami
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
for filename in $(ls artifacts); do
  if [ -f "artifacts/$filename/$filename" ]; then
    cp -v "artifacts/$filename/$filename" "dist/$filename"
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
