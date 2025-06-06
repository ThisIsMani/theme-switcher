#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "macos")]
pub use macos::{MacOSThemeMonitor, MacOSApplication};

use crate::error::Result;
use crate::handlers::ThemeHandler;
use crate::app::Application;
use std::sync::Arc;

/// Factory function to create platform-specific application
pub fn create_application(handler: Arc<dyn ThemeHandler>) -> Result<Box<dyn Application>> {
    #[cfg(target_os = "macos")]
    {
        Ok(Box::new(MacOSApplication::new(handler)?))
    }
    
    #[cfg(not(target_os = "macos"))]
    {
        use crate::error::ThemeSwitcherError;
        Err(ThemeSwitcherError::PlatformError(
            "This platform is not currently supported".to_string()
        ))
    }
}