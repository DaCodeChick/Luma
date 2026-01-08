/// Alignment options for widgets within their allocated space
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alignment {
    /// Align to the start (left or top)
    Start,
    /// Center alignment
    Center,
    /// Align to the end (right or bottom)
    End,
    /// Stretch to fill available space
    Fill,
}

impl Default for Alignment {
    fn default() -> Self {
        Self::Fill
    }
}

/// Padding around a widget
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Padding {
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
    pub left: u32,
}

impl Padding {
    /// No padding
    pub fn zero() -> Self {
        Self { top: 0, right: 0, bottom: 0, left: 0 }
    }
    
    /// Same padding on all sides
    pub fn all(value: u32) -> Self {
        Self { top: value, right: value, bottom: value, left: value }
    }
    
    /// Symmetric padding (vertical, horizontal)
    pub fn symmetric(vertical: u32, horizontal: u32) -> Self {
        Self {
            top: vertical,
            right: horizontal,
            bottom: vertical,
            left: horizontal,
        }
    }
    
    /// Custom padding for each side
    pub fn new(top: u32, right: u32, bottom: u32, left: u32) -> Self {
        Self { top, right, bottom, left }
    }
    
    /// Total horizontal padding (left + right)
    pub fn horizontal(&self) -> u32 {
        self.left + self.right
    }
    
    /// Total vertical padding (top + bottom)
    pub fn vertical(&self) -> u32 {
        self.top + self.bottom
    }
}

impl Default for Padding {
    fn default() -> Self {
        Self::zero()
    }
}

/// Layout constraints for a widget within a container
#[derive(Debug, Clone, Copy)]
pub struct LayoutConstraints {
    pub min_width: Option<u32>,
    pub max_width: Option<u32>,
    pub min_height: Option<u32>,
    pub max_height: Option<u32>,
    pub preferred_width: Option<u32>,
    pub preferred_height: Option<u32>,
    pub expand_horizontal: bool,
    pub expand_vertical: bool,
    pub alignment: Alignment,
    pub padding: Padding,
}

impl Default for LayoutConstraints {
    fn default() -> Self {
        Self {
            min_width: None,
            max_width: None,
            min_height: None,
            max_height: None,
            preferred_width: None,
            preferred_height: None,
            expand_horizontal: false,
            expand_vertical: false,
            alignment: Alignment::Fill,
            padding: Padding::zero(),
        }
    }
}

impl LayoutConstraints {
    /// Create new constraints with default values
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set preferred width
    pub fn preferred_width(mut self, width: u32) -> Self {
        self.preferred_width = Some(width);
        self
    }
    
    /// Set preferred height
    pub fn preferred_height(mut self, height: u32) -> Self {
        self.preferred_height = Some(height);
        self
    }
    
    /// Set whether to expand horizontally
    pub fn expand_horizontal(mut self, expand: bool) -> Self {
        self.expand_horizontal = expand;
        self
    }
    
    /// Set whether to expand vertically
    pub fn expand_vertical(mut self, expand: bool) -> Self {
        self.expand_vertical = expand;
        self
    }
    
    /// Set both expand flags to the same value
    pub fn expand_both(mut self, expand: bool) -> Self {
        self.expand_horizontal = expand;
        self.expand_vertical = expand;
        self
    }
    
    /// Set padding
    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }
    
    /// Set alignment
    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }
    
    /// Set minimum width
    pub fn min_width(mut self, width: u32) -> Self {
        self.min_width = Some(width);
        self
    }
    
    /// Set maximum width
    pub fn max_width(mut self, width: u32) -> Self {
        self.max_width = Some(width);
        self
    }
    
    /// Set minimum height
    pub fn min_height(mut self, height: u32) -> Self {
        self.min_height = Some(height);
        self
    }
    
    /// Set maximum height
    pub fn max_height(mut self, height: u32) -> Self {
        self.max_height = Some(height);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_padding() {
        let padding = Padding::all(10);
        assert_eq!(padding.horizontal(), 20);
        assert_eq!(padding.vertical(), 20);
    }
    
    #[test]
    fn test_constraints_builder() {
        let constraints = LayoutConstraints::default()
            .preferred_width(100)
            .preferred_height(50)
            .expand_horizontal(true);
        
        assert_eq!(constraints.preferred_width, Some(100));
        assert_eq!(constraints.preferred_height, Some(50));
        assert!(constraints.expand_horizontal);
        assert!(!constraints.expand_vertical);
    }
}
