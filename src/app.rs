use crate::error::Result;
use crate::ThemeMonitor;

/// Platform-agnostic application trait that can be implemented for different operating systems
pub trait Application {
    /// Run the application event loop
    fn run(self: Box<Self>) -> Result<()>;
}

/// Generic application runner that uses the platform-specific implementation
pub struct ApplicationRunner<M: ThemeMonitor> {
    monitor: M,
}

impl<M: ThemeMonitor> ApplicationRunner<M> {
    pub fn new(monitor: M) -> Self {
        Self { monitor }
    }
    
    pub fn run(self) -> Result<()> {
        println!("Starting theme monitor...");
        
        // Print initial theme
        println!("Current theme: {}", self.monitor.get_current_theme());
        
        // Start monitoring
        self.monitor.start()?;
        println!("Monitoring for theme changes. Press Ctrl+C to stop.");
        
        // Note: The actual event loop will be handled by platform-specific implementations
        // This is just the common setup logic
        
        Ok(())
    }
}