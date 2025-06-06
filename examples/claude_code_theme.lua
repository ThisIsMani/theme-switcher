-- Claude Code theme switcher script
-- Changes between light and dark themes

local json = require("json") -- Note: Requires a JSON library for Lua

local function change_claude_code_theme()
    local config_path = os.getenv("HOME") .. "/.claude.json"
    local theme = theme_switcher.get_current_theme()

    -- Read the current config
    local file = io.open(config_path, "r")
    if not file then
        print("Error: Could not open Claude Code config at " .. config_path)
        return
    end

    local content = file:read("*all")
    file:close()

    -- Simple pattern-based replacement for JSON
    -- This works for the simple case where theme is a string value
    local new_content = content:gsub('"theme"%s*:%s*".-"', '"theme": "' .. theme .. '"')

    -- Write the updated config
    file = io.open(config_path, "w")
    if not file then
        print("Error: Could not write to Claude Code config")
        return
    end

    file:write(new_content)
    file:close()

    print("Changed Claude Code theme to: " .. theme)
end

-- Execute the theme change
change_claude_code_theme()