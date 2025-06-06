use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ThemeSwitcherError {
    MonitorError(String),
    PlatformError(String),
}

impl fmt::Display for ThemeSwitcherError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ThemeSwitcherError::MonitorError(msg) => write!(f, "Monitor error: {}", msg),
            ThemeSwitcherError::PlatformError(msg) => write!(f, "Platform error: {}", msg),
        }
    }
}

impl Error for ThemeSwitcherError {}

impl From<Box<dyn Error>> for ThemeSwitcherError {
    fn from(err: Box<dyn Error>) -> Self {
        ThemeSwitcherError::MonitorError(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, ThemeSwitcherError>;
