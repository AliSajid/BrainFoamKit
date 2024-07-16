# SPDX-FileCopyrightText: 2023 - 2024 Ali Sajid Imami
#
# SPDX-License-Identifier: Apache-2.0
# SPDX-License-Identifier: MIT

# This Dockerfile is used to build a Docker image for the aaprop project
# The image is built in two stages:
# 1. The first stage uses the official Rust image as the builder image
#    It builds the Rust project and creates a binary
# 2. The second stage uses the official Debian image as the base image
#    It copies the binary from the builder image and sets it as the entry point of the container

# Use the official Rust image as the builder image
# Use the 1.75 version of the Rust image since it's the MSRV (Minimum Supported Rust Version) for the aaprop project
FROM rust:1.75@sha256:87f3b2f93b82995443a1a558c234212dafe79cfdc3af956539610560369ddcd0 as builder

# Set the working directory in the builder image to /usr/src
WORKDIR /usr/src/brainfoamkit

# Copy the Cargo.toml and Cargo.lock files to the aaprop directory
COPY Cargo.toml Cargo.lock ./

# Add the actual source code to the src directory
ADD src src

# Build the Rust project with the actual source code
RUN cargo build --release --locked

# Use the official distroless image as the base image
FROM gcr.io/distroless/cc-debian12@sha256:e1065a1d58800a7294f74e67c32ec4146d09d6cbe471c1fa7ed456b2d2bf06e0

# Copy the binary from the builder image to the base image
COPY --from=builder /usr/src/brainfoamkit/target/release/bfkrun /usr/local/bin/bfkrun

# Change the user to a non-root user for security
USER 1000

# Set the binary as the entry point of the container
# When the container starts, it will execute this binary
CMD [ "/usr/local/bin/bfkrun"]
