use luma_core::{Result, Point, Size, Rect, WidgetId, Widget, traits::CheckBoxBackend};
use crate::window::Window;
use crate::Win32CheckBox;

/// Cross-platform checkbox widget
pub struct CheckBox {
    backend: Win32CheckBox,
    id: WidgetId,
    bounds: Rect,
    on_checked_changed: Option<Box<dyn FnMut(bool)>>,
}

impl CheckBox {
    /// Create a checkbox builder
    pub fn builder() -> CheckBoxBuilder {
        CheckBoxBuilder::default()
    }
    
    /// Get the checked state
    pub fn is_checked(&self) -> Result<bool> {
        self.backend.is_checked()
    }
    
    /// Set the checked state
    pub fn set_checked(&mut self, checked: bool) -> Result<()> {
        self.backend.set_checked(checked)
    }
    
    /// Set the label text
    pub fn set_label(&mut self, label: &str) -> Result<()> {
        self.backend.set_label(label)
    }
}

impl Widget for CheckBox {
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

/// Builder for creating checkboxes
#[derive(Default)]
pub struct CheckBoxBuilder {
    label: Option<String>,
    position: Option<Point>,
    size: Option<Size>,
    checked: bool,
    on_checked_changed: Option<Box<dyn FnMut(bool)>>,
}

impl CheckBoxBuilder {
    /// Create a new checkbox builder
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set the checkbox label
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
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
    
    /// Set initial checked state
    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }
    
    /// Set the checked changed callback
    pub fn on_checked_changed<F>(mut self, callback: F) -> Self
    where
        F: FnMut(bool) + 'static,
    {
        self.on_checked_changed = Some(Box::new(callback));
        self
    }
    
    /// Build the checkbox
    pub fn build(self, parent: &Window) -> Result<CheckBox> {
        let label = self.label.as_deref().unwrap_or("Checkbox");
        let pos = self.position.unwrap_or(Point::new(0, 0));
        let size = self.size.unwrap_or(Size::new(150, 20));
        
        let parent_hwnd = parent.raw_handle();
        let backend = Win32CheckBox::new(parent_hwnd, label, pos, size, self.checked)?;
        
        Ok(CheckBox {
            backend,
            id: WidgetId::new(),
            bounds: Rect::from_point_size(pos, size),
            on_checked_changed: self.on_checked_changed,
        })
    }
}
