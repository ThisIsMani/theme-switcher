-- Example Lua script that runs on any theme change

-- Access theme information
theme_switcher.log("Theme changed to: " .. THEME)
theme_switcher.log("Is dark mode: " .. tostring(IS_DARK))
theme_switcher.log("Is light mode: " .. tostring(IS_LIGHT))

-- Example: Update a configuration file
if IS_DARK then
    -- Dark theme settings
    local config = [[
{
    "theme": "dark",
    "background": "#1e1e1e",
    "foreground": "#ffffff"
}
]]
    local file = io.open(os.getenv("HOME") .. "/.config/myapp/theme.json", "w")
    if file then
        file:write(config)
        file:close()
        theme_switcher.log("Updated config for dark theme")
    end
else
    -- Light theme settings
    local config = [[
{
    "theme": "light",
    "background": "#ffffff",
    "foreground": "#000000"
}
]]
    local file = io.open(os.getenv("HOME") .. "/.config/myapp/theme.json", "w")
    if file then
        file:write(config)
        file:close()
        theme_switcher.log("Updated config for light theme")
    end
end

-- Example: Execute system commands
local result = theme_switcher.execute("echo 'Current theme: " .. THEME .. "'")
if result.success then
    theme_switcher.log("Command output: " .. result.stdout)
else
    theme_switcher.log_error("Command failed: " .. result.stderr)
end

-- Example: Notify user (macOS)
if IS_DARK then
    theme_switcher.execute([[
        osascript -e 'display notification "Switched to dark theme" with title "Theme Switcher"'
    ]])
else
    theme_switcher.execute([[
        osascript -e 'display notification "Switched to light theme" with title "Theme Switcher"'
    ]])
end