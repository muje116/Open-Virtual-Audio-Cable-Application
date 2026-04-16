# 📦 Installation Guide

> Complete guide to installing and running the Virtual Audio Cable application

---

## 📋 Prerequisites

### Required Tools

| Tool | Version | Purpose |
|------|---------|---------|
| **Rust** | v1.70+ | Backend compilation |
| **Node.js** | v18+ | Frontend runtime |
| **pnpm** | Latest | Package manager |

### Installing Rust

```powershell
# Windows
winget install Rustlang.Rustup

# macOS/Linux
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

> **Note**: After installation, restart your terminal for PATH changes to take effect.

### Verifying Installation

```bash
# Check Rust installation
cargo --version
rustc --version

# Check Node.js installation
node --version

# Check pnpm installation
pnpm --version
```

---

## 🖥️ Platform-Specific Dependencies

### Windows

Install Visual Studio Build Tools with C++ support:

```powershell
winget install Microsoft.VisualStudio.2022.BuildTools `
  --override "--wait --passive --add Microsoft.VisualStudio.Workload.VCTools;includeRecommended"
```

**Alternative**: Download from [Visual Studio](https://visualstudio.microsoft.com/downloads/) and select "Desktop development with C++"

### macOS

Install Xcode Command Line Tools:

```bash
xcode-select --install
```

**Verification**:
```bash
xcode-select -p
# Should output: /Library/Developer/CommandLineTools
```

### Linux

Install required dependencies:

```bash
# Ubuntu/Debian
sudo apt update
sudo apt install libwebkit2gtk-4.1-dev build-essential curl \
  wget file libxdo-dev libssl-dev libayatana-appindicator3-dev \
  librsvg2-dev

# Fedora
sudo dnf install webkit2gtk4.1-devel openssl-devel curl \
  wget file libappindicator-gtk3-devel librsvg2-devel

# Arch Linux
sudo pacman -S webkit2gtk-4.1 base-devel curl wget \
  libappindicator-gtk3 librsvg
```

---

## 🚀 Installation Steps

### 1. Navigate to Project Directory

```powershell
cd d:/Projects/VAC
```

### 2. Install Node Dependencies

```bash
pnpm install
```

This installs:
- React and related libraries
- Tauri CLI
- TypeScript and build tools
- Tailwind CSS and dependencies

### 3. Verify Rust Installation

If Rust is not installed:

```powershell
# Download rustup-init.exe from https://rustup.rs/
# Run the installer
rustup-init.exe
```

### 4. Install Platform-Specific Dependencies

Follow the platform-specific instructions above.

---

## ▶️ Running the Application

### Development Mode

Start the application with hot-reload enabled:

```bash
pnpm tauri:dev
```

This will:
- ✅ Start the React dev server (Vite)
- ✅ Compile the Rust backend in debug mode
- ✅ Launch the Tauri application window
- ✅ Enable hot-reload for both frontend and backend

### Production Build

Build the application for your platform:

```bash
pnpm tauri:build
```

**Output Location**: `src-tauri/target/release/bundle/`

**Artifacts**:
- **Windows**: `.msi` installer
- **macOS**: `.dmg` disk image
- **Linux**: `.AppImage`, `.deb`, or `.rpm` package

---

## 🔧 Troubleshooting

### Rust Not Found

**Error**: `cargo: command not found`

**Solution**:
```powershell
# Install Rust from https://rustup.rs/
# After installation, restart your terminal
```

### Build Errors on Windows

**Error**: Linker errors or missing MSVC

**Solutions**:
1. Ensure Visual Studio Build Tools are installed
2. Run from "x64 Native Tools Command Prompt for VS 2022"
3. Set environment variable:
   ```powershell
   $env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"
   ```

### Build Errors on macOS

**Error**: `xcrun: error: invalid active developer path`

**Solution**:
```bash
xcode-select --install
# Accept the license agreement
sudo xcodebuild -license
```

### Build Errors on Linux

**Error**: Missing webkit2gtk or other dependencies

**Solution**:
```bash
# Install all required dependencies (see Platform-Specific Dependencies above)
# Ensure you have the latest versions:
sudo apt update && sudo apt upgrade
```

### Permission Denied (Linux)

**Error**: `EACCES` when running the application

**Solution**:
```bash
# Add your user to the audio group
sudo usermod -a -G audio $USER
# Log out and log back in for changes to take effect
```

---

## 📝 Next Steps

After successful installation:

1. ✅ **Run the application**: `pnpm tauri:dev`
2. 🎧 **Install virtual audio driver** (see [DRIVER_INSTALL.md](DRIVER_INSTALL.md))
3. 🔄 **Restart the application** to detect virtual devices
4. 🎛️ **Configure audio routing** using the UI
5. 💾 **Save your first preset** for quick access

---

## 🆘 Getting Help

If you encounter issues:

1. **Check the logs** - Look for error messages in the terminal
2. **Verify dependencies** - Ensure all prerequisites are installed
3. **Check platform-specific guides** - See [DRIVER_INSTALL.md](DRIVER_INSTALL.md)
4. **Review development guide** - See [DEVELOPMENT.md](DEVELOPMENT.md)
5. **Search existing issues** - Check GitHub Issues for similar problems

---

<div align="center">

**Ready to start?** Run `pnpm tauri:dev` to launch the application! 🚀

</div>
