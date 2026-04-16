#!/bin/bash

# macOS Build Script for Virtual Audio Cable

set -e

echo "Building Virtual Audio Cable for macOS..."

# Check if Rust is installed
if ! command -v rustc &> /dev/null; then
    echo "Error: Rust is not installed. Please install from https://rustup.rs/"
    exit 1
fi

# Check if Node.js is installed
if ! command -v node &> /dev/null; then
    echo "Error: Node.js is not installed. Please install from https://nodejs.org/"
    exit 1
fi

# Check if pnpm is installed
if ! command -v pnpm &> /dev/null; then
    echo "Installing pnpm..."
    npm install -g pnpm
fi

# Install dependencies
echo "Installing dependencies..."
pnpm install

# Build frontend
echo "Building frontend..."
pnpm build

# Build Tauri app
echo "Building Tauri application..."
pnpm tauri build

echo "Build complete! Output is in src-tauri/target/release/bundle/"
