# VAC Functional Guide

This guide covers:
- what is implemented and functional,
- how to validate it end-to-end,
- required Windows build tooling for Tauri/Rust,
- and a final activity checklist.

## 1) Current Functional Scope

### Frontend
- Device listing, routing matrix, FX chain UI, presets UI, settings/mobile placeholders.
- Route add/remove, route volume/mute updates.
- Preset save/load/delete/export/import actions.
- Toast notifications for success/error on backend actions.

### Backend (Tauri/Rust)
- Audio device enumeration.
- In-memory virtual device management (`create_virtual_device`, `delete_virtual_device`).
- In-memory routing state management (`set_route`, `remove_route`, `get_routes`, `set_volume`, `set_mute`).
- DSP settings persistence per input device (`set_device_dsp`, `get_device_dsp`).
- Preset persistence to disk (`save_preset`, `load_preset`, `get_presets`, `delete_preset`).
- Preset export/import (`export_preset`, `import_preset`).

## 2) Activity Checklist (Completion)

- [x] Rewired shared backend app state.
- [x] Implemented missing routing/device commands.
- [x] Implemented preset persistence commands.
- [x] Added DSP settings model and DSP pipeline usage in capture path.
- [x] Wired frontend route/preset/device actions to backend commands.
- [x] Added UX toasts for action failures and key successes.
- [x] Added preset export/import commands and UI wiring.
- [x] Fixed Add Route button behavior.
- [x] Frontend type/build check passes (`npm run build`).

## 3) End-to-End Validation Steps

Run these from project root:

```powershell
npm install
npm run build
```

Then run Tauri app:

```powershell
npm run tauri:dev
```

Validation flow in app:
1. Open **Devices** and create a virtual device.
2. Open **Dashboard**, add a route, move volume, toggle mute.
3. Open **FX Chain**, modify gain/noise gate/EQ/compressor.
4. Open **Presets**, save a preset, load it, delete it.
5. Export preset to JSON and re-import it.
6. Confirm toast messages appear on action success/failure.

## 4) Windows Tooling Required (Tauri + Rust)

If Rust/Tauri compile fails with errors such as missing `excpt.h` or `msvcrt.lib`, install/fix the following.

### Required installs
1. **Visual Studio 2022 Build Tools** (or full VS 2022).
2. Workload: **Desktop development with C++**.
3. Individual components:
   - **MSVC v143 - VS 2022 C++ x64/x86 build tools**
   - **Windows 10 SDK (10.0.19041+)** or **Windows 11 SDK (10.0.22621+)**
   - **C++ CMake tools for Windows** (recommended)
4. Rust toolchain:
   - `rustup default stable-x86_64-pc-windows-msvc`

### Verify from terminal

```powershell
rustup show
where cl
where link
```

You should see MSVC `cl.exe` and `link.exe` paths from VS 2022.

### Recommended shell
Use **x64 Native Tools Command Prompt for VS 2022** or a terminal launched after `VsDevCmd.bat`, then run:

```powershell
cd D:\Projects\VAC
npm run tauri:dev
```

## 5) Notes / Limitations

- Virtual device creation is app-managed metadata (not OS kernel driver installation).
- Real OS-level loopback/routing still depends on actual installed virtual audio driver stack (VB-Cable/BlackHole/PipeWire equivalents).

## 6) Quick Troubleshooting

- `Cannot open include file: excpt.h`:
  - C++ workload/Windows SDK incomplete; reinstall required components above.
- `LNK1104: cannot open file msvcrt.lib`:
  - MSVC runtime libs not available in current shell; use VS Native Tools prompt.
- Frontend works but Tauri fails:
  - Run `npm run build` first to isolate frontend vs native toolchain issue.
