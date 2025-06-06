use crate::{Theme, handlers::ThemeHandler};
use mlua::{Lua, Result as LuaResult};
use std::path::PathBuf;

/// Handler that executes Lua scripts when theme changes
pub struct LuaHandler {
    light_scripts: Vec<PathBuf>,
    dark_scripts: Vec<PathBuf>,
    any_change_scripts: Vec<PathBuf>,
}

impl LuaHandler {
    pub fn new() -> LuaResult<Self> {
        Ok(Self {
            light_scripts: Vec::new(),
            dark_scripts: Vec::new(),
            any_change_scripts: Vec::new(),
        })
    }
    
    fn create_lua_context(&self) -> LuaResult<Lua> {
        let lua = Lua::new();
        
        // Create theme-switcher module
        let theme_switcher = lua.create_table()?;
        
        // Add utility functions
        theme_switcher.set("execute", lua.create_function(|lua_ctx, cmd: String| {
            use std::process::Command;
            let output = Command::new("sh")
                .arg("-c")
                .arg(&cmd)
                .output()
                .map_err(|e| mlua::Error::external(e))?;
            
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            let success = output.status.success();
            
            let result = lua_ctx.create_table()?;
            result.set("stdout", stdout)?;
            result.set("stderr", stderr)?;
            result.set("success", success)?;
            
            Ok(result)
        })?)?;
        
        // Add logging function
        theme_switcher.set("log", lua.create_function(|_, msg: String| {
            use crate::config::log_info;
            log_info(&msg);
            Ok(())
        })?)?;
        
        // Add error logging function
        theme_switcher.set("log_error", lua.create_function(|_, msg: String| {
            eprintln!("{}", msg);
            Ok(())
        })?)?;
        
        // Set the module as a global
        lua.globals().set("theme_switcher", theme_switcher)?;
        
        Ok(lua)
    }
    
    pub fn with_light_script(mut self, path: PathBuf) -> Self {
        self.light_scripts.push(path);
        self
    }
    
    pub fn with_dark_script(mut self, path: PathBuf) -> Self {
        self.dark_scripts.push(path);
        self
    }
    
    pub fn with_any_change_script(mut self, path: PathBuf) -> Self {
        self.any_change_scripts.push(path);
        self
    }
    
    pub fn with_light_scripts(mut self, paths: Vec<PathBuf>) -> Self {
        self.light_scripts.extend(paths);
        self
    }
    
    pub fn with_dark_scripts(mut self, paths: Vec<PathBuf>) -> Self {
        self.dark_scripts.extend(paths);
        self
    }
    
    pub fn with_any_change_scripts(mut self, paths: Vec<PathBuf>) -> Self {
        self.any_change_scripts.extend(paths);
        self
    }
    
    fn execute_script(&self, script_path: &PathBuf, theme: Theme) {
        use crate::config::log_info;
        
        log_info(&format!("Executing Lua script: {:?}", script_path));
        
        // Create new Lua context for each execution
        let lua = match self.create_lua_context() {
            Ok(lua) => lua,
            Err(e) => {
                eprintln!("Failed to create Lua context: {}", e);
                return;
            }
        };
        
        // Set current theme in Lua globals
        if let Err(e) = self.set_theme_info(&lua, theme) {
            eprintln!("Failed to set theme info: {}", e);
            return;
        }
        
        // Read and execute the script
        match std::fs::read_to_string(script_path) {
            Ok(script_content) => {
                if let Err(e) = lua.load(&script_content).exec() {
                    eprintln!("Lua script error: {}", e);
                }
            }
            Err(e) => eprintln!("Failed to read Lua script: {}", e),
        }
    }
    
    fn set_theme_info(&self, lua: &Lua, theme: Theme) -> LuaResult<()> {
        let globals = lua.globals();
        
        // Set individual globals for convenience
        globals.set("THEME", theme.to_string())?;
        globals.set("THEME_UPPER", theme.to_string().to_uppercase())?;
        globals.set("IS_DARK", matches!(theme, Theme::Dark))?;
        globals.set("IS_LIGHT", matches!(theme, Theme::Light))?;
        
        // Update theme_switcher module
        let theme_switcher: mlua::Table = globals.get("theme_switcher")?;
        theme_switcher.set("current_theme", theme.to_string())?;
        theme_switcher.set("is_dark", matches!(theme, Theme::Dark))?;
        theme_switcher.set("is_light", matches!(theme, Theme::Light))?;
        
        Ok(())
    }
}

impl Default for LuaHandler {
    fn default() -> Self {
        Self::new().expect("Failed to create Lua handler")
    }
}

impl ThemeHandler for LuaHandler {
    fn on_theme_change(&self, theme: Theme) {
        // Execute theme-specific scripts
        match theme {
            Theme::Light => {
                for script in &self.light_scripts {
                    self.execute_script(script, theme);
                }
            }
            Theme::Dark => {
                for script in &self.dark_scripts {
                    self.execute_script(script, theme);
                }
            }
        }
        
        // Execute any-change scripts
        for script in &self.any_change_scripts {
            self.execute_script(script, theme);
        }
    }
}