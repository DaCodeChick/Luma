use crate::{Result, Point, Size, WindowFlags, ButtonFlags, ListBoxFlags};

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
    
    /// Get the client area size (the drawable area inside the window borders)
    fn get_client_size(&self) -> Result<Size>;
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
    
    /// Set the button bounds (position and size)
    fn set_bounds(&mut self, x: i32, y: i32, width: u32, height: u32) -> Result<()>;
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

/// Platform-specific label backend
pub trait LabelBackend {
    /// Create a new label
    fn new(
        parent_hwnd: *mut std::ffi::c_void,
        text: &str,
        pos: Point,
        size: Size,
    ) -> Result<Self> where Self: Sized;
    
    /// Set the label text
    fn set_text(&mut self, text: &str) -> Result<()>;
    
    /// Set the label bounds (position and size)
    fn set_bounds(&mut self, x: i32, y: i32, width: u32, height: u32) -> Result<()>;
}

/// Platform-specific text input backend
pub trait TextInputBackend {
    /// Create a new text input
    fn new(
        parent_hwnd: *mut std::ffi::c_void,
        pos: Point,
        size: Size,
        read_only: bool,
    ) -> Result<Self> where Self: Sized;
    
    /// Get the current text
    fn get_text(&self) -> Result<String>;
    
    /// Set the text
    fn set_text(&mut self, text: &str) -> Result<()>;
    
    /// Set read-only mode
    fn set_read_only(&mut self, read_only: bool) -> Result<()>;
    
    /// Set the text input bounds (position and size)
    fn set_bounds(&mut self, x: i32, y: i32, width: u32, height: u32) -> Result<()>;
}

/// Platform-specific checkbox backend
pub trait CheckBoxBackend {
    /// Create a new checkbox
    fn new(
        parent_hwnd: *mut std::ffi::c_void,
        label: &str,
        pos: Point,
        size: Size,
        checked: bool,
    ) -> Result<Self> where Self: Sized;
    
    /// Get the checked state
    fn is_checked(&self) -> Result<bool>;
    
    /// Set the checked state
    fn set_checked(&mut self, checked: bool) -> Result<()>;
    
    /// Set the label text
    fn set_label(&mut self, label: &str) -> Result<()>;
    
    /// Set the checkbox bounds (position and size)
    fn set_bounds(&mut self, x: i32, y: i32, width: u32, height: u32) -> Result<()>;
}

/// Platform-specific listbox backend
pub trait ListBoxBackend {
    /// Create a new listbox
    fn new(
        parent_hwnd: *mut std::ffi::c_void,
        pos: Point,
        size: Size,
        flags: ListBoxFlags,
    ) -> Result<Self> where Self: Sized;
    
    /// Add an item to the listbox
    fn add_item(&mut self, item: &str) -> Result<()>;
    
    /// Remove an item by index
    fn remove_item(&mut self, index: usize) -> Result<()>;
    
    /// Clear all items
    fn clear(&mut self) -> Result<()>;
    
    /// Get the number of items
    fn item_count(&self) -> Result<usize>;
    
    /// Get selected index (for single-select)
    fn get_selected_index(&self) -> Result<Option<usize>>;
    
    /// Get selected indices (for multi-select)
    fn get_selected_indices(&self) -> Result<Vec<usize>>;
    
    /// Set selected index (for single-select)
    fn set_selected_index(&mut self, index: Option<usize>) -> Result<()>;
    
    /// Set the listbox bounds (position and size)
    fn set_bounds(&mut self, x: i32, y: i32, width: u32, height: u32) -> Result<()>;
}
