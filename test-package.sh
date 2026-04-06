#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")" && pwd)"
cd "$ROOT"

echo "==> Building package..."
./build.sh package

echo ""
echo "==> Verifying path fixes..."
if grep -q '\./_app/' dist/test-package/index.html; then
    echo "    OK: found ./_app/ paths"
else
    echo "    FAIL: no ./_app/ paths found"
    exit 1
fi
if grep -q '"._app/' dist/test-package/index.html; then
    echo "    FAIL: found broken ._app/ paths"
    exit 1
else
    echo "    OK: no broken ._app/ paths"
fi

echo ""
echo "==> Starting flowdraft-serve from a different directory..."
echo "    (This tests that serve resolves paths relative to the executable)"
cd /tmp
exec "$ROOT/dist/test-package/flowdraft-serve-linux"
