use crate::Theme;
use std::sync::Arc;
use std::process::Command;
use std::path::PathBuf;

pub trait ThemeHandler: Send + Sync {
    fn on_theme_change(&self, theme: Theme);
}

pub struct LoggingThemeHandler;

impl ThemeHandler for LoggingThemeHandler {
    fn on_theme_change(&self, theme: Theme) {
        println!("Theme changed to: {}", theme);
        match theme {
            Theme::Light => {
                println!("Executing light theme actions...");
                // TODO: Execute light theme commands
            }
            Theme::Dark => {
                println!("Executing dark theme actions...");
                // TODO: Execute dark theme commands
            }
        }
    }
}

pub struct CompositeThemeHandler {
    handlers: Vec<Arc<dyn ThemeHandler>>,
}

impl CompositeThemeHandler {
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
        }
    }

    pub fn add_handler(&mut self, handler: Arc<dyn ThemeHandler>) {
        self.handlers.push(handler);
    }
}

impl Default for CompositeThemeHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl ThemeHandler for CompositeThemeHandler {
    fn on_theme_change(&self, theme: Theme) {
        for handler in &self.handlers {
            handler.on_theme_change(theme);
        }
    }
}

/// Handler that executes shell scripts when theme changes
pub struct ScriptHandler {
    light_scripts: Vec<PathBuf>,
    dark_scripts: Vec<PathBuf>,
    any_change_scripts: Vec<PathBuf>,
}

impl ScriptHandler {
    pub fn new() -> Self {
        Self {
            light_scripts: Vec::new(),
            dark_scripts: Vec::new(),
            any_change_scripts: Vec::new(),
        }
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
        
        log_info(&format!("Executing script: {:?}", script_path));
        
        let result = Command::new("sh")
            .arg("-c")
            .arg(script_path.to_string_lossy().as_ref())
            .env("THEME_SWITCHER_THEME", theme.to_string())
            .env("THEME_SWITCHER_THEME_UPPER", theme.to_string().to_uppercase())
            .spawn();
            
        match result {
            Ok(mut child) => {
                // Don't wait for the script to complete - run it in background
                match child.wait() {
                    Ok(status) => {
                        if !status.success() {
                            eprintln!("Script exited with non-zero status: {:?}", status);
                        }
                    }
                    Err(e) => eprintln!("Failed to wait for script: {}", e),
                }
            }
            Err(e) => eprintln!("Failed to execute script: {}", e),
        }
    }
}

impl Default for ScriptHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl ThemeHandler for ScriptHandler {
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
