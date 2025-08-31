#!/bin/bash

set -euo pipefail

REPO_URL="https://github.com/nixuuu/image-optimizer-rs"
BINARY_NAME="image-optimizer-rs"
INSTALL_DIR="$HOME/.local/bin"

print_header() {
    echo "════════════════════════════════════════════════════════════════"
    echo "  Image Optimizer RS - Installation Script"
    echo "  CLI tool for optimizing JPEG, PNG, and WebP images"
    echo "════════════════════════════════════════════════════════════════"
    echo
}

print_success() {
    echo "✅ $1"
}

print_error() {
    echo "❌ Error: $1" >&2
}

print_info() {
    echo "ℹ️  $1"
}

detect_platform() {
    local os=$(uname -s | tr '[:upper:]' '[:lower:]')
    local arch=$(uname -m)
    
    case "$os" in
        linux*)
            OS="linux"
            ;;
        darwin*)
            OS="macos"
            ;;
        *)
            print_error "Unsupported operating system: $os"
            echo "This installer supports Linux and macOS only."
            exit 1
            ;;
    esac
    
    case "$arch" in
        x86_64|amd64)
            ARCH="x86_64"
            ;;
        arm64|aarch64)
            ARCH="aarch64"
            ;;
        *)
            print_error "Unsupported architecture: $arch"
            echo "This installer supports x86_64 and aarch64 architectures only."
            exit 1
            ;;
    esac
    
    print_info "Detected platform: $OS-$ARCH"
}

check_dependencies() {
    print_info "Checking dependencies..."
    
    if ! command -v curl >/dev/null 2>&1; then
        print_error "curl is required but not installed"
        exit 1
    fi
    
    if ! command -v tar >/dev/null 2>&1; then
        print_error "tar is required but not installed"
        exit 1
    fi
    
    if command -v cargo >/dev/null 2>&1; then
        print_info "Rust/Cargo detected - will build from source"
        INSTALL_METHOD="source"
    else
        print_info "Rust/Cargo not found - will try to download prebuilt binary"
        INSTALL_METHOD="binary"
    fi
}

install_from_binary() {
    print_info "Attempting to download prebuilt binary..."
    
    local release_url
    local download_url
    local temp_dir
    
    temp_dir=$(mktemp -d)
    trap "rm -rf $temp_dir" EXIT
    
    release_url="https://api.github.com/repos/nixuuu/image-optimizer-rs/releases/latest"
    
    print_info "Fetching latest release information..."
    if ! curl -sL "$release_url" > "$temp_dir/release.json"; then
        print_error "Failed to fetch release information"
        return 1
    fi
    
    local target
    case "$OS-$ARCH" in
        linux-x86_64)
            target="x86_64-unknown-linux-gnu"
            ;;
        linux-aarch64)
            target="aarch64-unknown-linux-gnu"
            ;;
        macos-x86_64)
            target="x86_64-apple-darwin"
            ;;
        macos-aarch64)
            target="aarch64-apple-darwin"
            ;;
        *)
            print_error "Unsupported platform: $OS-$ARCH"
            return 1
            ;;
    esac
    
    download_url=$(grep -o "\"browser_download_url\".*image-optimizer-rs-${target}\"" "$temp_dir/release.json" | cut -d'"' -f4 | head -n1 || true)
    
    if [ -z "$download_url" ]; then
        print_error "No prebuilt binary found for $target ($OS-$ARCH)"
        print_info "Available binaries:"
        grep -o "\"name\":\"image-optimizer-rs-[^\"]*\"" "$temp_dir/release.json" | cut -d'"' -f4 || true
        return 1
    fi
    
    print_info "Downloading $download_url..."
    if ! curl -sL "$download_url" -o "$temp_dir/image-optimizer-rs-${target}"; then
        print_error "Failed to download binary"
        return 1
    fi
    
    local binary_path="$temp_dir/image-optimizer-rs-${target}"
    
    if [ ! -f "$binary_path" ]; then
        print_error "Binary not found after download"
        return 1
    fi
    
    mkdir -p "$INSTALL_DIR"
    cp "$binary_path" "$INSTALL_DIR/$BINARY_NAME"
    chmod +x "$INSTALL_DIR/$BINARY_NAME"
    
    print_success "Binary installed to $INSTALL_DIR/$BINARY_NAME"
}

install_from_source() {
    print_info "Building from source..."
    
    local temp_dir
    temp_dir=$(mktemp -d)
    trap "rm -rf $temp_dir" EXIT
    
    print_info "Cloning repository..."
    if ! git clone "$REPO_URL" "$temp_dir/image-optimizer-rs"; then
        print_error "Failed to clone repository"
        exit 1
    fi
    
    cd "$temp_dir/image-optimizer-rs"
    
    print_info "Building release binary..."
    if ! cargo build --release; then
        print_error "Failed to build from source"
        exit 1
    fi
    
    mkdir -p "$INSTALL_DIR"
    cp "target/release/$BINARY_NAME" "$INSTALL_DIR/$BINARY_NAME"
    chmod +x "$INSTALL_DIR/$BINARY_NAME"
    
    print_success "Built and installed to $INSTALL_DIR/$BINARY_NAME"
}

setup_path() {
    local shell_config
    local path_export="export PATH=\"\$PATH:$INSTALL_DIR\""
    
    case "$SHELL" in
        */zsh)
            shell_config="$HOME/.zshrc"
            ;;
        */bash)
            shell_config="$HOME/.bashrc"
            if [ "$OS" = "macos" ] && [ -f "$HOME/.bash_profile" ]; then
                shell_config="$HOME/.bash_profile"
            fi
            ;;
        */fish)
            mkdir -p "$HOME/.config/fish"
            shell_config="$HOME/.config/fish/config.fish"
            path_export="set -gx PATH \$PATH $INSTALL_DIR"
            ;;
        *)
            shell_config="$HOME/.profile"
            ;;
    esac
    
    if [ -f "$shell_config" ] && grep -q "$INSTALL_DIR" "$shell_config"; then
        print_info "PATH already configured in $shell_config"
    else
        print_info "Adding $INSTALL_DIR to PATH in $shell_config"
        echo "" >> "$shell_config"
        echo "# Added by image-optimizer-rs installer" >> "$shell_config"
        echo "$path_export" >> "$shell_config"
        print_success "PATH updated in $shell_config"
    fi
}

verify_installation() {
    if [ -x "$INSTALL_DIR/$BINARY_NAME" ]; then
        print_success "Installation verified"
        echo
        echo "🎉 Image Optimizer RS has been successfully installed!"
        echo
        echo "Usage:"
        echo "  $BINARY_NAME --help                    # Show help"
        echo "  $BINARY_NAME -i ./images -r            # Optimize images recursively"
        echo "  $BINARY_NAME -i input -o output --quality 90  # Optimize with custom quality"
        echo
        echo "To use the tool immediately, either:"
        echo "  1. Restart your terminal, or"
        echo "  2. Run: source ~/.$(basename $SHELL)rc"
        echo "  3. Use the full path: $INSTALL_DIR/$BINARY_NAME"
        echo
    else
        print_error "Installation verification failed"
        exit 1
    fi
}

main() {
    print_header
    detect_platform
    check_dependencies
    
    echo
    print_info "Installing Image Optimizer RS..."
    print_info "Install method: $INSTALL_METHOD"
    
    if [ "$INSTALL_METHOD" = "binary" ]; then
        if ! install_from_binary; then
            print_info "Binary installation failed, falling back to source build..."
            if ! command -v git >/dev/null 2>&1; then
                print_error "git is required for source installation but not found"
                exit 1
            fi
            if ! command -v cargo >/dev/null 2>&1; then
                print_error "Rust/Cargo is required for source installation but not found"
                echo "Please install Rust from https://rustup.rs/ and try again"
                exit 1
            fi
            install_from_source
        fi
    else
        if ! command -v git >/dev/null 2>&1; then
            print_error "git is required but not installed"
            exit 1
        fi
        install_from_source
    fi
    
    setup_path
    verify_installation
}

if [ "${BASH_SOURCE[0]}" = "${0}" ]; then
    main "$@"
fi