#!/bin/bash
# Example script for dark theme

echo "Switching to dark theme!"
echo "Theme is: $THEME_SWITCHER_THEME"
echo "Theme (uppercase) is: $THEME_SWITCHER_THEME_UPPER"

# Example: Change terminal colors
# osascript -e 'tell application "Terminal" to set current settings of first window to settings set "Pro"'

# Example: Change VS Code theme
# code --install-extension GitHub.github-vscode-theme
# echo '{"workbench.colorTheme": "GitHub Dark"}' > ~/.config/Code/User/settings.json