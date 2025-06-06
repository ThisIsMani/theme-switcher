-- Zellij theme switcher script
-- Changes between atom-one-light and atom-one-dark themes

local function change_zellij_theme()
	-- Try the stow location first, then fall back to standard location
	local config_path = os.getenv("HOME") .. "/dots/.config/zellij/config.kdl"
	local file = io.open(config_path, "r")
	if not file then
		config_path = os.getenv("HOME") .. "/.config/zellij/config.kdl"
	else
		file:close()
	end
	
	local theme = theme_switcher.get_current_theme()

	-- Determine the new theme name
	local new_theme = theme == "light" and "atom-one-light" or "atom-one-dark"

	-- Read the current config
	local file = io.open(config_path, "r")
	if not file then
		print("Error: Could not open Zellij config at " .. config_path)
		return
	end

	local content = file:read("*all")
	file:close()

	-- Replace the theme line
	local new_content = content:gsub('theme ".-"', 'theme "' .. new_theme .. '"')

	-- Write the updated config
	file = io.open(config_path, "w")
	if not file then
		print("Error: Could not write to Zellij config")
		return
	end

	file:write(new_content)
	file:close()

	print("Changed Zellij theme to: " .. new_theme)
	
	-- Touch the symlink to ensure Zellij notices the change
	local symlink_path = os.getenv("HOME") .. "/.config/zellij/config.kdl"
	os.execute("test -L '" .. symlink_path .. "' && touch -h '" .. symlink_path .. "'")
end

-- Execute the theme change
change_zellij_theme()

