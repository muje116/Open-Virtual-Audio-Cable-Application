# Installation Guide

## Prerequisites

### Required Tools

1. **Rust** - Install from [rustup.rs](https://rustup.rs/)
   ```powershell
   winget install Rustlang.Rustup
   # Or download from https://rustup.rs/
   ```

2. **Node.js** - v18 or higher (already installed: v22.17.0)

3. **pnpm** - Package manager (already installed: v10.24.0)

### Platform-Specific Dependencies

#### Windows
Install Visual Studio Build Tools with C++ support:
```powershell
winget install Microsoft.VisualStudio.2022.BuildTools --override "--wait --passive --add Microsoft.VisualStudio.Workload.VCTools;includeRecommended"
```

#### macOS
Install Xcode Command Line Tools:
```bash
xcode-select --install
```

#### Linux
```bash
sudo apt update
sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget file libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev
```

## Installation Steps

1. **Clone/Navigate to project**:
   ```powershell
   cd d:\Projects\VAC
   ```

2. **Install Node dependencies** (already done):
   ```powershell
   pnpm install
   ```

3. **Install Rust** (if not already installed):
   ```powershell
   # Download and run rustup-init.exe from https://rustup.rs/
   ```

4. **Verify Rust installation**:
   ```powershell
   cargo --version
   rustc --version
   ```

## Running the Application

### Development Mode

```powershell
# This will build the Rust backend and start the React dev server
pnpm tauri:dev
```

### Production Build

```powershell
# Build the application for your platform
pnpm tauri:build
```

The built application will be in `src-tauri/target/release/bundle/`

## Troubleshooting

### Rust not found
If you see "cargo: command not found", install Rust from [rustup.rs](https://rustup.rs/)

### Build errors on Windows
- Ensure Visual Studio Build Tools are installed
- Run from "x64 Native Tools Command Prompt for VS 2022"

### Build errors on macOS
- Ensure Xcode Command Line Tools are installed
- Run: `xcode-select --install`

### Build errors on Linux
- Install the required dependencies listed above
- Ensure you have the latest versions of webkit2gtk

## Next Steps After Installation

1. Run `pnpm tauri:dev` to start the application
2. The application will open with a mock UI (since audio drivers need OS-specific installation)
3. Install the appropriate virtual audio driver for your platform (see DRIVER_INSTALL.md)
4. Restart the application to enable full audio functionality
