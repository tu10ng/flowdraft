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
    pnpm run build
    cd "$ROOT"
    echo "    Done: web/build/"
}

run_tests() {
    echo "==> Running tests..."
    cargo test
}

test_package() {
    echo "==> Testing web package (simulating CI)..."

    # Build web
    build_web

    # Build serve binaries
    echo "==> Building serve binaries..."
    cargo build --release --features serve --bin flowdraft-serve

    # Create test package
    echo "==> Creating test package..."
    rm -rf dist/test-package
    mkdir -p dist/test-package
    cp -r web/build/* dist/test-package/
    cp target/release/flowdraft-serve dist/test-package/flowdraft-serve-linux
    cp web/README-SERVE.md dist/test-package/README.md
    chmod +x dist/test-package/flowdraft-serve-linux

    # Create zip
    cd dist/test-package
    zip -r ../flowdraft-web-test.zip .
    cd "$ROOT"

    echo "    Done: dist/flowdraft-web-test.zip"
    echo ""
    echo "To test:"
    echo "  cd dist/test-package && ./flowdraft-serve-linux"
    echo ""
    echo "Or extract the zip:"
    echo "  unzip dist/flowdraft-web-test.zip -d /tmp/flowdraft-test"
    echo "  cd /tmp/flowdraft-test && ./flowdraft-serve-linux"
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
    package)
        test_package
        ;;
    all)
        run_tests
        build_cli
        build_web
        ;;
    *)
        echo "Usage: $0 {cli|wasm|web|dev|test|package|all}"
        exit 1
        ;;
esac
