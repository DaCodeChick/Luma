//! Property value converters - convert strings to typed values.

use crate::error::{Result, XamlError};

/// Convert a string to a Brush value.
///
/// Supports:
/// - Named colors: "Red", "Blue", "Transparent"
/// - Hex colors: "#FF0000", "#AAFF0000"
pub fn parse_brush(value: &str) -> Result<String> {
    let trimmed = value.trim();
    
    // Hex color
    if trimmed.starts_with('#') {
        // Validate hex format
        let hex = &trimmed[1..];
        if hex.len() == 6 || hex.len() == 8 {
            // Validate all characters are hex digits
            if hex.chars().all(|c| c.is_ascii_hexdigit()) {
                return Ok(trimmed.to_string());
            }
        }
        return Err(XamlError::InvalidAttributeValue {
            attribute: "Brush".to_string(),
            line: 0,
            details: format!("Invalid hex color format: {}", trimmed),
        });
    }
    
    // Named color - just validate it's a valid identifier
    if trimmed.chars().all(|c| c.is_alphanumeric()) {
        return Ok(trimmed.to_string());
    }
    
    Err(XamlError::InvalidAttributeValue {
        attribute: "Brush".to_string(),
        line: 0,
        details: format!("Invalid brush value: {}", trimmed),
    })
}

/// Represents a thickness value (left, top, right, bottom).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Thickness {
    /// Left thickness
    pub left: f64,
    /// Top thickness
    pub top: f64,
    /// Right thickness
    pub right: f64,
    /// Bottom thickness
    pub bottom: f64,
}

impl Thickness {
    /// Create a uniform thickness with the same value on all sides.
    pub fn uniform(value: f64) -> Self {
        Self {
            left: value,
            top: value,
            right: value,
            bottom: value,
        }
    }
    
    /// Create a symmetric thickness with horizontal and vertical values.
    pub fn symmetric(horizontal: f64, vertical: f64) -> Self {
        Self {
            left: horizontal,
            top: vertical,
            right: horizontal,
            bottom: vertical,
        }
    }
}

/// Parse a Thickness value.
///
/// Supports:
/// - Single value: "10" -> 10,10,10,10
/// - Two values: "10,5" -> 10,5,10,5 (horizontal, vertical)
/// - Four values: "10,5,20,15" -> left,top,right,bottom
pub fn parse_thickness(value: &str) -> Result<Thickness> {
    let parts: Vec<&str> = value.split(',').map(|s| s.trim()).collect();
    
    match parts.len() {
        1 => {
            let val = parts[0].parse::<f64>().map_err(|_| {
                XamlError::InvalidAttributeValue {
                    attribute: "Thickness".to_string(),
                    line: 0,
                    details: format!("Invalid number: {}", parts[0]),
                }
            })?;
            Ok(Thickness::uniform(val))
        }
        2 => {
            let horizontal = parts[0].parse::<f64>().map_err(|_| {
                XamlError::InvalidAttributeValue {
                    attribute: "Thickness".to_string(),
                    line: 0,
                    details: format!("Invalid horizontal value: {}", parts[0]),
                }
            })?;
            let vertical = parts[1].parse::<f64>().map_err(|_| {
                XamlError::InvalidAttributeValue {
                    attribute: "Thickness".to_string(),
                    line: 0,
                    details: format!("Invalid vertical value: {}", parts[1]),
                }
            })?;
            Ok(Thickness::symmetric(horizontal, vertical))
        }
        4 => {
            let left = parts[0].parse::<f64>().map_err(|_| {
                XamlError::InvalidAttributeValue {
                    attribute: "Thickness".to_string(),
                    line: 0,
                    details: format!("Invalid left value: {}", parts[0]),
                }
            })?;
            let top = parts[1].parse::<f64>().map_err(|_| {
                XamlError::InvalidAttributeValue {
                    attribute: "Thickness".to_string(),
                    line: 0,
                    details: format!("Invalid top value: {}", parts[1]),
                }
            })?;
            let right = parts[2].parse::<f64>().map_err(|_| {
                XamlError::InvalidAttributeValue {
                    attribute: "Thickness".to_string(),
                    line: 0,
                    details: format!("Invalid right value: {}", parts[2]),
                }
            })?;
            let bottom = parts[3].parse::<f64>().map_err(|_| {
                XamlError::InvalidAttributeValue {
                    attribute: "Thickness".to_string(),
                    line: 0,
                    details: format!("Invalid bottom value: {}", parts[3]),
                }
            })?;
            Ok(Thickness {
                left,
                top,
                right,
                bottom,
            })
        }
        _ => Err(XamlError::InvalidAttributeValue {
            attribute: "Thickness".to_string(),
            line: 0,
            details: format!("Thickness must have 1, 2, or 4 values, got {}", parts.len()),
        }),
    }
}

/// Represents a corner radius value.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CornerRadius {
    /// Top-left corner radius
    pub top_left: f64,
    /// Top-right corner radius
    pub top_right: f64,
    /// Bottom-right corner radius
    pub bottom_right: f64,
    /// Bottom-left corner radius
    pub bottom_left: f64,
}

impl CornerRadius {
    /// Create a uniform corner radius with the same value for all corners.
    pub fn uniform(value: f64) -> Self {
        Self {
            top_left: value,
            top_right: value,
            bottom_right: value,
            bottom_left: value,
        }
    }
}

/// Parse a CornerRadius value.
///
/// Supports:
/// - Single value: "5" -> 5,5,5,5
/// - Four values: "5,10,5,10" -> topLeft,topRight,bottomRight,bottomLeft
pub fn parse_corner_radius(value: &str) -> Result<CornerRadius> {
    let parts: Vec<&str> = value.split(',').map(|s| s.trim()).collect();
    
    match parts.len() {
        1 => {
            let val = parts[0].parse::<f64>().map_err(|_| {
                XamlError::InvalidAttributeValue {
                    attribute: "CornerRadius".to_string(),
                    line: 0,
                    details: format!("Invalid number: {}", parts[0]),
                }
            })?;
            Ok(CornerRadius::uniform(val))
        }
        4 => {
            let top_left = parts[0].parse::<f64>().map_err(|_| {
                XamlError::InvalidAttributeValue {
                    attribute: "CornerRadius".to_string(),
                    line: 0,
                    details: format!("Invalid top-left value: {}", parts[0]),
                }
            })?;
            let top_right = parts[1].parse::<f64>().map_err(|_| {
                XamlError::InvalidAttributeValue {
                    attribute: "CornerRadius".to_string(),
                    line: 0,
                    details: format!("Invalid top-right value: {}", parts[1]),
                }
            })?;
            let bottom_right = parts[2].parse::<f64>().map_err(|_| {
                XamlError::InvalidAttributeValue {
                    attribute: "CornerRadius".to_string(),
                    line: 0,
                    details: format!("Invalid bottom-right value: {}", parts[2]),
                }
            })?;
            let bottom_left = parts[3].parse::<f64>().map_err(|_| {
                XamlError::InvalidAttributeValue {
                    attribute: "CornerRadius".to_string(),
                    line: 0,
                    details: format!("Invalid bottom-left value: {}", parts[3]),
                }
            })?;
            Ok(CornerRadius {
                top_left,
                top_right,
                bottom_right,
                bottom_left,
            })
        }
        _ => Err(XamlError::InvalidAttributeValue {
            attribute: "CornerRadius".to_string(),
            line: 0,
            details: format!("CornerRadius must have 1 or 4 values, got {}", parts.len()),
        }),
    }
}

/// Represents a GridLength value.
#[derive(Debug, Clone, PartialEq)]
pub enum GridLength {
    /// Absolute pixel value
    Absolute(f64),
    /// Automatic sizing
    Auto,
    /// Star sizing (proportional)
    Star(f64),
}

/// Parse a GridLength value.
///
/// Supports:
/// - Absolute: "100" -> 100 pixels
/// - Auto: "Auto" -> automatic sizing
/// - Star: "*" -> 1* (proportional)
/// - Star with multiplier: "2*" -> 2* (proportional)
pub fn parse_grid_length(value: &str) -> Result<GridLength> {
    let trimmed = value.trim();
    
    if trimmed.eq_ignore_ascii_case("Auto") {
        return Ok(GridLength::Auto);
    }
    
    if trimmed == "*" {
        return Ok(GridLength::Star(1.0));
    }
    
    if trimmed.ends_with('*') {
        let multiplier_str = &trimmed[..trimmed.len() - 1];
        if multiplier_str.is_empty() {
            return Ok(GridLength::Star(1.0));
        }
        let multiplier = multiplier_str.parse::<f64>().map_err(|_| {
            XamlError::InvalidAttributeValue {
                attribute: "GridLength".to_string(),
                line: 0,
                details: format!("Invalid star multiplier: {}", multiplier_str),
            }
        })?;
        return Ok(GridLength::Star(multiplier));
    }
    
    // Try to parse as absolute value
    let absolute = trimmed.parse::<f64>().map_err(|_| {
        XamlError::InvalidAttributeValue {
            attribute: "GridLength".to_string(),
            line: 0,
            details: format!("Invalid GridLength value: {}", trimmed),
        }
    })?;
    Ok(GridLength::Absolute(absolute))
}

/// Orientation enum for layout controls.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    /// Horizontal orientation
    Horizontal,
    /// Vertical orientation
    Vertical,
}

/// Parse an Orientation value.
///
/// Supports:
/// - "Horizontal"
/// - "Vertical"
pub fn parse_orientation(value: &str) -> Result<Orientation> {
    match value.trim() {
        "Horizontal" => Ok(Orientation::Horizontal),
        "Vertical" => Ok(Orientation::Vertical),
        _ => Err(XamlError::InvalidAttributeValue {
            attribute: "Orientation".to_string(),
            line: 0,
            details: format!("Invalid orientation value: {}. Expected 'Horizontal' or 'Vertical'", value),
        }),
    }
}

/// Visibility enum for UI elements.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Visibility {
    /// Element is visible
    Visible,
    /// Element is collapsed (takes no space)
    Collapsed,
    /// Element is hidden (takes space but not visible)
    Hidden,
}

/// Parse a Visibility value.
///
/// Supports:
/// - "Visible"
/// - "Collapsed"
/// - "Hidden"
pub fn parse_visibility(value: &str) -> Result<Visibility> {
    match value.trim() {
        "Visible" => Ok(Visibility::Visible),
        "Collapsed" => Ok(Visibility::Collapsed),
        "Hidden" => Ok(Visibility::Hidden),
        _ => Err(XamlError::InvalidAttributeValue {
            attribute: "Visibility".to_string(),
            line: 0,
            details: format!("Invalid visibility value: {}. Expected 'Visible', 'Collapsed', or 'Hidden'", value),
        }),
    }
}

/// HorizontalAlignment enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HorizontalAlignment {
    /// Align to the left
    Left,
    /// Align to the center
    Center,
    /// Align to the right
    Right,
    /// Stretch to fill width
    Stretch,
}

/// Parse a HorizontalAlignment value.
///
/// Supports:
/// - "Left"
/// - "Center"
/// - "Right"
/// - "Stretch"
pub fn parse_horizontal_alignment(value: &str) -> Result<HorizontalAlignment> {
    match value.trim() {
        "Left" => Ok(HorizontalAlignment::Left),
        "Center" => Ok(HorizontalAlignment::Center),
        "Right" => Ok(HorizontalAlignment::Right),
        "Stretch" => Ok(HorizontalAlignment::Stretch),
        _ => Err(XamlError::InvalidAttributeValue {
            attribute: "HorizontalAlignment".to_string(),
            line: 0,
            details: format!("Invalid horizontal alignment value: {}. Expected 'Left', 'Center', 'Right', or 'Stretch'", value),
        }),
    }
}

/// VerticalAlignment enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerticalAlignment {
    /// Align to the top
    Top,
    /// Align to the center
    Center,
    /// Align to the bottom
    Bottom,
    /// Stretch to fill height
    Stretch,
}

/// Parse a VerticalAlignment value.
///
/// Supports:
/// - "Top"
/// - "Center"
/// - "Bottom"
/// - "Stretch"
pub fn parse_vertical_alignment(value: &str) -> Result<VerticalAlignment> {
    match value.trim() {
        "Top" => Ok(VerticalAlignment::Top),
        "Center" => Ok(VerticalAlignment::Center),
        "Bottom" => Ok(VerticalAlignment::Bottom),
        "Stretch" => Ok(VerticalAlignment::Stretch),
        _ => Err(XamlError::InvalidAttributeValue {
            attribute: "VerticalAlignment".to_string(),
            line: 0,
            details: format!("Invalid vertical alignment value: {}. Expected 'Top', 'Center', 'Bottom', or 'Stretch'", value),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_brush_hex() {
        assert_eq!(parse_brush("#FF0000").unwrap(), "#FF0000");
        assert_eq!(parse_brush("#AAFF0000").unwrap(), "#AAFF0000");
    }

    #[test]
    fn test_parse_brush_named() {
        assert_eq!(parse_brush("Red").unwrap(), "Red");
        assert_eq!(parse_brush("Transparent").unwrap(), "Transparent");
    }

    #[test]
    fn test_parse_thickness_uniform() {
        let t = parse_thickness("10").unwrap();
        assert_eq!(t, Thickness::uniform(10.0));
    }

    #[test]
    fn test_parse_thickness_symmetric() {
        let t = parse_thickness("10,5").unwrap();
        assert_eq!(t, Thickness::symmetric(10.0, 5.0));
    }

    #[test]
    fn test_parse_thickness_four_values() {
        let t = parse_thickness("1,2,3,4").unwrap();
        assert_eq!(t.left, 1.0);
        assert_eq!(t.top, 2.0);
        assert_eq!(t.right, 3.0);
        assert_eq!(t.bottom, 4.0);
    }

    #[test]
    fn test_parse_corner_radius_uniform() {
        let cr = parse_corner_radius("5").unwrap();
        assert_eq!(cr, CornerRadius::uniform(5.0));
    }

    #[test]
    fn test_parse_corner_radius_four_values() {
        let cr = parse_corner_radius("1,2,3,4").unwrap();
        assert_eq!(cr.top_left, 1.0);
        assert_eq!(cr.top_right, 2.0);
        assert_eq!(cr.bottom_right, 3.0);
        assert_eq!(cr.bottom_left, 4.0);
    }

    #[test]
    fn test_parse_grid_length_absolute() {
        assert_eq!(parse_grid_length("100").unwrap(), GridLength::Absolute(100.0));
    }

    #[test]
    fn test_parse_grid_length_auto() {
        assert_eq!(parse_grid_length("Auto").unwrap(), GridLength::Auto);
        assert_eq!(parse_grid_length("auto").unwrap(), GridLength::Auto);
    }

    #[test]
    fn test_parse_grid_length_star() {
        assert_eq!(parse_grid_length("*").unwrap(), GridLength::Star(1.0));
        assert_eq!(parse_grid_length("2*").unwrap(), GridLength::Star(2.0));
        assert_eq!(parse_grid_length("0.5*").unwrap(), GridLength::Star(0.5));
    }

    #[test]
    fn test_parse_orientation() {
        assert_eq!(parse_orientation("Horizontal").unwrap(), Orientation::Horizontal);
        assert_eq!(parse_orientation("Vertical").unwrap(), Orientation::Vertical);
        assert!(parse_orientation("Invalid").is_err());
    }

    #[test]
    fn test_parse_visibility() {
        assert_eq!(parse_visibility("Visible").unwrap(), Visibility::Visible);
        assert_eq!(parse_visibility("Collapsed").unwrap(), Visibility::Collapsed);
        assert_eq!(parse_visibility("Hidden").unwrap(), Visibility::Hidden);
        assert!(parse_visibility("Invalid").is_err());
    }

    #[test]
    fn test_parse_horizontal_alignment() {
        assert_eq!(parse_horizontal_alignment("Left").unwrap(), HorizontalAlignment::Left);
        assert_eq!(parse_horizontal_alignment("Center").unwrap(), HorizontalAlignment::Center);
        assert_eq!(parse_horizontal_alignment("Right").unwrap(), HorizontalAlignment::Right);
        assert_eq!(parse_horizontal_alignment("Stretch").unwrap(), HorizontalAlignment::Stretch);
        assert!(parse_horizontal_alignment("Invalid").is_err());
    }

    #[test]
    fn test_parse_vertical_alignment() {
        assert_eq!(parse_vertical_alignment("Top").unwrap(), VerticalAlignment::Top);
        assert_eq!(parse_vertical_alignment("Center").unwrap(), VerticalAlignment::Center);
        assert_eq!(parse_vertical_alignment("Bottom").unwrap(), VerticalAlignment::Bottom);
        assert_eq!(parse_vertical_alignment("Stretch").unwrap(), VerticalAlignment::Stretch);
        assert!(parse_vertical_alignment("Invalid").is_err());
    }
}
