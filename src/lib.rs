#![allow(unexpected_cfgs)]

use std::error::Error;
use std::fmt;
use std::sync::Arc;

pub mod app;
pub mod cli;
pub mod config;
pub mod config_file;
pub mod error;
pub mod handlers;
pub mod lua_handler;
pub mod platform;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    Light,
    Dark,
}

impl fmt::Display for Theme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Theme::Light => write!(f, "light"),
            Theme::Dark => write!(f, "dark"),
        }
    }
}

pub trait ThemeMonitor {
    fn start(&self) -> Result<(), Box<dyn Error>>;
    fn stop(&self) -> Result<(), Box<dyn Error>>;
    fn get_current_theme(&self) -> Theme;
}

pub fn run() -> Result<(), Box<dyn Error>> {
    use clap::Parser;
    use cli::Args;
    use handlers::{CompositeThemeHandler, LoggingThemeHandler, ScriptHandler};
    use lua_handler::LuaHandler;
    use config_file::Config;
    
    let args = Args::parse();
    
    // Load config file if specified
    let config = if let Some(ref config_path) = args.config {
        config::log_info(&format!("Loading config from: {:?}", config_path));
        Some(Config::load_from_file(config_path)?)
    } else {
        // Try to load from default location
        let config_path = dirs::home_dir()
            .map(|home| home.join(".config/theme-switcher/config.toml"))
            .unwrap_or_default();
        
        if config_path.exists() {
            config::log_info(&format!("Loading config from: {:?}", config_path));
            Config::load_from_file(&config_path).ok()
        } else {
            None
        }
    };
    
    // Set quiet mode globally (command line takes precedence)
    let quiet = args.quiet || config.as_ref().map(|c| c.general.quiet).unwrap_or(false);
    config::set_quiet_mode(quiet);
    
    let mut composite = CompositeThemeHandler::new();
    
    // Add logging handler unless in quiet mode
    if !quiet {
        composite.add_handler(Arc::new(LoggingThemeHandler));
    }
    
    // Create script handler combining CLI args and config
    let mut script_handler = ScriptHandler::new();
    let mut has_scripts = false;
    
    // Add scripts from command line
    if let Some(ref light_script) = args.light_script {
        script_handler = script_handler.with_light_script(light_script.clone());
        has_scripts = true;
    }
    if let Some(ref dark_script) = args.dark_script {
        script_handler = script_handler.with_dark_script(dark_script.clone());
        has_scripts = true;
    }
    if let Some(ref any_script) = args.any_script {
        script_handler = script_handler.with_any_change_script(any_script.clone());
        has_scripts = true;
    }
    
    // Add scripts from config file
    if let Some(ref cfg) = config {
        if !cfg.scripts.light.is_empty() {
            script_handler = script_handler.with_light_scripts(cfg.scripts.light.clone());
            has_scripts = true;
        }
        if !cfg.scripts.dark.is_empty() {
            script_handler = script_handler.with_dark_scripts(cfg.scripts.dark.clone());
            has_scripts = true;
        }
        if !cfg.scripts.any.is_empty() {
            script_handler = script_handler.with_any_change_scripts(cfg.scripts.any.clone());
            has_scripts = true;
        }
    }
    
    if has_scripts {
        composite.add_handler(Arc::new(script_handler));
    }
    
    // Create Lua handler combining CLI args and config
    let mut lua_handler = LuaHandler::new()?;
    let mut has_lua_scripts = false;
    
    // Add Lua scripts from command line
    if let Some(lua_light) = args.lua_light {
        lua_handler = lua_handler.with_light_script(lua_light);
        has_lua_scripts = true;
    }
    if let Some(lua_dark) = args.lua_dark {
        lua_handler = lua_handler.with_dark_script(lua_dark);
        has_lua_scripts = true;
    }
    if let Some(lua_any) = args.lua_any {
        lua_handler = lua_handler.with_any_change_script(lua_any);
        has_lua_scripts = true;
    }
    
    // Add Lua scripts from config file
    if let Some(ref cfg) = config {
        if !cfg.lua_scripts.light.is_empty() {
            lua_handler = lua_handler.with_light_scripts(cfg.lua_scripts.light.clone());
            has_lua_scripts = true;
        }
        if !cfg.lua_scripts.dark.is_empty() {
            lua_handler = lua_handler.with_dark_scripts(cfg.lua_scripts.dark.clone());
            has_lua_scripts = true;
        }
        if !cfg.lua_scripts.any.is_empty() {
            lua_handler = lua_handler.with_any_change_scripts(cfg.lua_scripts.any.clone());
            has_lua_scripts = true;
        }
    }
    
    if has_lua_scripts {
        composite.add_handler(Arc::new(lua_handler));
    }
    
    let app = platform::create_application(Arc::new(composite))?;
    app.run()?;

    Ok(())
}
