#!/bin/bash
# Fly.io Deployment Test Script
# Run this locally before pushing to verify build will succeed

set -e

echo "========================================"
echo "  XFHandyman Deployment Test"
echo "========================================"
echo ""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

pass() { echo -e "${GREEN}[PASS]${NC} $1"; }
fail() { echo -e "${RED}[FAIL]${NC} $1"; exit 1; }
info() { echo -e "${YELLOW}[INFO]${NC} $1"; }

# 1. Check Rust toolchain
info "Checking Rust toolchain..."
rustup show | head -5 || fail "Rust not installed"
rustup target list --installed | grep -q "wasm32-unknown-unknown" || fail "wasm32 target not installed"
pass "Rust toolchain OK"

# 2. Check cargo-leptos
info "Checking cargo-leptos..."
cargo leptos --version || fail "cargo-leptos not installed"
pass "cargo-leptos OK"

# 3. Check Node.js and npm
info "Checking Node.js..."
node --version || fail "Node.js not installed"
npm --version || fail "npm not installed"
pass "Node.js OK"

# 4. Check formatting
info "Checking code formatting..."
cargo fmt --all -- --check || fail "Code not formatted. Run: cargo fmt --all"
pass "Formatting OK"

# 5. Check clippy
info "Running clippy..."
cargo clippy -- -D warnings 2>&1 | head -20 || info "Clippy warnings (non-blocking)"
pass "Clippy check done"

# 6. Build CSS
info "Building CSS..."
npm run build:css || fail "CSS build failed"
pass "CSS build OK"

# 7. Check SSR build (fast check, not full build)
info "Checking SSR build..."
cargo check -p frontend-leptos --features ssr || fail "SSR check failed"
pass "SSR check OK"

# 8. Check API build
info "Checking API build..."
cargo check -p api || fail "API check failed"
pass "API check OK"

# 9. Full Leptos build (optional - slow)
if [ "$1" == "--full" ]; then
    info "Running full Leptos build (this takes ~5-10 min)..."
    cargo leptos build --release || fail "Leptos build failed"
    pass "Full Leptos build OK"
fi

echo ""
echo "========================================"
echo -e "${GREEN}  All checks passed!${NC}"
echo "========================================"
echo ""
echo "Ready to deploy. Run:"
echo "  git add -A && git commit -m 'your message' && git push origin main"
echo ""
echo "Or run the full build test with:"
echo "  ./test_deploy.sh --full"
