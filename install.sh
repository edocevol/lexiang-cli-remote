#!/usr/bin/env bash
# lx - 乐享知识库命令行工具 安装脚本
# 用法: curl -fsSL https://raw.githubusercontent.com/tencent-lexiang/lexiang-cli/main/install.sh | bash
# 或:   curl -fsSL https://raw.githubusercontent.com/tencent-lexiang/lexiang-cli/main/install.sh | bash -s -- --dir /usr/local/bin

set -euo pipefail

REPO="tencent-lexiang/lexiang-cli"
INSTALL_DIR="${HOME}/.local/bin"
BINARY_NAME="lx"

# --- 颜色 ---
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
NC='\033[0m'

info()  { printf "${GREEN}[INFO]${NC}  %s\n" "$*"; }
warn()  { printf "${YELLOW}[WARN]${NC}  %s\n" "$*"; }
error() { printf "${RED}[ERROR]${NC} %s\n" "$*" >&2; exit 1; }

# --- 参数解析 ---
while [[ $# -gt 0 ]]; do
  case "$1" in
    --dir)
      INSTALL_DIR="$2"
      shift 2
      ;;
    --dir=*)
      INSTALL_DIR="${1#--dir=}"
      shift
      ;;
    -h|--help)
      echo "用法: install.sh [--dir <安装目录>]"
      echo ""
      echo "选项:"
      echo "  --dir <路径>  安装到指定目录 (默认: ~/.local/bin)"
      echo "  -h, --help    显示帮助信息"
      exit 0
      ;;
    *)
      error "未知参数: $1"
      ;;
  esac
done

# --- 检测平台 ---
detect_platform() {
  local os arch

  case "$(uname -s)" in
    Darwin) os="macos" ;;
    Linux)  os="linux" ;;
    MINGW*|MSYS*|CYGWIN*) os="windows" ;;
    *) error "不支持的操作系统: $(uname -s)" ;;
  esac

  case "$(uname -m)" in
    x86_64|amd64) arch="x86_64" ;;
    aarch64|arm64) arch="arm64" ;;
    *) error "不支持的架构: $(uname -m)" ;;
  esac

  # macOS 优先使用 universal binary
  if [[ "$os" == "macos" ]]; then
    echo "macos-universal"
  elif [[ "$os" == "linux" ]]; then
    echo "linux-${arch}"
  else
    echo "windows-${arch}"
  fi
}

# --- 获取最新版本 ---
get_latest_version() {
  local version
  version=$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" 2>/dev/null \
    | grep '"tag_name"' \
    | sed -E 's/.*"cli-v([^"]+)".*/\1/' \
    | head -1)

  if [[ -z "$version" ]]; then
    error "无法获取最新版本号，请检查网络或手动指定版本"
  fi

  echo "$version"
}

# --- 主流程 ---
main() {
  local platform version download_url tmp_dir archive_name

  platform=$(detect_platform)
  info "检测到平台: ${platform}"

  version=$(get_latest_version)
  info "最新版本: ${version}"

  # 构建下载 URL
  archive_name="lx-${platform}"
  if [[ "$platform" == windows-* ]]; then
    archive_name="${archive_name}.exe"
  fi

  download_url="https://github.com/${REPO}/releases/download/cli-v${version}/${archive_name}"

  info "下载地址: ${download_url}"

  # 创建临时目录
  tmp_dir=$(mktemp -d)
  trap 'rm -rf "$tmp_dir"' EXIT

  # 下载
  info "正在下载..."
  local downloaded_file="${tmp_dir}/${archive_name}"
  curl -fsSL --progress-bar -o "$downloaded_file" "$download_url" || error "下载失败"

  # macOS 签名验证
  if [[ "$(uname -s)" == "Darwin" ]]; then
    info "清除 quarantine 标记..."
    xattr -cr "$downloaded_file" 2>/dev/null || true
  fi

  # 创建安装目录
  mkdir -p "$INSTALL_DIR"

  # 安装
  local target="${INSTALL_DIR}/${BINARY_NAME}"
  if [[ "$platform" == windows-* ]]; then
    target="${target}.exe"
  fi

  mv "$downloaded_file" "$target"
  chmod +x "$target"

  info "已安装到: ${target}"

  # 检查 PATH
  if [[ ":${PATH}:" != *":${INSTALL_DIR}:"* ]]; then
    warn "${INSTALL_DIR} 不在 PATH 中，请添加:"
    echo ""
    echo "  echo 'export PATH=\"${INSTALL_DIR}:\$PATH\"' >> ~/.bashrc"
    echo "  source ~/.bashrc"
    if [[ -n "${ZSH_VERSION:-}" ]]; then
      echo ""
      echo "  # 或 Zsh:"
      echo "  echo 'export PATH=\"${INSTALL_DIR}:\$PATH\"' >> ~/.zshrc"
      echo "  source ~/.zshrc"
    fi
    echo ""
  fi

  # 验证
  if command -v lx &>/dev/null; then
    info "安装成功! $(lx version 2>/dev/null || echo "运行 lx version 查看版本")"
  else
    info "安装成功! 请重新打开终端或执行 source 使 PATH 生效，然后运行 lx version"
  fi
}

main
