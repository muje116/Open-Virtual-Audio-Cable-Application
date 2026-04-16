# Windows Build Script for Virtual Audio Cable

Write-Host "Building Virtual Audio Cable for Windows..." -ForegroundColor Green

# Check if Rust is installed
$rustc = Get-Command rustc -ErrorAction SilentlyContinue
if (-not $rustc) {
    Write-Host "Error: Rust is not installed. Please install from https://rustup.rs/" -ForegroundColor Red
    exit 1
}

# Check if Node.js is installed
$node = Get-Command node -ErrorAction SilentlyContinue
if (-not $node) {
    Write-Host "Error: Node.js is not installed. Please install from https://nodejs.org/" -ForegroundColor Red
    exit 1
}

# Check if pnpm is installed
$pnpm = Get-Command pnpm -ErrorAction SilentlyContinue
if (-not $pnpm) {
    Write-Host "Installing pnpm..."
    npm install -g pnpm
}

# Install dependencies
Write-Host "Installing dependencies..."
pnpm install

# Build frontend
Write-Host "Building frontend..."
pnpm build

# Build Tauri app
Write-Host "Building Tauri application..."
pnpm tauri build

Write-Host "Build complete! Output is in src-tauri\target\release\bundle\" -ForegroundColor Green
