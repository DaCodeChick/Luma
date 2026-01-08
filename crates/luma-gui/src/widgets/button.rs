use luma_core::{Result, Point, Size, ButtonFlags, Rect, WidgetId, Widget, traits::ButtonBackend};
use crate::window::Window;
use crate::Win32Button;

/// Cross-platform button widget
pub struct Button {
    backend: Win32Button,
    id: WidgetId,
    bounds: Rect,
    on_click: Option<Box<dyn FnMut()>>,
}

impl Button {
    /// Create a button builder
    pub fn builder() -> ButtonBuilder {
        ButtonBuilder::default()
    }
    
    /// Set the button label
    pub fn set_label(&mut self, label: &str) -> Result<()> {
        self.backend.set_label(label)
    }
    
    /// Enable or disable the button
    pub fn set_enabled(&mut self, enabled: bool) -> Result<()> {
        self.backend.set_enabled(enabled)
    }
    
    /// Get the backend HWND (for callback registration)
    pub(crate) fn hwnd(&self) -> isize {
        self.backend.hwnd().0
    }
}

impl Widget for Button {
    fn set_bounds(&mut self, bounds: Rect) -> Result<()> {
        self.bounds = bounds;
        self.backend.set_bounds(bounds.x, bounds.y, bounds.width, bounds.height)?;
        Ok(())
    }
    
    fn get_bounds(&self) -> Rect {
        self.bounds
    }
    
    fn id(&self) -> WidgetId {
        self.id
    }
}

impl Drop for Button {
    fn drop(&mut self) {
        // Unregister callback before widget is destroyed
        if self.on_click.is_some() {
            crate::unregister_callback(self.hwnd());
        }
    }
}

/// Builder for creating buttons
#[derive(Default)]
pub struct ButtonBuilder {
    label: Option<String>,
    position: Option<Point>,
    size: Option<Size>,
    flags: Option<ButtonFlags>,
    on_click: Option<Box<dyn FnMut()>>,
}

impl ButtonBuilder {
    /// Create a new button builder
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set the button label
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }
    
    /// Set the button position
    pub fn position(mut self, x: i32, y: i32) -> Self {
        self.position = Some(Point::new(x, y));
        self
    }
    
    /// Set the button size
    pub fn size(mut self, width: u32, height: u32) -> Self {
        self.size = Some(Size::new(width, height));
        self
    }
    
    /// Set button flags
    pub fn flags(mut self, flags: ButtonFlags) -> Self {
        self.flags = Some(flags);
        self
    }
    
    /// Set the click callback
    pub fn on_click<F>(mut self, callback: F) -> Self
    where
        F: FnMut() + 'static,
    {
        self.on_click = Some(Box::new(callback));
        self
    }
    
    /// Build the button
    pub fn build(self, parent: &Window) -> Result<Button> {
        let label = self.label.as_deref().unwrap_or("Button");
        let pos = self.position.unwrap_or(Point::new(0, 0));
        let size = self.size.unwrap_or(Size::new(100, 30));
        let flags = self.flags.unwrap_or_default();
        
        let parent_hwnd = parent.raw_handle();
        let backend = Win32Button::new(parent_hwnd, label, pos, size, flags)?;
        
        let mut button = Button {
            backend,
            id: WidgetId::new(),
            bounds: Rect::from_point_size(pos, size),
            on_click: self.on_click,
        };
        
        // Register callback if present
        if button.on_click.is_some() {
            let callback_ptr = button.on_click.as_mut().unwrap().as_mut() as *mut dyn FnMut();
            crate::register_callback(button.hwnd(), callback_ptr);
        }
        
        Ok(button)
    }
}
