#!/bin/bash

# STLVI Build Script
# Builds the application for the current platform with icon support

set -e

APP_NAME="STLVI"
BINARY_NAME="stlvi"
VERSION="1.0.0"
ICON_PATH="assets/icons/stlvi-icon.png"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}Building ${APP_NAME} v${VERSION}...${NC}"

# Check if icon exists
if [ ! -f "$ICON_PATH" ]; then
    echo -e "${YELLOW}Warning: Icon not found at ${ICON_PATH}${NC}"
fi

# Build release binary
echo -e "${GREEN}Compiling release binary...${NC}"
cargo build --release

if [ $? -ne 0 ]; then
    echo -e "${RED}Build failed!${NC}"
    exit 1
fi

echo -e "${GREEN}Build successful!${NC}"

# Platform-specific packaging
case "$(uname -s)" in
    Darwin)
        echo -e "${GREEN}Creating macOS application bundle...${NC}"

        # Create app bundle structure
        APP_BUNDLE="target/release/${APP_NAME}.app"
        CONTENTS_DIR="${APP_BUNDLE}/Contents"
        MACOS_DIR="${CONTENTS_DIR}/MacOS"
        RESOURCES_DIR="${CONTENTS_DIR}/Resources"

        rm -rf "${APP_BUNDLE}"
        mkdir -p "${MACOS_DIR}"
        mkdir -p "${RESOURCES_DIR}"

        # Copy binary
        cp "target/release/${BINARY_NAME}" "${MACOS_DIR}/${APP_NAME}"

        # Create icns from png if icon exists
        if [ -f "$ICON_PATH" ]; then
            echo -e "${GREEN}Converting icon to icns format...${NC}"

            ICONSET_DIR="target/release/${APP_NAME}.iconset"
            mkdir -p "${ICONSET_DIR}"

            # Generate different icon sizes using sips
            sips -z 16 16     "$ICON_PATH" --out "${ICONSET_DIR}/icon_16x16.png" 2>/dev/null || true
            sips -z 32 32     "$ICON_PATH" --out "${ICONSET_DIR}/icon_16x16@2x.png" 2>/dev/null || true
            sips -z 32 32     "$ICON_PATH" --out "${ICONSET_DIR}/icon_32x32.png" 2>/dev/null || true
            sips -z 64 64     "$ICON_PATH" --out "${ICONSET_DIR}/icon_32x32@2x.png" 2>/dev/null || true
            sips -z 128 128   "$ICON_PATH" --out "${ICONSET_DIR}/icon_128x128.png" 2>/dev/null || true
            sips -z 256 256   "$ICON_PATH" --out "${ICONSET_DIR}/icon_128x128@2x.png" 2>/dev/null || true
            sips -z 256 256   "$ICON_PATH" --out "${ICONSET_DIR}/icon_256x256.png" 2>/dev/null || true
            sips -z 512 512   "$ICON_PATH" --out "${ICONSET_DIR}/icon_256x256@2x.png" 2>/dev/null || true
            sips -z 512 512   "$ICON_PATH" --out "${ICONSET_DIR}/icon_512x512.png" 2>/dev/null || true
            sips -z 1024 1024 "$ICON_PATH" --out "${ICONSET_DIR}/icon_512x512@2x.png" 2>/dev/null || true

            # Convert iconset to icns
            iconutil -c icns "${ICONSET_DIR}" -o "${RESOURCES_DIR}/${APP_NAME}.icns" 2>/dev/null || {
                echo -e "${YELLOW}Could not create icns file, copying png instead${NC}"
                cp "$ICON_PATH" "${RESOURCES_DIR}/${APP_NAME}.png"
            }

            rm -rf "${ICONSET_DIR}"
        fi

        # Create Info.plist
        cat > "${CONTENTS_DIR}/Info.plist" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleName</key>
    <string>${APP_NAME}</string>
    <key>CFBundleDisplayName</key>
    <string>${APP_NAME}</string>
    <key>CFBundleIdentifier</key>
    <string>com.mantreshkhurana.stlvi</string>
    <key>CFBundleVersion</key>
    <string>${VERSION}</string>
    <key>CFBundleShortVersionString</key>
    <string>${VERSION}</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleExecutable</key>
    <string>${APP_NAME}</string>
    <key>CFBundleIconFile</key>
    <string>${APP_NAME}</string>
    <key>LSMinimumSystemVersion</key>
    <string>10.13</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>CFBundleDocumentTypes</key>
    <array>
        <dict>
            <key>CFBundleTypeName</key>
            <string>STL File</string>
            <key>CFBundleTypeRole</key>
            <string>Editor</string>
            <key>LSHandlerRank</key>
            <string>Owner</string>
            <key>CFBundleTypeExtensions</key>
            <array>
                <string>stl</string>
                <string>STL</string>
            </array>
        </dict>
    </array>
    <key>NSAppleEventsUsageDescription</key>
    <string>STLVI needs to access files to open STL models.</string>
</dict>
</plist>
EOF

        echo -e "${GREEN}macOS app bundle created at: ${APP_BUNDLE}${NC}"
        echo ""
        echo -e "To install, run:"
        echo -e "  ${YELLOW}cp -r \"${APP_BUNDLE}\" /Applications/${NC}"
        echo ""
        echo -e "Or open the app directly:"
        echo -e "  ${YELLOW}open \"${APP_BUNDLE}\"${NC}"
        ;;

    Linux)
        echo -e "${GREEN}Creating Linux package...${NC}"

        LINUX_DIR="target/release/${APP_NAME}-linux"
        mkdir -p "${LINUX_DIR}"

        # Copy binary
        cp "target/release/${BINARY_NAME}" "${LINUX_DIR}/"

        # Copy icon if exists
        if [ -f "$ICON_PATH" ]; then
            cp "$ICON_PATH" "${LINUX_DIR}/"
        fi

        # Create desktop entry
        cat > "${LINUX_DIR}/${BINARY_NAME}.desktop" << EOF
[Desktop Entry]
Name=${APP_NAME}
Comment=STL 3D Model Viewer
Exec=${BINARY_NAME} %f
Icon=stlvi-icon
Terminal=false
Type=Application
Categories=Graphics;3DGraphics;
MimeType=model/stl;application/sla;
EOF

        echo -e "${GREEN}Linux package created at: ${LINUX_DIR}${NC}"
        echo ""
        echo -e "To install system-wide:"
        echo -e "  ${YELLOW}sudo cp ${LINUX_DIR}/${BINARY_NAME} /usr/local/bin/${NC}"
        echo -e "  ${YELLOW}sudo cp ${LINUX_DIR}/${BINARY_NAME}.desktop /usr/share/applications/${NC}"
        if [ -f "$ICON_PATH" ]; then
            echo -e "  ${YELLOW}sudo cp ${LINUX_DIR}/stlvi-icon.png /usr/share/icons/hicolor/256x256/apps/${NC}"
        fi
        ;;

    MINGW*|MSYS*|CYGWIN*)
        echo -e "${GREEN}Windows build complete!${NC}"
        echo ""
        echo -e "Binary location: ${YELLOW}target/release/${BINARY_NAME}.exe${NC}"

        # Copy to a dist folder
        WINDOWS_DIR="target/release/${APP_NAME}-windows"
        mkdir -p "${WINDOWS_DIR}"
        cp "target/release/${BINARY_NAME}.exe" "${WINDOWS_DIR}/"

        if [ -f "$ICON_PATH" ]; then
            cp "$ICON_PATH" "${WINDOWS_DIR}/"
        fi

        echo -e "${GREEN}Windows package created at: ${WINDOWS_DIR}${NC}"
        ;;

    *)
        echo -e "${YELLOW}Unknown platform. Binary is at: target/release/${BINARY_NAME}${NC}"
        ;;
esac

echo ""
echo -e "${GREEN}Build complete!${NC}"
echo -e "Binary size: $(du -h "target/release/${BINARY_NAME}" | cut -f1)"
