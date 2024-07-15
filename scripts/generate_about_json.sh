#!/usr/bin/env bash

# SPDX-FileCopyrightText: 2023 - 2024 Ali Sajid Imami
#
# SPDX-License-Identifier: Apache-2.0
# SPDX-License-Identifier: MIT

# A script that uses `cargo about` to generate a machine-readable summary of
# the current crate's dependencies and their licenses and saves them
# to a JSON file.

# The script is intended to be run from the root of the crate.
# It can take an optional parameter which will be the path
# to the JSON file to be generated. If no parameter is given,
# the file will be generated in the root of the crate.

## Step 1: Check that rustc and cargo are installed

if ! command -v rustc &> /dev/null
then
    echo "rustc could not be found"
    exit
fi

if ! command -v cargo &> /dev/null
then
    echo "cargo could not be found"
    exit
fi

## Step 2: Check that cargo-about is installed

if ! command -v cargo-about &> /dev/null
then
    echo "cargo-about could not be installed"
    exit
fi

## Step 3: Check that jq is installed

if ! command -v jq &> /dev/null
then
    echo "jq could not be found"
    exit
fi

## Step 4: Generate the JSON file

OUTPUT_FILE=${1:-licenses_report.json}

cargo about generate --format json | jq --sort-keys --indent 4 -r > "$OUTPUT_FILE"
