# 🛠️ Environment Setup

Complete this setup **before** the workshop begins. Estimated time: **15–20 minutes**.

---

## Required Tools

### 1. Wasmtime (WASM Runtime)

Wasmtime is the primary runtime used in this workshop.

```bash
# macOS / Linux
curl https://wasmtime.dev/install.sh -sSf | bash

# Verify installation
wasmtime --version
```

**Windows:** Download the installer from https://wasmtime.dev

---

### 2. Rust Toolchain (for compiling to WASM)

```bash
# Install Rust via rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add the WASM compilation target
rustup target add wasm32-wasip1

# Verify
rustc --version
cargo --version
```

---

### 3. WASI SDK (for C/C++ compilation – optional)

```bash
# macOS via Homebrew
brew install wasi-sdk

# Or download from GitHub releases:
# https://github.com/WebAssembly/wasi-sdk/releases
```

---

### 4. wat2wasm / wasm-tools (WebAssembly Text Format tools)

```bash
# Install wasm-tools via cargo
cargo install wasm-tools

# Verify
wasm-tools --version
```

---

### 5. Node.js (for browser/serverless labs)

```bash
# Download from https://nodejs.org (LTS recommended)
# Or via nvm:
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh | bash
nvm install --lts

# Verify
node --version
npm --version
```

---

### 6. Docker (for container comparison labs)

```bash
# Install Docker Desktop: https://www.docker.com/products/docker-desktop

# Verify
docker --version
```

---

## Verify Your Full Setup

Run the following checklist script to confirm everything is ready:

```bash
cd /path/to/webassembly-workshop/setup
chmod +x verify-setup.sh
./verify-setup.sh
```

---

## Troubleshooting

| Issue | Fix |
|-------|-----|
| `wasmtime: command not found` | Restart your terminal after install, or add `~/.wasmtime/bin` to `$PATH` |
| `rustup target add` fails | Run `rustup update` first |
| Permission denied on `.sh` script | Run `chmod +x <script>.sh` |
| Docker daemon not running | Start Docker Desktop |

---

> ⚠️ If you encounter issues, raise them in the workshop chat or contact the instructor before the session starts.
