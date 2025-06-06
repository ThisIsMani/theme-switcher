-- Example: Change VS Code theme based on system theme

local vscode_settings_path = os.getenv("HOME") .. "/.config/Code/User/settings.json"

-- Read current settings
local function read_json_file(path)
    local file = io.open(path, "r")
    if not file then
        return nil
    end
    local content = file:read("*all")
    file:close()
    return content
end

-- Simple JSON update (for demonstration - use a proper JSON library in production)
local function update_theme_in_json(json_content, theme_name)
    if not json_content then
        -- Create new settings file
        return string.format([[{
    "workbench.colorTheme": "%s"
}]], theme_name)
    end
    
    -- Update existing theme setting
    local updated = json_content:gsub(
        '"workbench%.colorTheme"%s*:%s*"[^"]*"',
        '"workbench.colorTheme": "' .. theme_name .. '"'
    )
    
    -- If pattern not found, add it
    if updated == json_content then
        -- Insert before the last }
        updated = json_content:gsub(
            '}%s*$',
            ',\n    "workbench.colorTheme": "' .. theme_name .. '"\n}'
        )
    end
    
    return updated
end

-- Determine VS Code theme based on system theme
local vscode_theme
if IS_DARK then
    vscode_theme = "GitHub Dark"
    theme_switcher.log("Setting VS Code to dark theme")
else
    vscode_theme = "GitHub Light"
    theme_switcher.log("Setting VS Code to light theme")
end

-- Update VS Code settings
local current_settings = read_json_file(vscode_settings_path)
local new_settings = update_theme_in_json(current_settings, vscode_theme)

if new_settings then
    -- Ensure directory exists
    theme_switcher.execute("mkdir -p ~/.config/Code/User")
    
    -- Write updated settings
    local file = io.open(vscode_settings_path, "w")
    if file then
        file:write(new_settings)
        file:close()
        theme_switcher.log("VS Code theme updated to: " .. vscode_theme)
    else
        theme_switcher.log_error("Failed to write VS Code settings")
    end
end

-- If VS Code is running, you might need to reload the window
-- theme_switcher.execute("code --reload-window")