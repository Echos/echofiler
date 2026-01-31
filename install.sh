#!/bin/bash
# Installation script for echofiler

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Default values
PREFIX="${PREFIX:-$HOME/.local}"
BINDIR="$PREFIX/bin"
CONFIGDIR="$HOME/.config/echofiler"
FEATURES="${FEATURES:-archive,preview,plugin}"
BUILD_TYPE="${BUILD_TYPE:-release}"

# Helper functions
print_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if cargo is installed
if ! command -v cargo &> /dev/null; then
    print_error "Cargo is not installed. Please install Rust first:"
    echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

# Build
print_info "Building echofiler ($BUILD_TYPE)..."
if [ "$BUILD_TYPE" = "release" ]; then
    cargo build --release --features "$FEATURES"
    BINARY="target/release/echofiler"
else
    cargo build --features "$FEATURES"
    BINARY="target/debug/echofiler"
fi

if [ ! -f "$BINARY" ]; then
    print_error "Build failed. Binary not found: $BINARY"
    exit 1
fi

print_info "Build complete: $BINARY"

# Install binary
print_info "Installing binary..."
mkdir -p "$BINDIR"
install -m 755 "$BINARY" "$BINDIR/echofiler"
print_info "Binary installed: $BINDIR/echofiler"

# Install config files
print_info "Installing config files..."
mkdir -p "$CONFIGDIR"
mkdir -p "$CONFIGDIR/plugins"

for config_file in echofiler.toml keymap.toml theme.toml opener.toml; do
    if [ -f "$CONFIGDIR/$config_file" ]; then
        print_warn "Config already exists: $CONFIGDIR/$config_file (skipped)"
    else
        install -m 644 "config/default/$config_file" "$CONFIGDIR/$config_file"
        print_info "Config installed: $CONFIGDIR/$config_file"
    fi
done

# Check PATH
print_info "Installation complete!"
echo ""
echo "Binary: $BINDIR/echofiler"
echo "Config: $CONFIGDIR/"
echo ""

if [[ ":$PATH:" != *":$BINDIR:"* ]]; then
    print_warn "$BINDIR is not in your PATH."
    echo "Add this line to your ~/.bashrc or ~/.zshrc:"
    echo ""
    echo "  export PATH=\"$BINDIR:\$PATH\""
    echo ""
fi

print_info "To run echofiler, type: echofiler"
