#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")" && pwd)"
cd "$ROOT"

# Ensure cargo-installed binaries are in PATH
export PATH="$HOME/.cargo/bin:$PATH"

build_cli() {
    echo "==> Building CLI..."
    cargo build --release
    echo "    Done: target/release/flowdraft"
}

build_wasm() {
    echo "==> Building WASM..."
    wasm-pack build --target web --features wasm --no-default-features
    mkdir -p web/src/lib/pkg
    cp -r pkg/* web/src/lib/pkg/
    echo "    Done: web/src/lib/pkg/"
}

build_web() {
    build_wasm
    echo "==> Building Web..."
    cd web
    pnpm install --frozen-lockfile
    pnpm exec vite build
    cd "$ROOT"
    echo "    Done: web/build/"
}

run_tests() {
    echo "==> Running tests..."
    cargo test
}

case "${1:-all}" in
    cli)
        build_cli
        ;;
    wasm)
        build_wasm
        ;;
    web)
        build_web
        ;;
    dev)
        build_wasm
        echo "==> Starting dev server..."
        cd web
        pnpm install --frozen-lockfile
        exec pnpm exec vite dev --open
        ;;
    test)
        run_tests
        ;;
    all)
        run_tests
        build_cli
        build_web
        ;;
    *)
        echo "Usage: $0 {cli|wasm|web|dev|test|all}"
        exit 1
        ;;
esac
