# 🎧 Virtual Audio Driver Installation Guide

> This guide explains how to install the virtual audio drivers required for VAC to function on each platform.

---

## 📋 Overview

Virtual Audio Cable requires a kernel-level driver to create virtual audio devices that appear in your system's sound settings. Each platform has different driver requirements and installation procedures.

---

## 🍎 macOS

### Option 1: BlackHole (Recommended)

**BlackHole** is a modern, open-source virtual audio driver for macOS.

#### Installation Steps

1. **Download BlackHole**
   - Visit [existential.audio/blackhole](https://existential.audio/blackhole/)
   - Download the latest version for your macOS version

2. **Install the driver**
   ```bash
   # Double-click the downloaded .pkg file
   # Follow the installation prompts
   ```

3. **Grant permissions**
   - When prompted, grant microphone permission
   - Go to **System Preferences** → **Security & Privacy** → **Privacy** → **Microphone**
   - Ensure your terminal or VAC app has permission

4. **Restart the application**
   - Relaunch VAC to detect the new virtual devices

### Option 2: Custom AudioServerPlugin

For production builds, you can bundle a custom AudioServerPlugin:

- Requires Apple Developer certification
- See `src-tauri/drivers/macos/` for implementation details

### 🔧 Troubleshooting (macOS)

| Issue | Solution |
|-------|----------|
| Devices don't appear | Restart your Mac |
| Permission denied | Check System Preferences → Security & Privacy → Microphone |
| No audio output | Ensure VAC app has microphone access in Sound settings |

---

## 🪟 Windows

### Option 1: VB-Cable (Free)

**VB-Cable** is a free virtual audio driver for Windows.

#### Installation Steps

1. **Download VB-Cable**
   - Visit [vb-audio.com](https://vb-audio.com/Cable/)
   - Download the VB-Cable installer

2. **Run installer as Administrator**
   ```powershell
   # Right-click the installer
   # Select "Run as administrator"
   # Accept the UAC prompt
   ```

3. **Restart your computer**
   - A restart is required for the driver to load properly

### Option 2: Custom WDM Driver (Future)

For production, we'll implement a custom WDM kernel driver:

- Requires Windows Driver Kit (WDK)
- Must be code-signed for distribution
- See `src-tauri/drivers/windows/` for implementation details

### 🔧 Troubleshooting (Windows)

| Issue | Solution |
|-------|----------|
| Installation fails | Run installer as Administrator |
| Devices not visible | Check Device Manager for driver status |
| No audio output | Restart the computer after installation |
| Security warning | Install from official source only |

---

## 🐧 Linux

### PipeWire (Recommended for modern distros)

**PipeWire** is the default on Fedora 34+, Ubuntu 22.04+, and Arch Linux.

#### Installation Steps

1. **Install PipeWire** (if not already installed)

   ```bash
   # Ubuntu/Debian
   sudo apt update
   sudo apt install pipewire pipewire-pulse wireplumber
   
   # Fedora
   sudo dnf install pipewire pipewire-pulseaudio wireplumber
   
   # Arch Linux
   sudo pacman -S pipewire pipewire-pulse wireplumber
   ```

2. **Create virtual sink**
   ```bash
   pactl load-module module-null-sink \
     sink_name=VAC-1 \
     sink_properties=device.description="VAC-1"
   ```

3. **Automatic configuration**
   - The VAC application will automatically configure PipeWire on first launch

### PulseAudio (Legacy distros)

For older systems still using PulseAudio:

#### Installation Steps

1. **Install PulseAudio**
   ```bash
   sudo apt update
   sudo apt install pulseaudio pulseaudio-utils
   ```

2. **Create virtual sink**
   ```bash
   pactl load-module module-null-sink sink_name=VAC-1
   ```

### 🔧 Troubleshooting (Linux)

| Issue | Solution |
|-------|----------|
| No audio devices | Add user to `audio` group: `sudo usermod -a -G audio $USER` |
| Devices not visible | Restart PipeWire: `systemctl --user restart pipewire` |
| Permission denied | Ensure user is in audio group, then logout and login |
| Verify devices | Run `pactl list sinks` to check virtual devices |

---

## ✅ Verification

After installing the driver, verify the installation:

1. **Open your system's Sound Settings**
   - Windows: Settings → System → Sound
   - macOS: System Preferences → Sound
   - Linux: Settings → Sound or use `pactl list sinks`

2. **Look for virtual devices**
   - Named "VAC-1", "VAC-2", "VB-Cable", "BlackHole", etc.

3. **Launch the VAC application**
   - The devices should appear in the **Device Manager** tab

4. **Test audio routing**
   - Route an input to the virtual device
   - Check that audio flows correctly

---

## 🗑️ Uninstalling Drivers

### macOS

```bash
# Remove BlackHole driver
sudo rm -rf /Library/Audio/Plug-Ins/HAL/BlackHole2ch.driver
sudo kextunload -b audio.cma.BlackHole

# Restart to complete removal
```

### Windows

```powershell
# Use Windows Settings
Settings → Apps → Installed apps → VB-Cable → Uninstall

# Or use Device Manager
devmgmt.msc → Sound, video and game controllers → VB-Cable → Uninstall device
```

### Linux

```bash
# Remove PipeWire virtual sink
pactl unload-module module-null-sink

# Or remove PulseAudio virtual sink
pactl unload-module module-null-sink
```

---

## 🔒 Security Notes

| Platform | Security Considerations |
|----------|------------------------|
| **Windows** | Drivers must be code-signed to avoid security warnings |
| **macOS** | Kernel extensions must be approved in Recovery Mode |
| **Linux** | PipeWire runs in user space, no special privileges needed |

### General Security Best Practices

- ✅ Always download drivers from official sources
- ✅ Verify digital signatures when available
- ✅ Review driver permissions before installation
- ❌ Never install drivers from untrusted sources
- ❌ Avoid pirated or modified driver packages

---

## 📚 Additional Resources

- [BlackHole Documentation](https://github.com/ExistentialAudio/BlackHole)
- [VB-Audio Documentation](https://vb-audio.com/)
- [PipeWire Documentation](https://docs.pipewire.org/)
- [PulseAudio Documentation](https://www.freedesktop.org/wiki/Software/PulseAudio/)

---

<div align="center">

**Need help?** Check the [Installation Guide](INSTALL.md) or [Development Guide](DEVELOPMENT.md)

</div>
