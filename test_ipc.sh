#!/bin/bash
# Test script for IPC functionality

echo "Testing theme-switcher IPC..."

# Start theme-switcher with IPC in background
echo "Starting theme-switcher with IPC..."
./target/debug/theme-switcher --ipc &
THEME_PID=$!

# Wait for socket to be created
sleep 2

# Find socket path
SOCKET_PATH=""
for path in "$XDG_RUNTIME_DIR/theme-switcher.sock" "$HOME/.local/run/theme-switcher.sock" "/tmp/theme-switcher.sock"; do
    if [ -e "$path" ]; then
        SOCKET_PATH="$path"
        break
    fi
done

if [ -z "$SOCKET_PATH" ]; then
    echo "Error: Could not find theme-switcher socket"
    kill $THEME_PID
    exit 1
fi

echo "Found socket at: $SOCKET_PATH"

# Connect to socket using nc
echo "Connecting to socket..."
nc -U "$SOCKET_PATH" &
NC_PID=$!

# Wait a bit to see output
sleep 3

# Send quit command
echo "quit" | nc -U "$SOCKET_PATH"

# Clean up
kill $NC_PID 2>/dev/null
kill $THEME_PID 2>/dev/null

echo "Test complete!"