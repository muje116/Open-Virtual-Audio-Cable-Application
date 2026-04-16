# 🛠️ Development Guide

> Comprehensive guide for contributing to the Virtual Audio Cable project

---

## 🚀 Getting Started

### Prerequisites

| Tool | Purpose |
|------|---------|
| **Rust** | Backend compilation |
| **Node.js** | Frontend runtime |
| **pnpm** | Package manager |
| **VS Build Tools** (Windows) | Native compilation |
| **Xcode CLI** (macOS) | Native compilation |

### Installing Rust

```powershell
# Windows
winget install Rustlang.Rustup

# macOS/Linux
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Installing Node.js & pnpm

```powershell
# Node.js should be v18+ (already installed: v22.17.0)
npm install -g pnpm
```

### Platform-Specific Dependencies

- **Windows**: Visual Studio Build Tools
- **macOS**: Xcode Command Line Tools
- **Linux**: libwebkit2gtk and dependencies (see INSTALL.md)

---

## 🔄 Development Workflow

### 1. Clone the Repository

```bash
cd d:/Projects/VAC
```

### 2. Install Dependencies

```bash
pnpm install
```

### 3. Run in Development Mode

```bash
pnpm tauri:dev
```

This command will:
- ✅ Start the React dev server (Vite) with hot-reload
- ✅ Compile the Rust backend in debug mode
- ✅ Launch the Tauri application window
- ✅ Enable hot-reload for both frontend and backend

---

## 📁 Project Structure

```
src/                    # Frontend (React + TypeScript)
├── components/         # Reusable UI components
│   ├── RoutingMatrix.tsx    # N×M routing grid
│   ├── VuMeter.tsx          # Audio level meters
│   ├── DeviceList.tsx       # Device selection
│   ├── FxChain.tsx          # DSP effects UI
│   └── Presets.tsx          # Preset management
├── App.tsx            # Main application component
├── main.tsx           # React entry point
└── index.css          # Global styles

src-tauri/             # Backend (Rust)
├── src/
│   ├── main.rs        # Application entry point
│   ├── audio.rs       # Audio capture engine
│   ├── commands.rs    # Tauri IPC handlers
│   ├── config.rs      # Configuration management
│   ├── devices.rs     # Virtual device management
│   ├── dsp.rs         # Audio processing pipeline
│   └── routing.rs     # Routing matrix logic
├── Cargo.toml         # Rust dependencies
└── tauri.conf.json    # Tauri configuration
```

---

## 🎨 Frontend Development

### Adding a New Component

1. **Create component file** in `src/components/`
2. **Import in parent component**
3. **Add to appropriate tab or section**

```typescript
// src/components/MyComponent.tsx
export function MyComponent() {
  return <div>My Component</div>;
}

// src/App.tsx
import { MyComponent } from "./components/MyComponent";

// Use in JSX
<MyComponent />
```

### Tauri IPC Communication

Call Rust commands from React:

```typescript
import { invoke } from "@tauri-apps/api/core";

// Call a command
const devices = await invoke<Device[]>("get_audio_devices");

// With parameters
await invoke("start_audio_capture", { deviceId: "mic_1" });
```

### State Management

Use React hooks for local state:

```typescript
const [devices, setDevices] = useState<Device[]>([]);
const [isLoading, setIsLoading] = useState(true);

useEffect(() => {
  loadDevices();
}, []);
```

---

## 🦀 Backend Development

### Adding a New Tauri Command

1. **Add command function** in `src-tauri/src/commands.rs`:

```rust
#[tauri::command]
pub async fn my_command(param: String) -> Result<String, String> {
    // Your logic here
    Ok(format!("Received: {}", param))
}
```

2. **Register in `src-tauri/src/main.rs`**:

```rust
.invoke_handler(tauri::generate_handler![
    // ... existing commands
    commands::my_command,
])
```

3. **Call from frontend**:

```typescript
await invoke("my_command", { param: "hello" });
```

### Audio Processing

The audio pipeline is in `src-tauri/src/dsp.rs`:

```rust
pub fn process_samples(&self, samples: &[f32]) -> Vec<f32> {
    samples
        .iter()
        .map(|&sample| {
            // Apply gain
            let processed = sample * self.gain;
            
            // Apply noise gate
            let processed = if self.noise_gate_enabled {
                self.apply_noise_gate(processed)
            } else {
                processed
            };
            
            // Clamp to prevent clipping
            processed.clamp(-1.0, 1.0)
        })
        .collect()
}
```

### Adding Audio Effects

1. Implement effect in `src-tauri/src/dsp.rs`
2. Add to `DspProcessor` struct
3. Add UI controls in `src/components/FxChain.tsx`
4. Wire up via Tauri commands

---

## 🧪 Testing

### Frontend Testing

```bash
# TypeScript type check
npx tsc --noEmit

# ESLint
npx eslint src --ext .ts,.tsx

# Format with Prettier
npx prettier --write src/
```

### Backend Testing

```bash
# Rust format check
cargo fmt --check --manifest-path src-tauri/Cargo.toml

# Clippy (linter)
cargo clippy --manifest-path src-tauri/Cargo.toml

# Run tests
cargo test --manifest-path src-tauri/Cargo.toml

# Build check
cargo check --manifest-path src-tauri/Cargo.toml
```

### Manual Testing

1. Start development server: `pnpm tauri:dev`
2. Test each tab and component
3. Test audio routing with actual devices
4. Test DSP effects
5. Test preset save/load

---

## 🐛 Debugging

### Frontend Debugging

- Use browser DevTools (F12)
- React DevTools extension recommended
- Console logs appear in DevTools console

### Backend Debugging

```bash
# Enable debug logging
RUST_LOG=debug pnpm tauri:dev

# Or in code
println!("Debug info: {:?}", data);
```

### Common Issues

| Issue | Solution |
|-------|----------|
| **Rust not found** | Install Rust from https://rustup.rs/ and restart terminal |
| **Build errors on Windows** | Install Visual Studio Build Tools, use x64 Native Tools Command Prompt |
| **Tauri commands fail** | Check console for errors, verify command is registered in `main.rs`, check parameter types |
| **Audio devices not detected** | Check microphone permissions, verify virtual audio driver is installed, restart application |

---

## 📦 Building for Production

### Single Platform

```powershell
# Windows
.\scripts\build-windows.ps1

# macOS
chmod +x scripts/build-macos.sh
./scripts/build-macos.sh

# Linux
chmod +x scripts/build-linux.sh
./scripts/build-linux.sh
```

### Using Tauri CLI

```bash
pnpm tauri build
```

**Output**: `src-tauri/target/release/bundle/`

---

## 🚢 Release Process

1. **Update version** in `src-tauri/Cargo.toml` and `package.json`
2. **Update CHANGELOG.md**
3. **Run release preparation script**:
   ```bash
   ./scripts/prepare-release.sh v1.0.0
   ```
4. **Build for all platforms**
5. **Create GitHub release** with artifacts
6. **Upload release notes**

---

## 🤝 Contributing

### Code Style

- **Rust**: Use `cargo fmt` for formatting
- **TypeScript**: Use ESLint with Prettier
- **Components**: Use functional components with hooks

### Commit Messages

Follow conventional commits:

```
feat: add new feature
fix: fix bug in audio capture
docs: update installation guide
refactor: simplify routing logic
test: add unit tests for DSP
```

### Pull Request Process

1. Fork the repository
2. Create feature branch
3. Make changes with tests
4. Run linters and tests
5. Submit pull request with description

---

## ⚡ Performance Optimization

### Frontend

- Use React.memo for expensive components
- Virtualize long lists
- Debounce user inputs

### Backend

- Use lock-free channels for audio
- Minimize buffer copies
- Use SIMD for DSP (future enhancement)

### Profiling

```bash
# Flamegraph for Rust
cargo flamegraph --bin vac

# React Profiler
# Use React DevTools Profiler tab
```

---

## 📚 Resources

- [Tauri Documentation](https://tauri.app/v1/guides/)
- [React Documentation](https://react.dev/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [cpal Documentation](https://docs.rs/cpal/)
- [dasp Documentation](https://docs.rs/dasp/)

---

## 🆘 Getting Help

- Check existing issues on GitHub
- Read [ARCHITECTURE.md](ARCHITECTURE.md) for system design
- Read [INSTALL.md](INSTALL.md) for setup issues
- Read [DRIVER_INSTALL.md](DRIVER_INSTALL.md) for driver issues

---

<div align="center">

Happy coding! 🚀

</div>
