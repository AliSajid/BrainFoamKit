#!/usr/bin/env bash

# SPDX-FileCopyrightText: 2023 - 2026 Ali Sajid Imami
#
# SPDX-License-Identifier: Apache-2.0
# SPDX-License-Identifier: MIT

#MISE description="Build and Deploy the documentation website to Vercel"
#MISE alias="docs"

# Exit immediately if a command exits with a non-zero status
set -euo pipefail

# Ensure required environment variables are set
if [[ -z "$PROJECT_ENVIRONMENT" || -z "$VERCEL_TOKEN" ]]; then
    echo "Error: PROJECT_ENVIRONMENT and VERCEL_TOKEN must be set."
    exit 1
fi

# Step 1: Populate Preview Variables
echo "Populating Vercel environment variables for environment: $PROJECT_ENVIRONMENT"
vercel pull --yes --environment="$PROJECT_ENVIRONMENT" --token="$VERCEL_TOKEN"

# Step 2: Build the Book
echo "Building the book..."
mdbook build guide

# Common options for Vercel build and deploy commands
BUILD_OPTIONS="--cwd guide/book --token=$VERCEL_TOKEN --yes --debug"
DEPLOY_OPTIONS="--prebuilt $BUILD_OPTIONS --yes --debug"

if [[ "$PROJECT_ENVIRONMENT" == "preview" ]]; then
    echo "Building and deploying for Vercel (preview)..."
    vercel build "$BUILD_OPTIONS"
    vercel deploy "$DEPLOY_OPTIONS" >deployment_url
elif [[ "$PROJECT_ENVIRONMENT" == "production" ]]; then
    echo "Building and deploying for Vercel (production)..."
    vercel build "$BUILD_OPTIONS"
    vercel deploy "$DEPLOY_OPTIONS" --prod >deployment_url
else
    echo "Error: Unknown PROJECT_ENVIRONMENT value: $PROJECT_ENVIRONMENT"
    exit 1
fi

# Output deployment URL
echo "Deployment completed. Deployment URL: "
cat deployment_url
