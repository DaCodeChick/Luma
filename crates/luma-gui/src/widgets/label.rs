use luma_core::{Result, Point, Size, Rect, WidgetId, Widget, traits::LabelBackend};
use crate::window::Window;
use crate::Win32Label;

/// Cross-platform label widget
pub struct Label {
    backend: Win32Label,
    id: WidgetId,
    bounds: Rect,
}

impl Label {
    /// Create a label builder
    pub fn builder() -> LabelBuilder {
        LabelBuilder::default()
    }
    
    /// Set the label text
    pub fn set_text(&mut self, text: &str) -> Result<()> {
        self.backend.set_text(text)
    }
}

impl Widget for Label {
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

/// Builder for creating labels
#[derive(Default)]
pub struct LabelBuilder {
    text: Option<String>,
    position: Option<Point>,
    size: Option<Size>,
}

impl LabelBuilder {
    /// Create a new label builder
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set the label text
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }
    
    /// Set the label position
    pub fn position(mut self, x: i32, y: i32) -> Self {
        self.position = Some(Point::new(x, y));
        self
    }
    
    /// Set the label size
    pub fn size(mut self, width: u32, height: u32) -> Self {
        self.size = Some(Size::new(width, height));
        self
    }
    
    /// Build the label
    pub fn build(self, parent: &Window) -> Result<Label> {
        let text = self.text.as_deref().unwrap_or("Label");
        let pos = self.position.unwrap_or(Point::new(0, 0));
        let size = self.size.unwrap_or(Size::new(100, 20));
        
        let parent_hwnd = parent.raw_handle();
        let backend = Win32Label::new(parent_hwnd, text, pos, size)?;
        
        Ok(Label {
            backend,
            id: WidgetId::new(),
            bounds: Rect::from_point_size(pos, size),
        })
    }
}
