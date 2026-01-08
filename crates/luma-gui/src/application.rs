use luma_core::{Result, traits::ApplicationBackend};
use crate::Win32Application;

/// Cross-platform application instance
/// 
/// This is the entry point for all Luma applications.
pub struct Application {
    backend: Win32Application,
}

impl Application {
    /// Create a new application instance
    /// 
    /// # Example
    /// 
    /// ```no_run
    /// use luma_gui::Application;
    /// 
    /// let app = Application::new()?;
    /// # Ok::<(), luma_gui::Error>(())
    /// ```
    pub fn new() -> Result<Self> {
        Ok(Self {
            backend: Win32Application::new()?,
        })
    }
    
    /// Run the application event loop
    /// 
    /// This blocks until the application quits.
    /// 
    /// # Example
    /// 
    /// ```no_run
    /// use luma_gui::Application;
    /// 
    /// let mut app = Application::new()?;
    /// app.run()?;
    /// # Ok::<(), luma_gui::Error>(())
    /// ```
    pub fn run(&mut self) -> Result<()> {
        self.backend.run()
    }
    
    /// Quit the application
    pub fn quit(&mut self) -> Result<()> {
        self.backend.quit()
    }
}
