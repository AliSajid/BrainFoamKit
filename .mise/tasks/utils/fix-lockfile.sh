#!/usr/bin/env bash

# SPDX-FileCopyrightText: 2023 - 2026 Ali Sajid Imami
#
# SPDX-License-Identifier: Apache-2.0
# SPDX-License-Identifier: MIT

#MISE description="Rebuild the Cargo.lock file after fixing conflicts"

if [ -f Cargo.lock ]; then
    echo "Removing the conflicted Cargo.lock file"
    rm -v Cargo.lock
    echo "Updating the cargo dependencies"
    cargo update
elif [[ -f Cargo.toml ]]; then
    echo "No existing lock file found. Updating"
    cargo update
else
    echo "Likely not in a rust project. Aborting"
    exit 1
fi
