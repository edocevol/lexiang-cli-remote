#!/usr/bin/env bash
#
# 本地多架构构建脚本
# 用法:
#   ./scripts/build-all.sh           # 构建所有目标
#   ./scripts/build-all.sh macos     # 仅构建 macOS (Intel + ARM + Universal)
#   ./scripts/build-all.sh linux     # 仅构建 Linux (需要 cross)
#   ./scripts/build-all.sh windows   # 仅构建 Windows (需要 cross)
#

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
DIST_DIR="$PROJECT_DIR/dist"

# 颜色输出
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

info() { echo -e "${GREEN}[INFO]${NC} $*"; }
warn() { echo -e "${YELLOW}[WARN]${NC} $*"; }
error() { echo -e "${RED}[ERROR]${NC} $*" >&2; }

# 检查依赖
check_deps() {
    if ! command -v cargo &>/dev/null; then
        error "cargo not found. Please install Rust: https://rustup.rs"
        exit 1
    fi
}

# 检查 cross 是否安装
check_cross() {
    if ! command -v cross &>/dev/null; then
        warn "cross not found. Installing..."
        cargo install cross --git https://github.com/cross-rs/cross
    fi
}

# 安装 Rust target
ensure_target() {
    local target=$1
    if ! rustup target list --installed | grep -q "$target"; then
        info "Installing target: $target"
        rustup target add "$target"
    fi
}

# 构建单个目标
build_target() {
    local target=$1
    local use_cross=${2:-false}
    local output_name=$3

    info "Building for $target..."

    cd "$PROJECT_DIR"

    if [ "$use_cross" = "true" ]; then
        check_cross
        cross build --release --target "$target"
    else
        ensure_target "$target"
        cargo build --release --target "$target"
    fi

    # 复制产物
    mkdir -p "$DIST_DIR"
    local src="$PROJECT_DIR/target/$target/release/lx"
    if [[ "$target" == *"windows"* ]]; then
        src="${src}.exe"
        cp "$src" "$DIST_DIR/${output_name}.exe"
    else
        cp "$src" "$DIST_DIR/$output_name"
        chmod +x "$DIST_DIR/$output_name"
    fi

    info "Built: $DIST_DIR/$output_name"
}

# 构建 macOS Universal Binary
build_macos_universal() {
    info "Creating macOS Universal Binary..."

    local x86="$DIST_DIR/lx-macos-x86_64"
    local arm="$DIST_DIR/lx-macos-arm64"
    local universal="$DIST_DIR/lx-macos-universal"

    if [[ ! -f "$x86" ]] || [[ ! -f "$arm" ]]; then
        error "Both x86_64 and arm64 builds required for universal binary"
        return 1
    fi

    lipo -create "$x86" "$arm" -output "$universal"
    chmod +x "$universal"

    info "Built: $universal"
}

# 构建 macOS 目标
build_macos() {
    build_target "x86_64-apple-darwin" "false" "lx-macos-x86_64"
    build_target "aarch64-apple-darwin" "false" "lx-macos-arm64"
    build_macos_universal
}

# 构建 Linux 目标
build_linux() {
    # x86_64 可以在 macOS 上用 cross 构建
    build_target "x86_64-unknown-linux-gnu" "true" "lx-linux-x86_64"
    build_target "aarch64-unknown-linux-gnu" "true" "lx-linux-arm64"
}

# 构建 Windows 目标
build_windows() {
    build_target "x86_64-pc-windows-gnu" "true" "lx-windows-x86_64"
}

# 生成校验和
generate_checksums() {
    info "Generating checksums..."
    cd "$DIST_DIR"

    if command -v sha256sum &>/dev/null; then
        sha256sum lx-* > SHA256SUMS.txt
    elif command -v shasum &>/dev/null; then
        shasum -a 256 lx-* > SHA256SUMS.txt
    else
        warn "sha256sum/shasum not found, skipping checksums"
        return
    fi

    info "Generated: $DIST_DIR/SHA256SUMS.txt"
}

# 清理
clean() {
    info "Cleaning dist directory..."
    rm -rf "$DIST_DIR"
}

# 显示帮助
show_help() {
    cat <<EOF
Usage: $(basename "$0") [COMMAND]

Commands:
  all       Build all targets (default)
  macos     Build macOS targets (x86_64 + arm64 + universal)
  linux     Build Linux targets (requires cross/Docker)
  windows   Build Windows targets (requires cross/Docker)
  clean     Remove dist directory
  help      Show this help

Examples:
  $(basename "$0")           # Build all
  $(basename "$0") macos     # Build macOS only
  $(basename "$0") clean     # Clean dist
EOF
}

# 主入口
main() {
    check_deps

    local cmd="${1:-all}"

    case "$cmd" in
        all)
            clean
            build_macos
            build_linux
            build_windows
            generate_checksums
            info "All builds complete! Check $DIST_DIR"
            ;;
        macos)
            build_macos
            generate_checksums
            ;;
        linux)
            build_linux
            generate_checksums
            ;;
        windows)
            build_windows
            generate_checksums
            ;;
        clean)
            clean
            ;;
        help|--help|-h)
            show_help
            ;;
        *)
            error "Unknown command: $cmd"
            show_help
            exit 1
            ;;
    esac
}

main "$@"
