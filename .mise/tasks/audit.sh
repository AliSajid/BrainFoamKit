#!/usr/bin/env bash

# SPDX-FileCopyrightText: 2023 - 2026 Ali Sajid Imami
#
# SPDX-License-Identifier: Apache-2.0
# SPDX-License-Identifier: MIT

#MISE description="Audit the crate's dependencies for security issues"
#MISE alias="a"
#MISE sources=["Cargo.toml", "Cargo.lock"]
#MISE outputs=["Cargo.lock"]

cargo audit
