use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    #[serde(default)]
    pub general: GeneralConfig,
    
    #[serde(default)]
    pub scripts: ScriptsConfig,
    
    #[serde(default)]
    pub lua_scripts: LuaScriptsConfig,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GeneralConfig {
    #[serde(default)]
    pub quiet: bool,
    
    #[serde(default)]
    pub log_file: Option<PathBuf>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ScriptsConfig {
    #[serde(default)]
    pub light: Vec<PathBuf>,
    
    #[serde(default)]
    pub dark: Vec<PathBuf>,
    
    #[serde(default)]
    pub any: Vec<PathBuf>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LuaScriptsConfig {
    #[serde(default)]
    pub light: Vec<PathBuf>,
    
    #[serde(default)]
    pub dark: Vec<PathBuf>,
    
    #[serde(default)]
    pub any: Vec<PathBuf>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            general: GeneralConfig::default(),
            scripts: ScriptsConfig::default(),
            lua_scripts: LuaScriptsConfig::default(),
        }
    }
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            quiet: false,
            log_file: None,
        }
    }
}

impl Default for ScriptsConfig {
    fn default() -> Self {
        Self {
            light: Vec::new(),
            dark: Vec::new(),
            any: Vec::new(),
        }
    }
}

impl Default for LuaScriptsConfig {
    fn default() -> Self {
        Self {
            light: Vec::new(),
            dark: Vec::new(),
            any: Vec::new(),
        }
    }
}

impl Config {
    pub fn load_from_file(path: &PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
    
    pub fn has_scripts(&self) -> bool {
        !self.scripts.light.is_empty() || 
        !self.scripts.dark.is_empty() || 
        !self.scripts.any.is_empty()
    }
    
    pub fn has_lua_scripts(&self) -> bool {
        !self.lua_scripts.light.is_empty() || 
        !self.lua_scripts.dark.is_empty() || 
        !self.lua_scripts.any.is_empty()
    }
}