# cargo install just
# 默认：列出所有可用命令
default:
    @just --list

# 构建项目
build:
    cargo build

# install
install:
    cargo install --force --path .

# 构建 release 版本
release:
    cargo build --release

# 运行
run *ARGS:
    cargo run -- {{ARGS}}

# 检查编译
check:
    cargo check

# 格式化代码
fmt:
    cargo fmt --all

# 格式化检查（不修改）
fmt-check:
    cargo fmt --all -- --check

# Clippy 检查
lint:
    cargo clippy --all-targets --all-features -- -D warnings

# Clippy 自动修复
lint-fix:
    cargo clippy --all-targets --all-features --fix --allow-dirty --allow-staged -- -D warnings

# Markdown lint
markdown-lint:
    npx --yes markdownlint-cli2 --fix

# 运行测试
test:
    cargo test

# 格式化 + lint + 编译检查（提交前手动跑一遍）
pre-commit: fmt lint-fix check
    @echo "✅ All checks passed!"

# 安装 git hooks（首次 clone 后跑一次）
setup:
    cargo clean -p cargo-husky
    cargo test --no-run
    @echo "✅ Git hooks installed!"

# 清理构建产物
clean:
    cargo clean
