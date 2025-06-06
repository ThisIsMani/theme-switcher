-- Example Neovim configuration for theme-switcher
-- Add this to your init.lua or source it

-- For lazy.nvim users:
return {
	{
		"ThisIsMani/theme-switcher",
		config = function()
			require("theme-switcher").setup({
				schemes = {
					-- Simple colorscheme names
					light = "tokyonight-day",
					dark = "tokyonight-night",

					-- Or with more options:
					-- light = {
					--   colorscheme = "gruvbox",
					--   background = "light",
					--   lightline = "gruvbox",
					-- },
					-- dark = {
					--   colorscheme = "gruvbox",
					--   background = "dark",
					--   lightline = "gruvbox",
					-- }
				},

				-- If using lualine
				lualine = {
					light = "tokyonight",
					dark = "tokyonight",
				},

				-- Custom callback
				onchange = function(theme)
					-- You can add custom logic here
					-- For example, change other plugin settings
					vim.notify("Theme changed to " .. theme, vim.log.levels.INFO)
				end,
			})
		end,
	},
}

-- For manual setup without plugin manager:
-- require('theme-switcher').setup({
--   schemes = {
--     light = "tokyonight-day",
--     dark = "tokyonight-night"
--   }
-- })

