#!/usr/bin/env bash

# SPDX-FileCopyrightText: 2023 - 2026 Ali Sajid Imami
#
# SPDX-License-Identifier: Apache-2.0
# SPDX-License-Identifier: MIT

#MISE description="Dry-run semantic-release to see whether we are doing a release"

semantic-release \
    --dry-run \
    --plugins semantic-release-export-data \
    --verify-conditions semantic-release-export-data \
    --verify-release '' \
    --generate-notes semantic-release-export-data \
    --prepare '' \
    --publish '' \
    --success '' \
    --fail ''
