#!/bin/sh
set -e

# Hubbit installer script
# Usage: curl -sSfL https://raw.githubusercontent.com/daveio/hubbit/main/install.sh | sh

REPO="daveio/hubbit"
BINARY="hubbit"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"

# Detect OS and architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case "$OS" in
darwin) OS="Darwin" ;;
linux) OS="Linux" ;;
*)
	echo "Unsupported OS: $OS"
	exit 1
	;;
esac

case "$ARCH" in
x86_64) ARCH="x86_64" ;;
amd64) ARCH="x86_64" ;;
aarch64) ARCH="arm64" ;;
arm64) ARCH="arm64" ;;
*)
	echo "Unsupported architecture: $ARCH"
	exit 1
	;;
esac

# Get latest release
echo "Fetching latest release..."
LATEST_RELEASE=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

if [ -z "$LATEST_RELEASE" ]; then
	echo "Failed to fetch latest release"
	exit 1
fi

# Construct download URL
FILENAME="${BINARY}_${OS}_${ARCH}.tar.gz"
DOWNLOAD_URL="https://github.com/$REPO/releases/download/$LATEST_RELEASE/$FILENAME"

echo "Downloading $BINARY $LATEST_RELEASE for $OS/$ARCH..."

# Create temp directory
TMP_DIR=$(mktemp -d)
trap "rm -rf $TMP_DIR" EXIT

# Download and extract
curl -sL "$DOWNLOAD_URL" | tar xz -C "$TMP_DIR"

# Create install directory if needed
mkdir -p "$INSTALL_DIR"

# Install binary
mv "$TMP_DIR/$BINARY" "$INSTALL_DIR/"
chmod +x "$INSTALL_DIR/$BINARY"

echo "Successfully installed $BINARY to $INSTALL_DIR/$BINARY"
echo ""
echo "Make sure $INSTALL_DIR is in your PATH:"
echo "  export PATH=\"\$PATH:$INSTALL_DIR\""
