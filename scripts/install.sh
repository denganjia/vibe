#!/bin/bash
set -e

# Configuration
REPO="anjia/vibe-cli"
INSTALL_DIR="$HOME/.local/bin"
BINARY_NAME="vibe"

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}==> Installing Vibe CLI...${NC}"

# Detect OS and Architecture
OS="$(uname -s)"
ARCH="$(uname -m)"

case "${OS}" in
    Darwin)
        case "${ARCH}" in
            x86_64)  SUFFIX="macos-x64.tar.gz" ;;
            arm64)   SUFFIX="macos-arm64.tar.gz" ;;
            *) echo "Unsupported architecture: ${ARCH}"; exit 1 ;;
        esac
        ;;
    Linux)
        SUFFIX="linux-x64.tar.gz"
        ;;
    *)
        echo "Unsupported OS: ${OS}"
        exit 1
        ;;
esac

# Get latest release tag
LATEST_TAG=$(curl -s "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

if [ -z "${LATEST_TAG}" ]; then
    echo "Error: Could not find latest release for ${REPO}."
    exit 1
fi

DOWNLOAD_URL="https://github.com/$(REPO)/releases/download/${LATEST_TAG}/vibe-${SUFFIX}"

# Create install directory
mkdir -p "${INSTALL_DIR}"

# Download and Extract
echo -e "${BLUE}==> Downloading ${LATEST_TAG} for ${OS} ${ARCH}...${NC}"
curl -L "${DOWNLOAD_URL}" | tar xz -C "${INSTALL_DIR}"

chmod +x "${INSTALL_DIR}/${BINARY_NAME}"

echo -e "${GREEN}==> Vibe CLI installed successfully to ${INSTALL_DIR}/${BINARY_NAME}${NC}"

# PATH hint
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo -e "${BLUE}==> Tip: Add ${INSTALL_DIR} to your PATH to use 'vibe' everywhere.${NC}"
    echo "    Add this line to your .zshrc or .bashrc:"
    echo "    export PATH=\"\$PATH:${INSTALL_DIR}\""
fi

echo -e "${GREEN}==> Done! Try running 'vibe check'${NC}"
