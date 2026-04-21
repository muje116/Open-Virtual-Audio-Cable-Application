# Virtual Audio Cable (VAC)

<div align="center">

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey.svg)
![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)
![React](https://img.shields.io/badge/react-18%2B-blue.svg)

*A cross-platform virtual audio routing application with DSP capabilities*

[Features](#-features) • [Installation](#-installation) • [Usage](#-usage) • [Development](#-development) • [Contributing](#-contributing)

</div>

---

## ✨ Features

### Core Functionality
- 🎛️ **N×M Routing Matrix** - Route any input to any output with individual volume/mute controls
- 🎚️ **Real-time VU Meters** - Visual audio level monitoring for all channels
- 🎛️ **DSP Effects Chain** - Per-channel effects:
  - Gain control
  - 5-band parametric EQ
  - Noise gate
  - Compressor/limiter
- 💾 **Preset Management** - Save and load routing configurations
- 🔊 **Virtual Device Support** - Works with VB-Cable, BlackHole, and PipeWire

### Cross-Platform
- **Windows** - WASAPI + VB-Cable or custom WDM driver
- **macOS** - CoreAudio + BlackHole or AudioServerPlugin
- **Linux** - PipeWire/PulseAudio (userspace, no kernel driver required)

### Technical Highlights
- ⚡ **Low Latency** - Configurable buffer sizes (64-2048 samples)
- 🔒 **Secure** - Tauri sandbox, minimal permissions
- 🎨 **Modern UI** - React + TypeScript + Tailwind CSS
- 🦀 **Performance** - Rust backend with lock-free audio path

---

## 📦 Installation

### Prerequisites

1. **Rust** (v1.70+)
   ```powershell
   # Windows
   winget install Rustlang.Rustup
   
   # macOS/Linux
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Node.js** (v18+)
   ```powershell
   # Already installed: v22.17.0
   ```

3. **pnpm**
   ```powershell
   npm install -g pnpm
   ```

### Platform-Specific Dependencies

| Platform | Dependencies |
|----------|--------------|
| **Windows** | Visual Studio Build Tools with C++ support |
| **macOS** | Xcode Command Line Tools |
| **Linux** | libwebkit2gtk, build-essential (see INSTALL.md) |

### Quick Start

```bash
# Clone the repository
cd d:/Projects/VAC

# Install dependencies
pnpm install

# Run in development mode
pnpm tauri:dev
```

### Virtual Audio Driver

After installing the application, you need to install a virtual audio driver for your platform:

- **Windows**: [VB-Cable](https://vb-audio.com/Cable/) (free)
- **macOS**: [BlackHole](https://existential.audio/blackhole/) (free)
- **Linux**: PipeWire (built into most modern distros)

See [DRIVER_INSTALL.md](DRIVER_INSTALL.md) for detailed instructions.

---

## 🚀 Usage

### Basic Setup

1. **Launch the application** - `pnpm tauri:dev` or run the built executable
2. **Select Input Devices** - Choose microphones or system audio in the "Devices" tab
3. **Configure Routing** - Use the routing matrix to connect inputs to outputs
4. **Apply Effects** - Adjust DSP settings in the "Effects" tab
5. **Save Preset** - Save your configuration for later use

### Routing Matrix

The routing matrix allows you to:
- **Click any cell** to toggle routing on/off
- **Drag sliders** to adjust volume per route
- **Right-click** to mute individual routes
- **Use presets** for common configurations

### DSP Effects

- **Gain**: Adjust overall volume (-60dB to +12dB)
- **Noise Gate**: Suppress low-level noise
- **5-Band EQ**: Shape frequency response at 60Hz, 250Hz, 1kHz, 4kHz, 16kHz
- **Compressor**: Control dynamic range with threshold, ratio, attack, release

---

## 🛠️ Development

### Project Structure

```
src/                    # Frontend (React + TypeScript)
├── components/         # UI components
│   ├── RoutingMatrix.tsx
│   ├── VuMeter.tsx
│   ├── DeviceList.tsx
│   ├── FxChain.tsx
│   └── Presets.tsx
├── App.tsx            # Main application
└── main.tsx           # Entry point

src-tauri/             # Backend (Rust)
├── src/
│   ├── main.rs        # Entry point
│   ├── audio.rs       # Audio capture engine
│   ├── commands.rs    # Tauri IPC handlers
│   ├── devices.rs     # Device management
│   ├── dsp.rs         # Audio processing
│   └── routing.rs     # Routing logic
├── Cargo.toml         # Rust dependencies
└── tauri.conf.json    # Tauri configuration
```

### Development Commands

```bash
# Development mode with hot-reload
pnpm tauri:dev

# Type check frontend
npx tsc --noEmit

# Lint frontend
npx eslint src --ext .ts,.tsx

# Format Rust code
cargo fmt --manifest-path src-tauri/Cargo.toml

# Lint Rust code
cargo clippy --manifest-path src-tauri/Cargo.toml

# Build for production
pnpm tauri:build
```

### Adding New Features

See [DEVELOPMENT.md](DEVELOPMENT.md) for detailed development guide.

---

## 📚 Documentation

- [Installation Guide](INSTALL.md) - Detailed installation instructions
- [Driver Installation](DRIVER_INSTALL.md) - Virtual audio driver setup
- [Development Guide](DEVELOPMENT.md) - Development workflow and contributing
- [Architecture Documentation](ARCHITECTURE.md) - System design and components
- [Project Status](PROJECT_STATUS.md) - Current implementation status

---

## 🏗️ Architecture

The application uses a modern hybrid architecture:

```
┌─────────────────────────────────────────────────────────┐
│                    Desktop Application                   │
├─────────────────────────────────────────────────────────┤
│  React UI ←→ Tauri IPC ←→ Rust Core                    │
│  (Frontend)    (Commands)     (Backend)                 │
│                                                         │
│  Audio Pipeline: Capture → DSP → Route → Output         │
└─────────────────────────────────────────────────────────┘
```

See [ARCHITECTURE.md](ARCHITECTURE.md) for detailed architecture documentation.

---

## 🤝 Contributing

Contributions are welcome! Please follow these guidelines:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'feat: add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Code Style

- **Rust**: Use `cargo fmt` and `cargo clippy`
- **TypeScript**: Use ESLint with Prettier
- **Components**: Use functional components with hooks

### Commit Messages

Follow conventional commits:
- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation changes
- `refactor:` - Code refactoring
- `test:` - Test additions/changes

---

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 🙏 Acknowledgments

- [Tauri](https://tauri.app/) - Cross-platform desktop framework
- [cpal](https://github.com/RustAudio/cpal) - Cross-platform audio I/O library
- [dasp](https://github.com/RustAudio/dasp) - Digital signal processing
- [React](https://react.dev/) - UI library
- [Tailwind CSS](https://tailwindcss.com/) - Styling framework

---

## 📞 Support

- 📖 [Documentation](#-documentation)
- 🐛 [Report Issues](https://github.com/yourusername/vac/issues)
- 💬 [Discussions](https://github.com/yourusername/vac/discussions)

---

<div align="center">

Made with ❤️ by the James Simbi

</div>
