use luma_core::{Result, Point, Size, Rect, WidgetId, Widget, traits::TextInputBackend};
use crate::window::Window;
use crate::Win32TextInput;

/// Cross-platform text input widget
pub struct TextInput {
    backend: Win32TextInput,
    id: WidgetId,
    bounds: Rect,
}

impl TextInput {
    /// Create a text input builder
    pub fn builder() -> TextInputBuilder {
        TextInputBuilder::default()
    }
    
    /// Get the current text
    pub fn get_text(&self) -> Result<String> {
        self.backend.get_text()
    }
    
    /// Set the text
    pub fn set_text(&mut self, text: &str) -> Result<()> {
        self.backend.set_text(text)
    }
    
    /// Set read-only mode
    pub fn set_read_only(&mut self, read_only: bool) -> Result<()> {
        self.backend.set_read_only(read_only)
    }
}

impl Widget for TextInput {
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

/// Builder for creating text inputs
#[derive(Default)]
pub struct TextInputBuilder {
    initial_text: Option<String>,
    position: Option<Point>,
    size: Option<Size>,
    read_only: bool,
}

impl TextInputBuilder {
    /// Create a new text input builder
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set initial text
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.initial_text = Some(text.into());
        self
    }
    
    /// Set the position
    pub fn position(mut self, x: i32, y: i32) -> Self {
        self.position = Some(Point::new(x, y));
        self
    }
    
    /// Set the size
    pub fn size(mut self, width: u32, height: u32) -> Self {
        self.size = Some(Size::new(width, height));
        self
    }
    
    /// Set read-only mode
    pub fn read_only(mut self, read_only: bool) -> Self {
        self.read_only = read_only;
        self
    }
    
    /// Build the text input
    pub fn build(self, parent: &Window) -> Result<TextInput> {
        let pos = self.position.unwrap_or(Point::new(0, 0));
        let size = self.size.unwrap_or(Size::new(200, 24));
        
        let parent_hwnd = parent.raw_handle();
        let backend = Win32TextInput::new(parent_hwnd, pos, size, self.read_only)?;
        
        let mut text_input = TextInput {
            backend,
            id: WidgetId::new(),
            bounds: Rect::from_point_size(pos, size),
        };
        
        // Set initial text if provided
        if let Some(text) = self.initial_text {
            text_input.set_text(&text)?;
        }
        
        Ok(text_input)
    }
}
