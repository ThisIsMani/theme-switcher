#!/bin/bash
# Installation script for theme-switcher

set -e

echo "🎨 Installing theme-switcher..."

# Build in release mode
echo "Building..."
cargo build --release

# Create config directory
CONFIG_DIR="$HOME/.config/theme-switcher"
mkdir -p "$CONFIG_DIR"

# Copy binary to /usr/local/bin
echo "Installing binary..."
cp target/release/theme-switcher $HOME/.local/bin/

# Create example config if it doesn't exist
if [ ! -f "$CONFIG_DIR/config.toml" ]; then
    echo "Creating example config..."
    cp examples/theme-switcher.toml "$CONFIG_DIR/config.toml"
    echo "Config created at: $CONFIG_DIR/config.toml"
    echo "Please edit this file to configure your scripts."
fi

# Create LaunchAgent plist
LAUNCH_AGENT_DIR="$HOME/Library/LaunchAgents"
PLIST_FILE="$LAUNCH_AGENT_DIR/com.theme-switcher.plist"

if [ ! -f "$PLIST_FILE" ]; then
    echo "Creating LaunchAgent..."
    mkdir -p "$LAUNCH_AGENT_DIR"

    cat >"$PLIST_FILE" <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.theme-switcher</string>
    <key>ProgramArguments</key>
    <array>
        <string>$HOME/.local/bin/theme-switcher</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
    <key>StandardOutPath</key>
    <string>$HOME/.config/theme-switcher/theme-switcher.log</string>
    <key>StandardErrorPath</key>
    <string>$HOME/.config/theme-switcher/theme-switcher.error.log</string>
</dict>
</plist>
EOF

    echo "Loading LaunchAgent..."
    launchctl load "$PLIST_FILE"
fi

echo "✅ Installation complete!"
echo ""
echo "Next steps:"
echo "1. Edit your config: $CONFIG_DIR/config.toml"
echo "2. Add your scripts to the config file"
echo "3. The service is now running in the background"
echo ""
echo "To stop the service: launchctl unload $PLIST_FILE"
echo "To start the service: launchctl load $PLIST_FILE"
