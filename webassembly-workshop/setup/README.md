# 🛠️ Environment Setup

Complete this setup **before** the workshop begins. Estimated time: **15–20 minutes**.

> **Tested versions (April 2026)**  
> `wasmtime 43.0.1` · `rustc 1.94.1` · `wasm-tools 1.246.2` · `node 25.8.2` · `docker 29.1.3`

Jump to your OS: [macOS](#macos) · [Linux](#linux) · [Windows](#windows)

---

## Required Tools

### 1. Wasmtime (WASM Runtime)

The primary runtime used throughout this workshop.

#### macOS
```bash
curl https://wasmtime.dev/install.sh -sSf | bash

# Reload shell so wasmtime is on $PATH
source ~/.zshrc

# Verify — expect: wasmtime 43.x.x or newer
wasmtime --version
```

#### Linux
```bash
curl https://wasmtime.dev/install.sh -sSf | bash

# Reload shell
source ~/.bashrc

# Verify
wasmtime --version
```

#### Windows
Download and run the **MSI installer** from https://wasmtime.dev  
After install, **restart your terminal** (PowerShell or CMD) so `%USERPROFILE%\.wasmtime\bin` is on `%PATH%`.
```powershell
# Verify in PowerShell or CMD
wasmtime --version
```

---

### 2. Rust Toolchain

Required for compiling Rust source code to WASM.

#### macOS
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# Choose option 1 (default install) at the prompt
source ~/.zshrc

rustup update stable                  # ensure 1.94+
rustup target add wasm32-wasip1       # add WASM target

# Verify
rustc --version    # expect: rustc 1.94.1 or newer
cargo --version
rustup target list --installed | grep wasm32-wasip1
```

#### Linux
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.bashrc

rustup update stable
rustup target add wasm32-wasip1

# Verify
rustc --version
cargo --version
rustup target list --installed | grep wasm32-wasip1
```

#### Windows
Download and run **rustup-init.exe** from https://rustup.rs  
Choose option 1 (default install). After the installer finishes, **open a new PowerShell window**, then:
```powershell
rustup update stable
rustup target add wasm32-wasip1

# Verify
rustc --version
cargo --version
rustup target list --installed | Select-String "wasm32-wasip1"
```

> ⚠️ Windows also requires the **MSVC build tools**. If you don't have Visual Studio, the rustup installer will prompt you to install them — follow its instructions.

---

### 3. wasm-tools (WebAssembly binary toolkit)

Used in Module 1 to compile WAT files and inspect WASM binaries.  
Requires Rust to be installed first.

#### macOS & Linux
```bash
cargo install wasm-tools

# Verify — expect: wasm-tools 1.246.x or newer
wasm-tools --version
```

#### Windows
```powershell
cargo install wasm-tools

# Verify
wasm-tools --version
```

> ⏱ First install compiles from source — takes **1–2 minutes**. This is normal.

---

### 4. Node.js (for serverless / browser labs)

#### macOS
```bash
# Option A — Homebrew (recommended)
brew install node

# Option B — nvm
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh | bash
source ~/.zshrc
nvm install --lts

# Verify — expect: v22.x or v24.x LTS
node --version
npm --version
```

#### Linux
```bash
# Option A — nvm (works on all distros)
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh | bash
source ~/.bashrc
nvm install --lts

# Option B — apt (Debian/Ubuntu)
curl -fsSL https://deb.nodesource.com/setup_lts.x | sudo -E bash -
sudo apt-get install -y nodejs

# Verify
node --version
npm --version
```

#### Windows
Download the **LTS installer** from https://nodejs.org and run it.
```powershell
# Verify in a new PowerShell window
node --version
npm --version
```

---

### 5. Docker (for container comparison labs in Module 3)

#### macOS
Download and install **Docker Desktop for Mac** from https://www.docker.com/products/docker-desktop  
Open Docker Desktop and wait for the menu-bar whale icon to show **"Docker Desktop is running"**.
```bash
# Verify — expect: Docker version 27.x or newer
docker --version
docker ps    # should return an empty table, not an error
```

#### Linux
```bash
# Install Docker Engine (Debian/Ubuntu example)
sudo apt-get update
sudo apt-get install -y ca-certificates curl
sudo install -m 0755 -d /etc/apt/keyrings
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo tee /etc/apt/keyrings/docker.asc
echo "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.asc] \
  https://download.docker.com/linux/ubuntu $(. /etc/os-release && echo "$VERSION_CODENAME") stable" \
  | sudo tee /etc/apt/sources.list.d/docker.list
sudo apt-get update
sudo apt-get install -y docker-ce docker-ce-cli containerd.io

# Allow running Docker without sudo
sudo usermod -aG docker $USER
newgrp docker

# Verify
docker --version
docker ps
```
> For other distros see: https://docs.docker.com/engine/install/

#### Windows
Download and install **Docker Desktop for Windows** from https://www.docker.com/products/docker-desktop  
Requires WSL 2 (the installer will guide you if it's missing).
```powershell
# Verify in a new PowerShell window
docker --version
docker ps
```

---

### 6. Fermyon Spin (Module 4 only — install during the break before Module 4)

#### macOS & Linux
```bash
curl -fsSL https://developer.fermyon.com/downloads/install.sh | bash

# Add to PATH for this session
export PATH="$HOME/.fermyon/bin:$PATH"

# Make it permanent
echo 'export PATH="$HOME/.fermyon/bin:$PATH"' >> ~/.zshrc   # macOS
# echo 'export PATH="$HOME/.fermyon/bin:$PATH"' >> ~/.bashrc  # Linux

# Verify
spin --version
```

#### Windows
Download the latest `spin-windows-amd64.zip` from https://github.com/fermyon/spin/releases  
Extract it and move `spin.exe` to a folder that is on your `%PATH%` (e.g. `C:\tools\`).
```powershell
# Verify
spin --version
```

---

### 7. WASI SDK (optional — C/C++ only)

Not required for any core lab. Only needed if you want to compile C or C++ to WASM.

#### macOS
```bash
brew install wasi-sdk
```

#### Linux
Download the tarball for your arch from https://github.com/WebAssembly/wasi-sdk/releases  
```bash
tar xf wasi-sdk-*.tar.gz -C /opt
export WASI_SDK_PATH=/opt/wasi-sdk-*
```

#### Windows
Download the `.zip` from https://github.com/WebAssembly/wasi-sdk/releases and extract it.  
Set the `WASI_SDK_PATH` environment variable to the extracted folder.

---

## Verify Your Full Setup

#### macOS & Linux
```bash
cd /path/to/webassembly-workshop/setup
./verify-setup.sh
```

#### Windows (PowerShell)
```powershell
cd C:\path\to\webassembly-workshop\setup
bash verify-setup.sh    # requires Git Bash or WSL
```

All 8 items should show ✅. If anything is ❌, see the table below.

---

## Troubleshooting

| Symptom | macOS / Linux fix | Windows fix |
|---------|-------------------|-------------|
| `wasmtime: command not found` | `source ~/.zshrc` or new terminal; check `~/.wasmtime/bin` in `$PATH` | Open new PowerShell; check `%USERPROFILE%\.wasmtime\bin` in `%PATH%` |
| `rustup target add` fails | `rustup update stable` then retry | Same |
| `wasm32-wasip1` missing after add | `rustup toolchain install stable && rustup target add wasm32-wasip1` | Same in PowerShell |
| `cargo install wasm-tools` hangs | Normal — compiles from source (~2 min). Let it finish. | Same |
| `Permission denied: ./verify-setup.sh` | `chmod +x verify-setup.sh` | Run in Git Bash or WSL |
| Docker: `Cannot connect to the Docker daemon` | Open Docker Desktop; wait for whale icon | Open Docker Desktop; ensure WSL 2 backend is enabled |
| `spin: command not found` | `export PATH="$HOME/.fermyon/bin:$PATH"` | Add `spin.exe` folder to `%PATH%` |
| Rust on Windows: linker errors | Install **MSVC Build Tools** via Visual Studio Installer | — |

---

> ⚠️ If you're still stuck, raise it in the workshop chat or contact the instructor **before** the session starts — setup issues on the day cost everyone time.
