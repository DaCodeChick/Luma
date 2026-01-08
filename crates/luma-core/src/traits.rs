use crate::{Result, Point, Size, WindowFlags, ButtonFlags};

/// Platform-specific application backend
pub trait ApplicationBackend {
    /// Initialize a new application instance
    fn new() -> Result<Self> where Self: Sized;
    
    /// Run the application event loop
    /// 
    /// This blocks until the application quits
    fn run(&mut self) -> Result<()>;
    
    /// Quit the application
    fn quit(&mut self) -> Result<()>;
}

/// Platform-specific window backend
pub trait WindowBackend {
    /// Create a new window
    fn new(title: &str, width: u32, height: u32, flags: WindowFlags) -> Result<Self> 
        where Self: Sized;
    
    /// Set the window title
    fn set_title(&mut self, title: &str) -> Result<()>;
    
    /// Set the window size
    fn set_size(&mut self, width: u32, height: u32) -> Result<()>;
    
    /// Show the window
    fn show(&mut self) -> Result<()>;
    
    /// Hide the window
    fn hide(&mut self) -> Result<()>;
    
    /// Get the raw window handle (for creating child widgets)
    fn raw_handle(&self) -> *mut std::ffi::c_void;
}

/// Platform-specific button backend
pub trait ButtonBackend {
    /// Create a new button
    fn new(
        parent_hwnd: *mut std::ffi::c_void,
        label: &str,
        pos: Point,
        size: Size,
        flags: ButtonFlags
    ) -> Result<Self> where Self: Sized;
    
    /// Set the button label
    fn set_label(&mut self, label: &str) -> Result<()>;
    
    /// Enable or disable the button
    fn set_enabled(&mut self, enabled: bool) -> Result<()>;
}

/// Platform-specific panel (container) backend
pub trait PanelBackend {
    /// Create a new panel
    fn new(
        parent_hwnd: *mut std::ffi::c_void,
        pos: Point,
        size: Size,
    ) -> Result<Self> where Self: Sized;
    
    /// Get the raw panel handle (for creating child widgets)
    fn raw_handle(&self) -> *mut std::ffi::c_void;
    
    /// Set the panel bounds
    fn set_bounds(&mut self, x: i32, y: i32, width: u32, height: u32) -> Result<()>;
}
