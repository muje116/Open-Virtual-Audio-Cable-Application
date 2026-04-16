#!/bin/bash

# Prepare Release Script - Creates release notes and bundles

set -e

VERSION=$1

if [ -z "$VERSION" ]; then
    echo "Usage: ./prepare-release.sh <version>"
    exit 1
fi

echo "Preparing release $VERSION..."

# Create release notes
cat > RELEASE_NOTES.md << EOF
# Virtual Audio Cable v$VERSION

## Release Notes

### Features
- Virtual Audio Cable application for routing audio between applications
- Support for multiple input sources (microphone, system audio, files)
- N×M routing matrix with per-route volume and mute controls
- Real-time VU meters and audio level monitoring
- FX Chain with gain, noise gate, 5-band EQ, and compressor
- Preset management system
- Cross-platform support (macOS, Windows, Linux)

### Installation

#### Windows
1. Download \`vac-$VERSION-setup.exe\`
2. Run the installer as Administrator
3. Install VB-Cable virtual audio driver (see DRIVER_INSTALL.md)
4. Restart the application

#### macOS
1. Download \`vac_$VERSION.dmg\`
2. Drag to Applications folder
3. Install BlackHole virtual audio driver (see DRIVER_INSTALL.md)
4. Grant microphone permission when prompted

#### Linux
1. Download \`vac_$VERSION.AppImage\`
2. Make executable: \`chmod +x vac_$VERSION.AppImage\`
3. Run: \`./vac_$VERSION.AppImage\`
4. PipeWire/PulseAudio will be configured automatically

### Known Issues
- Virtual audio drivers must be installed separately
- Requires microphone permission on macOS
- May require restarting after driver installation

### Documentation
- Full documentation: https://github.com/yourusername/vac
- Installation guide: INSTALL.md
- Driver installation: DRIVER_INSTALL.md
- Architecture: ARCHITECTURE.md
EOF

echo "Release notes created in RELEASE_NOTES.md"
echo "Manual steps:"
echo "1. Review and update RELEASE_NOTES.md"
echo "2. Run build scripts for each platform"
echo "3. Create GitHub release with artifacts"
echo "4. Upload release notes"
