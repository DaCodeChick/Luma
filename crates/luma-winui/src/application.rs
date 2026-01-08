//! WinUI application and message loop.

use crate::error::Result;

/// WinUI application.
pub struct WinUIApplication {
    // TODO: Add application state
}

impl WinUIApplication {
    /// Create a new WinUI application.
    pub fn new() -> Result<Self> {
        todo!("WinUIApplication not yet implemented")
    }

    /// Run the application message loop.
    pub fn run(&self) -> Result<()> {
        todo!("WinUIApplication::run not yet implemented")
    }
}

impl Default for WinUIApplication {
    fn default() -> Self {
        Self::new().expect("Failed to create WinUI application")
    }
}
