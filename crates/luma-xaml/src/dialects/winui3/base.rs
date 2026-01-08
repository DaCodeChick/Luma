//! Base types in the WinUI 3 type hierarchy.

use crate::types::{BasicXamlType, XamlProperty, XamlTypeName};
use super::types::*;

/// DependencyObject - root of the dependency property system.
pub fn dependency_object_type() -> BasicXamlType {
    BasicXamlType::new(XamlTypeName::new(WINUI3_NAMESPACE, "DependencyObject"))
        .as_abstract()
}

/// UIElement - base class for all visual elements.
pub fn ui_element_type_def() -> BasicXamlType {
    BasicXamlType::new(ui_element_type())
        .with_base_type(XamlTypeName::new(WINUI3_NAMESPACE, "DependencyObject"))
        .as_abstract()
        // Common properties
        .with_property(
            XamlProperty::new("Visibility", visibility_type())
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("Opacity", double_type())
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("IsHitTestVisible", boolean_type())
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("RenderTransform", object_type())
                .dependency_property()
        )
}

/// FrameworkElement - base class for elements with layout.
pub fn framework_element_type_def() -> BasicXamlType {
    BasicXamlType::new(framework_element_type())
        .with_base_type(ui_element_type())
        .as_abstract()
        // Layout properties
        .with_property(
            XamlProperty::new("Width", double_type())
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("Height", double_type())
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("MinWidth", double_type())
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("MinHeight", double_type())
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("MaxWidth", double_type())
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("MaxHeight", double_type())
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("Margin", thickness_type())
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("Padding", thickness_type())
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("HorizontalAlignment", horizontal_alignment_type())
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("VerticalAlignment", vertical_alignment_type())
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("Tag", object_type())
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("DataContext", object_type())
                .dependency_property()
        )
}

/// Panel - base class for layout panels.
pub fn panel_type_def() -> BasicXamlType {
    BasicXamlType::new(panel_type())
        .with_base_type(framework_element_type())
        .as_abstract()
        .with_property(
            XamlProperty::new("Background", brush_type())
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("Children", object_type())
                .collection()
                .readonly()
                .content_property()
        )
        .with_content_property("Children")
}

/// Control - base class for interactive controls.
pub fn control_type_def() -> BasicXamlType {
    BasicXamlType::new(control_type())
        .with_base_type(framework_element_type())
        .as_abstract()
        .with_property(
            XamlProperty::new("Background", brush_type())
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("Foreground", brush_type())
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("BorderBrush", brush_type())
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("BorderThickness", thickness_type())
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("FontFamily", font_family_type())
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("FontSize", double_type())
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("FontWeight", font_weight_type())
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("FontStyle", font_style_type())
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("IsEnabled", boolean_type())
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("IsTabStop", boolean_type())
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("TabIndex", int32_type())
                .dependency_property()
        )
}

/// ContentControl - control that displays a single piece of content.
pub fn content_control_type_def() -> BasicXamlType {
    BasicXamlType::new(content_control_type())
        .with_base_type(control_type())
        .as_abstract()
        .with_property(
            XamlProperty::new("Content", object_type())
                .dependency_property()
                .content_property()
        )
        .with_content_property("Content")
}
