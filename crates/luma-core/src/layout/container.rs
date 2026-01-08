use crate::{Result, Size, Rect};

/// A widget that can be positioned and sized
pub trait Widget {
    /// Set the bounds (position and size) of the widget
    fn set_bounds(&mut self, bounds: Rect) -> Result<()>;
    
    /// Get the current bounds of the widget
    fn get_bounds(&self) -> Rect;
    
    /// Get the widget's ID
    fn id(&self) -> crate::ids::WidgetId;
}

/// A container that can hold and layout child widgets
pub trait Container {
    /// Perform layout calculation and position all children
    /// 
    /// This is called when the container is resized or children are added/removed
    fn layout(&mut self, available_space: Size) -> Result<()>;
}
