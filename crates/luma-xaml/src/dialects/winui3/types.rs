//! Common primitive types used in WinUI 3.

use crate::types::XamlTypeName;

/// WinUI 3 namespace URI.
pub const WINUI3_NAMESPACE: &str = "http://schemas.microsoft.com/winfx/2006/xaml/presentation";

/// XAML language namespace URI.
#[allow(dead_code)]
pub const XAML_NAMESPACE: &str = "http://schemas.microsoft.com/winfx/2006/xaml";

// Common type names for reuse
pub fn string_type() -> XamlTypeName {
    XamlTypeName::new("System", "String")
}

pub fn int32_type() -> XamlTypeName {
    XamlTypeName::new("System", "Int32")
}

pub fn double_type() -> XamlTypeName {
    XamlTypeName::new("System", "Double")
}

pub fn boolean_type() -> XamlTypeName {
    XamlTypeName::new("System", "Boolean")
}

pub fn object_type() -> XamlTypeName {
    XamlTypeName::new("System", "Object")
}

pub fn brush_type() -> XamlTypeName {
    XamlTypeName::new(WINUI3_NAMESPACE, "Brush")
}

pub fn thickness_type() -> XamlTypeName {
    XamlTypeName::new(WINUI3_NAMESPACE, "Thickness")
}

pub fn corner_radius_type() -> XamlTypeName {
    XamlTypeName::new(WINUI3_NAMESPACE, "CornerRadius")
}

pub fn font_family_type() -> XamlTypeName {
    XamlTypeName::new(WINUI3_NAMESPACE, "FontFamily")
}

pub fn font_weight_type() -> XamlTypeName {
    XamlTypeName::new(WINUI3_NAMESPACE, "FontWeight")
}

pub fn font_style_type() -> XamlTypeName {
    XamlTypeName::new(WINUI3_NAMESPACE, "FontStyle")
}

pub fn horizontal_alignment_type() -> XamlTypeName {
    XamlTypeName::new(WINUI3_NAMESPACE, "HorizontalAlignment")
}

pub fn vertical_alignment_type() -> XamlTypeName {
    XamlTypeName::new(WINUI3_NAMESPACE, "VerticalAlignment")
}

pub fn visibility_type() -> XamlTypeName {
    XamlTypeName::new(WINUI3_NAMESPACE, "Visibility")
}

pub fn orientation_type() -> XamlTypeName {
    XamlTypeName::new(WINUI3_NAMESPACE, "Orientation")
}

pub fn ui_element_type() -> XamlTypeName {
    XamlTypeName::new(WINUI3_NAMESPACE, "UIElement")
}

pub fn framework_element_type() -> XamlTypeName {
    XamlTypeName::new(WINUI3_NAMESPACE, "FrameworkElement")
}

pub fn panel_type() -> XamlTypeName {
    XamlTypeName::new(WINUI3_NAMESPACE, "Panel")
}

pub fn control_type() -> XamlTypeName {
    XamlTypeName::new(WINUI3_NAMESPACE, "Control")
}

pub fn content_control_type() -> XamlTypeName {
    XamlTypeName::new(WINUI3_NAMESPACE, "ContentControl")
}
