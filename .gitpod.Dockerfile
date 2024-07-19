# SPDX-FileCopyrightText: 2023 - 2024 Ali Sajid Imami
#
# SPDX-License-Identifier: Apache-2.0
# SPDX-License-Identifier: MIT

FROM gitpod/workspace-full:2023-04-14-07-10-23@sha256:bfff4e64d6c72d41c979b7c7580253a99dcafbfc527d9dd3b7e2f08f48ab8490

# Install your tools here

RUN brew install \
    tmux \
    bat \
    tldr \
    ripgrep \
    exa \
    fzf \
    gitleaks \
    markdownlint-cli \
    direnv \
    commitizen \
    tree \
    yq \
    yamllint \
    pre-commit
