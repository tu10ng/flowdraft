# Flowdraft Web Playground

This package contains the Flowdraft web playground as a static website.

## Quick Start

### Linux
```bash
./flowdraft-serve-linux
```

### Windows
```cmd
flowdraft-serve-windows.exe
```

The server will automatically open your browser to the playground.

## Manual Setup

If you prefer to use your own web server, you can serve the files using any static file server:

```bash
# Python
python -m http.server 8000

# Node.js
npx serve

# PHP
php -S localhost:8000
```

Then open http://localhost:8000 in your browser.

## Note

Due to browser security restrictions, you cannot open `index.html` directly with `file://` protocol. You must use a web server (like the included `flowdraft-serve` binaries).
