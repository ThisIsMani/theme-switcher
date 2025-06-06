use std::error::Error;
use std::sync::Arc;
use tokio::runtime::Runtime;
use crate::{cli::Args, config, config_file::Config, handlers::*, ipc::*, lua_handler::LuaHandler};
use crate::app::Application;

pub fn run_with_tokio(args: Args) -> Result<(), Box<dyn Error>> {
    // Create tokio runtime
    let runtime = Runtime::new()?;
    
    runtime.block_on(async {
        run_async(args).await
    })
}

async fn run_async(args: Args) -> Result<(), Box<dyn Error>> {
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
    
    // Setup IPC if requested
    let ipc_server = if args.ipc {
        let server = IpcServer::new()?;
        server.start().await?;
        
        // Add IPC handler to composite
        composite.add_handler(Arc::new(IpcHandler::new(
            server.get_broadcaster(),
            server.get_current_theme_state()
        )));
        
        Some(server)
    } else {
        None
    };
    
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
    
    // Create the app with handlers
    let handler = Arc::new(composite);
    
    // Create the application
    #[cfg(target_os = "macos")]
    {
        use crate::platform::MacOSApplication;
        let app = MacOSApplication::new(handler)?;
        
        // If IPC is enabled, set initial theme
        if let Some(ref server) = ipc_server {
            let current_theme = app.get_current_theme();
            server.set_current_theme(current_theme);
        }
        
        // Run the app
        Box::new(app).run()?;
        
        // Cleanup IPC if it was started
        if let Some(ref server) = ipc_server {
            server.cleanup();
        }
    }
    
    #[cfg(not(target_os = "macos"))]
    {
        use crate::error::ThemeSwitcherError;
        return Err(Box::new(ThemeSwitcherError::PlatformError(
            "This platform is not currently supported".to_string()
        )));
    }
    
    Ok(())
}