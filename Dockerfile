# SPDX-FileCopyrightText: 2023 - 2026 Ali Sajid Imami
#
# SPDX-License-Identifier: Apache-2.0
# SPDX-License-Identifier: MIT

# This Containerfile/Dockerfile is used to build a container image for BrainFoamKit
# Compatible with both Docker and Podman container engines
# The image is built in two phases:
# 1. The Preparation phase starts from the official Rust image to prepare
#    the environment for the final build using cargo-chef for dependency caching.
# 2. The Build phase finally builds the binary of the BrainFoamKit project.
#    It copies the binary from the builder image to a distroless image and
#    sets it as the entry point of the container
# All in all, this results in the total process of building the container image
# consisting of four stages:
# 1. The `chef` stage installs the `cargo-chef` tool in the official Rust image
# 2. The `planner` stage examines the project and builds a minimal recipe for
#    the project.
# 3. The `builder` stage uses the recipe from the `planner` stage to build the
#    dependencies and the final binary.
# 4. The `distroless` stage copies the binary from the `builder` stage to distroless
#    image and readies it for production.

# ===================================================================================
# Preparation phase
# ===================================================================================

# -----------------------------------------------------------------------------------
# chef stage
# -----------------------------------------------------------------------------------

# Use the official Rust image with cargo-chef pre-installed
# Use the 1.86.0 version of the Rust image (latest) for BrainFoamKit
FROM lukemathwalker/cargo-chef:0.1.71-rust-1.86.0 AS chef

WORKDIR /app

ENV CARGO_TARGET_DIR=/app/target

# -----------------------------------------------------------------------------------
# planner stage
# -----------------------------------------------------------------------------------

# Use the previous stage as the base image
FROM chef AS planner

# Copy the project files to the image
COPY . .

# Use `cargo-chef prepare` to generate a minimal recipe for the project
RUN cargo chef prepare --recipe-path recipe.json

# ===================================================================================
# Build phase
# ===================================================================================

# -----------------------------------------------------------------------------------
# builder stage
# -----------------------------------------------------------------------------------

# Use the previous stage as the base image
FROM chef AS builder

ARG SOURCE_DATE_EPOCH
ENV SOURCE_DATE_EPOCH=${SOURCE_DATE_EPOCH}
RUN echo "SOURCE_DATE_EPOCH raw: $SOURCE_DATE_EPOCH"
RUN date -u --date="$SOURCE_DATE_EPOCH" '+%Y-%m-%d %H:%M:%S UTC'

ENV RUSTFLAGS="--remap-path-prefix=/app=."

# Copy the `cargo-chef` recipe from the `planner` stage to the current image
COPY --from=planner /app/recipe.json recipe.json

# Use `cargo-chef cook` to build the dependencies of the project
RUN cargo chef cook --release --locked --recipe-path recipe.json

# Copy the project files to the image
COPY . .

# Build the Rust project with the actual source code
RUN cargo build --release --locked

# -----------------------------------------------------------------------------------
# distroless stage
# -----------------------------------------------------------------------------------

# Use the official distroless image as the base image
FROM gcr.io/distroless/cc-debian12@sha256:e1065a1d58800a7294f74e67c32ec4146d09d6cbe471c1fa7ed456b2d2bf06e0 AS distroless

# Copy the binary from the builder image to the base image
COPY --from=builder /app/target/release/bfkrun /usr/local/bin/bfkrun

# Set the binary as the entry point of the container
# When the container starts, it will execute this binary
ENTRYPOINT [ "/usr/local/bin/bfkrun" ]
