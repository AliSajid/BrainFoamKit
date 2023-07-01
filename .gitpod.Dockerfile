FROM gitpod/workspace-full:2023-04-14-07-10-23

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
    kubectl \
    kubectx \
    k9s \
    infracost \
    tree \
    yq \
    yamllint \
    helm \
    pre-commit
