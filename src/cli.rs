use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Script to run when switching to light theme
    #[arg(short, long)]
    pub light_script: Option<PathBuf>,
    
    /// Script to run when switching to dark theme
    #[arg(short, long)]
    pub dark_script: Option<PathBuf>,
    
    /// Script to run on any theme change
    #[arg(short, long)]
    pub any_script: Option<PathBuf>,
    
    /// Lua script to run when switching to light theme
    #[arg(long)]
    pub lua_light: Option<PathBuf>,
    
    /// Lua script to run when switching to dark theme
    #[arg(long)]
    pub lua_dark: Option<PathBuf>,
    
    /// Lua script to run on any theme change
    #[arg(long)]
    pub lua_any: Option<PathBuf>,
    
    /// Path to configuration file
    #[arg(short, long)]
    pub config: Option<PathBuf>,
    
    /// Run in quiet mode (suppress informational output)
    #[arg(short, long)]
    pub quiet: bool,
    
    /// Enable IPC server for Neovim integration
    #[arg(long)]
    pub ipc: bool,
}

impl Args {
    pub fn has_scripts(&self) -> bool {
        self.light_script.is_some() || self.dark_script.is_some() || self.any_script.is_some()
    }
    
    pub fn has_lua_scripts(&self) -> bool {
        self.lua_light.is_some() || self.lua_dark.is_some() || self.lua_any.is_some()
    }
}