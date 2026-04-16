# Architecture Documentation

## System Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                         Desktop Application                      │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐         │
│  │   React UI  │◄──►│  Tauri IPC  │◄──►│ Rust Core   │         │
│  │ (Frontend)  │    │  Commands   │    │  Backend    │         │
│  └─────────────┘    └─────────────┘    └─────────────┘         │
│                                               │                  │
│                                               ▼                  │
│  ┌──────────────────────────────────────────────────────┐     │
│  │              Audio Processing Pipeline                │     │
│  │  ┌──────┐  ┌──────┐  ┌──────┐  ┌──────┐  ┌──────┐ │     │
│  │  │Capture│→│ DSP  │→│ Route│→│ Resample│→│Output│ │     │
│  │  │Thread │  │Engine│  │Matrix│  │  Engine  ││Device│ │     │
│  │  └──────┘  └──────┘  └──────┘  └──────┘  └──────┘ │     │
│  └──────────────────────────────────────────────────────┘     │
│                                               │                  │
│                                               ▼                  │
│  ┌──────────────────────────────────────────────────────┐     │
│  │           Platform Audio Drivers                      │     │
│  │  ┌─────────┐  ┌─────────┐  ┌─────────┐               │     │
│  │  │ macOS   │  │ Windows │  │ Linux   │               │     │
│  │  │CoreAudio│  │ WASAPI  │  │PipeWire │               │     │
│  │  └─────────┘  └─────────┘  └─────────┘               │     │
│  └──────────────────────────────────────────────────────┘     │
│                                                                   │
└─────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────┐
│                      Mobile Companion App                        │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐         │
│  │  Flutter UI │◄──►│ Platform    │◄──►│  Audio      │         │
│  │             │    │ Channels    │    │  Engine     │         │
│  └─────────────┘    └─────────────┘    └─────────────┘         │
│                                               │                  │
│                                               ▼                  │
│  ┌──────────────────────────────────────────────────────┐     │
│  │           Network Streaming (WebRTC/UDP)              │     │
│  └──────────────────────────────────────────────────────┘     │
│                                               │                  │
│                                               ▼                  │
│  ┌──────────────────────────────────────────────────────┐     │
│  │              Desktop VAC (Network Input)              │     │
│  └──────────────────────────────────────────────────────┘     │
│                                                                   │
└─────────────────────────────────────────────────────────────────┘
```

## Component Breakdown

### Frontend (React + TypeScript + Tailwind CSS)

**Location**: `src/`

- **App.tsx**: Main application component with tab navigation
- **components/**: Reusable UI components
  - `RoutingMatrix.tsx`: N×M routing grid with volume/mute controls
  - `VuMeter.tsx`: Real-time audio level visualization
  - `DeviceList.tsx`: Audio device selection interface
  - `FxChain.tsx`: Per-channel DSP configuration (EQ, compressor, noise gate)
  - `Presets.tsx`: Save/load routing configurations

**State Management**: React hooks (useState, useEffect)
**IPC Communication**: Tauri `invoke()` API for Rust backend calls

### Backend (Rust + Tauri v2)

**Location**: `src-tauri/src/`

- **main.rs**: Application entry point, Tauri setup
- **audio.rs**: Audio capture engine using cpal
  - Device enumeration
  - Stream management
  - Multi-format support (f32, i16, u16)
- **commands.rs**: Tauri IPC command handlers
  - Device management
  - Route configuration
  - Preset operations
- **config.rs**: Application configuration (buffer size, sample rate, theme)
- **devices.rs**: Virtual device management
- **dsp.rs**: Audio processing pipeline
  - Gain control
  - Noise gate
  - 5-band EQ
  - Compressor/limiter
- **routing.rs**: Routing matrix logic (N inputs → M outputs)

### Audio Processing Pipeline

1. **Capture Thread**: Reads audio from input devices (microphone, system audio, files)
2. **DSP Engine**: Applies per-channel effects (gain, EQ, noise gate, compressor)
3. **Routing Matrix**: Routes processed audio to virtual output devices
4. **Resampling**: Converts sample rates to match destination requirements
5. **Output**: Writes to virtual audio devices

### Platform-Specific Audio Drivers

**macOS**:
- CoreAudio framework
- BlackHole virtual audio driver
- AudioServerPlugin for custom virtual devices

**Windows**:
- WASAPI (Windows Audio Session API)
- VB-Cable or custom WDM kernel driver
- Loopback capture for system audio

**Linux**:
- PipeWire (modern) or PulseAudio (legacy)
- Virtual sink/source creation via libpulse/pipewire-sys
- No kernel driver required (userspace)

## Data Flow

```
User Input (UI)
    ↓
Tauri IPC Command
    ↓
Rust Command Handler
    ↓
Audio Engine / DSP
    ↓
Platform Audio Driver
    ↓
Virtual Audio Device
    ↓
System Audio Subsystem
    ↓
Other Applications (Zoom, Audition, etc.)
```

## Threading Model

- **Main Thread**: UI rendering, Tauri IPC
- **Audio Capture Thread**: Reads from input devices (one per device)
- **DSP Thread**: Processes audio samples
- **Routing Thread**: Mixes and routes to outputs
- **Output Thread**: Writes to virtual devices

## Performance Considerations

- **Lock-free audio path**: Uses crossbeam channels for audio data
- **Zero-copy where possible**: Minimizes buffer copies
- **Configurable buffer size**: 64-2048 samples (latency vs stability tradeoff)
- **SIMD optimizations**: Future enhancement for DSP operations

## Configuration Storage

- **Location**: `~/.config/vac/config.json`
- **Format**: JSON
- **Settings**: Buffer size, sample rate, theme, startup behavior, default preset

## Preset Storage

- **Location**: `~/.config/vac/presets/`
- **Format**: JSON files (one per preset)
- **Contents**: Routing matrix, DSP settings, device configurations

## Security Model

- **Sandboxing**: Tauri provides security by default
- **Permissions**: Microphone access requested on first launch
- **Driver Installation**: Requires admin/sudo privileges
- **Network**: Mobile companion uses LAN only (no internet required)

## Extensibility

### Adding New Audio Effects

1. Implement effect in `src-tauri/src/dsp.rs`
2. Add UI controls in `src/components/FxChain.tsx`
3. Add Tauri command in `src-tauri/src/commands.rs`
4. Update preset schema in `src-tauri/src/routing.rs`

### Adding New Platform Support

1. Create platform-specific module in `src-tauri/src/`
2. Implement audio capture/output using native APIs
3. Add driver installation instructions in `DRIVER_INSTALL.md`
4. Update build configuration in `src-tauri/Cargo.toml`

## Future Enhancements

- [ ] Waveform oscilloscope visualization
- [ ] Real-time spectrum analyzer
- [ ] VST plugin support
- [ ] Network audio streaming (RTP/SRT)
- [ ] Cloud preset sync
- [ ] Scriptable routing (Lua/JavaScript)
- [ ] Multi-channel surround sound support
- [ ] Automatic gain control (AGC)
- [ ] Acoustic echo cancellation (AEC)
