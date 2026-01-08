use crate::{Result, Size, Rect};
use super::{Container, LayoutConstraints, Widget};

/// Layout direction for BoxLayout
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutDirection {
    /// Arrange children horizontally (left to right)
    Horizontal,
    /// Arrange children vertically (top to bottom)
    Vertical,
}

/// A box layout that arranges widgets in a single row or column
/// 
/// Similar to Java Swing's BoxLayout or CSS Flexbox (single direction)
pub struct BoxLayout {
    direction: LayoutDirection,
    gap: u32,
    children: Vec<(Box<dyn Widget>, LayoutConstraints)>,
}

impl BoxLayout {
    /// Create a new horizontal BoxLayout
    pub fn horizontal() -> Self {
        Self {
            direction: LayoutDirection::Horizontal,
            gap: 0,
            children: Vec::new(),
        }
    }
    
    /// Create a new vertical BoxLayout
    pub fn vertical() -> Self {
        Self {
            direction: LayoutDirection::Vertical,
            gap: 0,
            children: Vec::new(),
        }
    }
    
    /// Set the gap between children
    pub fn with_gap(mut self, gap: u32) -> Self {
        self.gap = gap;
        self
    }
    
    /// Add a child widget with constraints
    pub fn add(&mut self, widget: Box<dyn Widget>, constraints: LayoutConstraints) {
        self.children.push((widget, constraints));
    }
    
    /// Get the number of children
    pub fn child_count(&self) -> usize {
        self.children.len()
    }
}

impl Container for BoxLayout {
    fn layout(&mut self, available_space: Size) -> Result<()> {
        if self.children.is_empty() {
            return Ok(());
        }
        
        match self.direction {
            LayoutDirection::Horizontal => self.layout_horizontal(available_space),
            LayoutDirection::Vertical => self.layout_vertical(available_space),
        }
    }
}

impl BoxLayout {
    fn layout_vertical(&mut self, available: Size) -> Result<()> {
        tracing::debug!(
            "BoxLayout::layout_vertical: {} children, available space: {}x{}",
            self.children.len(),
            available.width,
            available.height
        );
        
        // Phase 1: Calculate sizes
        let mut total_fixed_height = 0u32;
        let mut expand_count = 0u32;
        
        for (_, constraints) in &self.children {
            if constraints.expand_vertical {
                expand_count += 1;
            } else {
                let height = constraints.preferred_height.unwrap_or(30);
                total_fixed_height += height + constraints.padding.vertical();
            }
        }
        
        // Calculate remaining space for expanding children
        let total_gaps = self.gap * (self.children.len().saturating_sub(1) as u32);
        let available_height = available.height.saturating_sub(total_gaps);
        let remaining_height = available_height.saturating_sub(total_fixed_height);
        let expand_height = if expand_count > 0 {
            remaining_height / expand_count
        } else {
            0
        };
        
        tracing::debug!(
            "Layout calc: total_fixed={}, expand_count={}, expand_height={}",
            total_fixed_height,
            expand_count,
            expand_height
        );
        
        // Phase 2: Position widgets
        let mut y = 0i32;
        
        for (widget, constraints) in &mut self.children {
            // Calculate widget height
            let widget_height = if constraints.expand_vertical {
                expand_height.saturating_sub(constraints.padding.vertical())
            } else {
                constraints.preferred_height.unwrap_or(30)
            };
            
            // Calculate widget width
            let widget_width = if constraints.expand_horizontal {
                available.width.saturating_sub(constraints.padding.horizontal())
            } else {
                constraints.preferred_width.unwrap_or(available.width.saturating_sub(constraints.padding.horizontal()))
            };
            
            // Apply padding
            let padding = constraints.padding;
            let content_x = padding.left as i32;
            let content_y = y + padding.top as i32;
            
            // Create bounds
            let bounds = Rect::new(content_x, content_y, widget_width, widget_height);
            
            tracing::debug!(
                "Positioning widget at ({}, {}) with size {}x{}",
                bounds.x,
                bounds.y,
                bounds.width,
                bounds.height
            );
            
            widget.set_bounds(bounds)?;
            
            // Move to next position
            y += widget_height as i32 + padding.vertical() as i32 + self.gap as i32;
        }
        
        Ok(())
    }
    
    fn layout_horizontal(&mut self, available: Size) -> Result<()> {
        tracing::debug!(
            "BoxLayout::layout_horizontal: {} children, available space: {}x{}",
            self.children.len(),
            available.width,
            available.height
        );
        
        // Phase 1: Calculate sizes
        let mut total_fixed_width = 0u32;
        let mut expand_count = 0u32;
        
        for (_, constraints) in &self.children {
            if constraints.expand_horizontal {
                expand_count += 1;
            } else {
                let width = constraints.preferred_width.unwrap_or(100);
                total_fixed_width += width + constraints.padding.horizontal();
            }
        }
        
        // Calculate remaining space for expanding children
        let total_gaps = self.gap * (self.children.len().saturating_sub(1) as u32);
        let available_width = available.width.saturating_sub(total_gaps);
        let remaining_width = available_width.saturating_sub(total_fixed_width);
        let expand_width = if expand_count > 0 {
            remaining_width / expand_count
        } else {
            0
        };
        
        // Phase 2: Position widgets
        let mut x = 0i32;
        
        for (widget, constraints) in &mut self.children {
            // Calculate widget width
            let widget_width = if constraints.expand_horizontal {
                expand_width.saturating_sub(constraints.padding.horizontal())
            } else {
                constraints.preferred_width.unwrap_or(100)
            };
            
            // Calculate widget height
            let widget_height = if constraints.expand_vertical {
                available.height.saturating_sub(constraints.padding.vertical())
            } else {
                constraints.preferred_height.unwrap_or(available.height.saturating_sub(constraints.padding.vertical()))
            };
            
            // Apply padding
            let padding = constraints.padding;
            let content_x = x + padding.left as i32;
            let content_y = padding.top as i32;
            
            // Create bounds
            let bounds = Rect::new(content_x, content_y, widget_width, widget_height);
            
            widget.set_bounds(bounds)?;
            
            // Move to next position
            x += widget_width as i32 + padding.horizontal() as i32 + self.gap as i32;
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::WidgetId;
    
    // Mock widget for testing
    struct MockWidget {
        id: WidgetId,
        bounds: Rect,
    }
    
    impl Widget for MockWidget {
        fn set_bounds(&mut self, bounds: Rect) -> Result<()> {
            self.bounds = bounds;
            Ok(())
        }
        
        fn get_bounds(&self) -> Rect {
            self.bounds
        }
        
        fn id(&self) -> WidgetId {
            self.id
        }
    }
    
    #[test]
    fn test_vertical_layout() {
        let mut layout = BoxLayout::vertical();
        
        let widget1 = Box::new(MockWidget {
            id: WidgetId::new(),
            bounds: Rect::default(),
        });
        let widget2 = Box::new(MockWidget {
            id: WidgetId::new(),
            bounds: Rect::default(),
        });
        
        layout.add(widget1, LayoutConstraints::default().preferred_height(50));
        layout.add(widget2, LayoutConstraints::default().preferred_height(50));
        
        let available = Size::new(200, 200);
        layout.layout(available).unwrap();
        
        assert_eq!(layout.child_count(), 2);
    }
}
