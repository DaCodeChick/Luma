use luma_core::{Result, Point, WindowFlags, traits::WindowBackend, Rect, Container, WidgetId, Widget};
use crate::Win32Window;

/// Cross-platform window
pub struct Window {
    pub(crate) backend: Win32Window,
    id: WidgetId,
    layout: Option<Box<dyn Container>>,
}

impl Window {
    /// Create a window builder
    pub fn builder() -> WindowBuilder {
        WindowBuilder::default()
    }
    
    /// Show the window
    pub fn show(&mut self) -> Result<()> {
        self.backend.show()
    }
    
    /// Hide the window
    pub fn hide(&mut self) -> Result<()> {
        self.backend.hide()
    }
    
    /// Set the window title
    pub fn set_title(&mut self, title: &str) -> Result<()> {
        self.backend.set_title(title)
    }
    
    /// Set the window size
    pub fn set_size(&mut self, width: u32, height: u32) -> Result<()> {
        self.backend.set_size(width, height)
    }
    
    /// Set the layout for this window
    pub fn set_layout(&mut self, mut layout: BoxLayout) -> Result<()> {
        // Trigger initial layout with actual client area size
        let size = self.backend.get_client_size()?;
        layout.layout(size)?;
        
        // Store layout in the window
        self.layout = Some(Box::new(layout));
        
        // Register the layout pointer with the Win32 backend for resize handling
        // SAFETY: The layout lives as long as the Window, and we unregister on drop
        let layout_ptr = self.layout.as_mut().unwrap().as_mut() as *mut dyn Container;
        self.backend.set_layout_ptr(layout_ptr);
        
        Ok(())
    }
    
    /// Get the window ID
    pub fn id(&self) -> WidgetId {
        self.id
    }
    
    /// Get the raw window handle (for creating child widgets)
    pub(crate) fn raw_handle(&self) -> *mut std::ffi::c_void {
        self.backend.raw_handle()
    }
}

// Import BoxLayout here to avoid circular dependency
use luma_core::BoxLayout;

/// Builder for creating windows
#[derive(Default)]
pub struct WindowBuilder {
    title: Option<String>,
    width: Option<u32>,
    height: Option<u32>,
    position: Option<Point>,
    flags: Option<WindowFlags>,
}

impl WindowBuilder {
    /// Create a new window builder
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set the window title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
    
    /// Set the window size
    pub fn size(mut self, width: u32, height: u32) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }
    
    /// Set the window position
    pub fn position(mut self, x: i32, y: i32) -> Self {
        self.position = Some(Point::new(x, y));
        self
    }
    
    /// Set window flags
    pub fn flags(mut self, flags: WindowFlags) -> Self {
        self.flags = Some(flags);
        self
    }
    
    /// Make the window resizable or not
    pub fn resizable(mut self, resizable: bool) -> Self {
        let mut flags = self.flags.unwrap_or_default();
        flags.set(WindowFlags::RESIZABLE, resizable);
        self.flags = Some(flags);
        self
    }
    
    /// Build the window
    pub fn build(self) -> Result<Window> {
        let title = self.title.as_deref().unwrap_or("Window");
        let width = self.width.unwrap_or(800);
        let height = self.height.unwrap_or(600);
        let flags = self.flags.unwrap_or_default();
        
        let backend = Win32Window::new(title, width, height, flags)?;
        
        Ok(Window {
            backend,
            id: WidgetId::new(),
            layout: None,
        })
    }
}

impl Widget for Window {
    fn set_bounds(&mut self, _bounds: Rect) -> Result<()> {
        // Windows don't have bounds set from outside
        Ok(())
    }
    
    fn get_bounds(&self) -> Rect {
        // TODO: Get actual window client area
        Rect::new(0, 0, 800, 600)
    }
    
    fn id(&self) -> WidgetId {
        self.id
    }
}
