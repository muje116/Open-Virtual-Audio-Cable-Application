# Virtual Audio Driver Installation Guide

This guide explains how to install the virtual audio drivers required for VAC to function on each platform.

## Overview

Virtual Audio Cable requires a kernel-level driver to create virtual audio devices that appear in your system's sound settings.

## macOS

### Option 1: BlackHole (Recommended)

BlackHole is a modern, open-source virtual audio driver for macOS.

1. Download BlackHole from [existential.audio/blackhole](https://existential.audio/blackhole/)
2. Install the `.pkg` file
3. Grant microphone permission when prompted
4. Restart the application

### Option 2: Custom AudioServerPlugin

For production builds, you can bundle a custom AudioServerPlugin:
- This requires Apple Developer certification
- See `src-tauri/drivers/macos/` for implementation details

### Troubleshooting
- If devices don't appear, restart your Mac
- Check System Preferences > Security & Privacy > Microphone permissions
- Ensure the VAC app has microphone access

## Windows

### Option 1: VB-Cable (Free)

1. Download VB-Cable from [vb-audio.com](https://vb-audio.com/Cable/)
2. Run the installer as Administrator
3. Accept the UAC prompt
4. Restart your computer

### Option 2: Custom WDM Driver (Future)

For production, we'll implement a custom WDM kernel driver:
- Requires Windows Driver Kit (WDK)
- Must be code-signed for distribution
- See `src-tauri/drivers/windows/` for implementation details

### Troubleshooting
- Run installer as Administrator
- Check Device Manager for driver installation status
- Restart if virtual devices don't appear in Sound settings

## Linux

### PipeWire (Recommended for modern distros)

PipeWire is the default on Fedora 34+, Ubuntu 22.04+, and Arch Linux.

1. Install PipeWire if not already installed:
   ```bash
   # Ubuntu/Debian
   sudo apt install pipewire pipewire-pulse wireplumber
   
   # Fedora
   sudo dnf install pipewire pipewire-pulseaudio wireplumber
   
   # Arch
   sudo pacman -S pipewire pipewire-pulse wireplumber
   ```

2. Create virtual sink:
   ```bash
   pactl load-module module-null-sink sink_name=VAC-1 sink_properties=device.description="VAC-1"
   ```

3. The application will automatically configure PipeWire on first launch

### PulseAudio (Legacy distros)

For older systems still using PulseAudio:

1. Install PulseAudio:
   ```bash
   sudo apt install pulseaudio pulseaudio-utils
   ```

2. Create virtual sink:
   ```bash
   pactl load-module module-null-sink sink_name=VAC-1
   ```

### Troubleshooting
- Ensure your user is in the `audio` group: `sudo usermod -a -G audio $USER`
- Restart PipeWire/PulseAudio: `systemctl --user restart pipewire`
- Check `pactl list sinks` to verify virtual devices

## Verification

After installing the driver:

1. Open your system's Sound Settings
2. Look for virtual devices named "VAC-1", "VAC-2", etc.
3. Launch the VAC application
4. The devices should appear in the Device Manager tab

## Uninstalling Drivers

### macOS
```bash
# Remove BlackHole
sudo rm -rf /Library/Audio/Plug-Ins/HAL/BlackHole2ch.driver
sudo kextunload -b audio.cma.BlackHole
```

### Windows
Use "Add or Remove Programs" to uninstall VB-Cable
Or manually remove from Device Manager

### Linux
```bash
# Remove PipeWire virtual sink
pactl unload-module module-null-sink
```

## Security Notes

- Always download drivers from official sources
- Windows drivers must be code-signed to avoid security warnings
- macOS requires kernel extensions to be approved in Recovery Mode
- Linux PipeWire runs in user space, no special privileges needed
