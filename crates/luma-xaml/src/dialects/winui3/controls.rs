//! Common WinUI 3 controls.

use crate::types::{BasicXamlType, XamlProperty, XamlTypeName};
use super::types::*;

/// Button control.
pub fn button_type() -> BasicXamlType {
    BasicXamlType::new(XamlTypeName::new(WINUI3_NAMESPACE, "Button"))
        .with_base_type(content_control_type())
        .with_property(
            XamlProperty::new("ClickMode", object_type())
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("Command", object_type())
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("CommandParameter", object_type())
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("CornerRadius", corner_radius_type())
                .dependency_property()
        )
}

/// TextBlock - displays text.
pub fn text_block_type() -> BasicXamlType {
    BasicXamlType::new(XamlTypeName::new(WINUI3_NAMESPACE, "TextBlock"))
        .with_base_type(framework_element_type())
        .with_property(
            XamlProperty::new("Text", string_type())
                .dependency_property()
                .content_property()
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
            XamlProperty::new("Foreground", brush_type())
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("TextWrapping", object_type())
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("TextAlignment", object_type())
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("TextTrimming", object_type())
                .dependency_property()
        )
        .with_content_property("Text")
}

/// TextBox - editable text input.
pub fn text_box_type() -> BasicXamlType {
    BasicXamlType::new(XamlTypeName::new(WINUI3_NAMESPACE, "TextBox"))
        .with_base_type(control_type())
        .with_property(
            XamlProperty::new("Text", string_type())
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("PlaceholderText", string_type())
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("IsReadOnly", boolean_type())
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("AcceptsReturn", boolean_type())
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("TextWrapping", object_type())
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("MaxLength", int32_type())
                .dependency_property()
        )
}

/// CheckBox - two or three-state checkbox.
pub fn check_box_type() -> BasicXamlType {
    BasicXamlType::new(XamlTypeName::new(WINUI3_NAMESPACE, "CheckBox"))
        .with_base_type(content_control_type())
        .with_property(
            XamlProperty::new("IsChecked", boolean_type())
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("IsThreeState", boolean_type())
                .dependency_property()
        )
}

/// RadioButton - mutually exclusive selection button.
pub fn radio_button_type() -> BasicXamlType {
    BasicXamlType::new(XamlTypeName::new(WINUI3_NAMESPACE, "RadioButton"))
        .with_base_type(content_control_type())
        .with_property(
            XamlProperty::new("IsChecked", boolean_type())
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("GroupName", string_type())
                .dependency_property()
        )
}

/// ToggleSwitch - on/off switch control.
pub fn toggle_switch_type() -> BasicXamlType {
    BasicXamlType::new(XamlTypeName::new(WINUI3_NAMESPACE, "ToggleSwitch"))
        .with_base_type(control_type())
        .with_property(
            XamlProperty::new("IsOn", boolean_type())
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("Header", object_type())
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("OnContent", object_type())
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("OffContent", object_type())
                .dependency_property()
        )
}

/// Slider - value selection slider.
pub fn slider_type() -> BasicXamlType {
    BasicXamlType::new(XamlTypeName::new(WINUI3_NAMESPACE, "Slider"))
        .with_base_type(control_type())
        .with_property(
            XamlProperty::new("Value", double_type())
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("Minimum", double_type())
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("Maximum", double_type())
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("StepFrequency", double_type())
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("Orientation", orientation_type())
                .dependency_property()
        )
}

/// ProgressBar - progress indicator.
pub fn progress_bar_type() -> BasicXamlType {
    BasicXamlType::new(XamlTypeName::new(WINUI3_NAMESPACE, "ProgressBar"))
        .with_base_type(control_type())
        .with_property(
            XamlProperty::new("Value", double_type())
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("Minimum", double_type())
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("Maximum", double_type())
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("IsIndeterminate", boolean_type())
                .dependency_property()
        )
}

/// Image - displays images.
pub fn image_type() -> BasicXamlType {
    BasicXamlType::new(XamlTypeName::new(WINUI3_NAMESPACE, "Image"))
        .with_base_type(framework_element_type())
        .with_property(
            XamlProperty::new("Source", object_type())
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("Stretch", object_type())
                .dependency_property()
        )
}

/// Border - draws a border and background.
pub fn border_type() -> BasicXamlType {
    BasicXamlType::new(XamlTypeName::new(WINUI3_NAMESPACE, "Border"))
        .with_base_type(framework_element_type())
        .with_property(
            XamlProperty::new("Background", brush_type())
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
            XamlProperty::new("CornerRadius", corner_radius_type())
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("Child", object_type())
                .dependency_property()
                .content_property()
        )
        .with_property(
            XamlProperty::new("Padding", thickness_type())
                .dependency_property()
        )
        .with_content_property("Child")
}

/// Rectangle - draws a rectangle shape.
pub fn rectangle_type() -> BasicXamlType {
    BasicXamlType::new(XamlTypeName::new(WINUI3_NAMESPACE, "Rectangle"))
        .with_base_type(framework_element_type())
        .with_property(
            XamlProperty::new("Fill", brush_type())
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("Stroke", brush_type())
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("StrokeThickness", double_type())
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("RadiusX", double_type())
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("RadiusY", double_type())
                .dependency_property()
        )
}

/// Ellipse - draws an ellipse shape.
pub fn ellipse_type() -> BasicXamlType {
    BasicXamlType::new(XamlTypeName::new(WINUI3_NAMESPACE, "Ellipse"))
        .with_base_type(framework_element_type())
        .with_property(
            XamlProperty::new("Fill", brush_type())
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("Stroke", brush_type())
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("StrokeThickness", double_type())
                .dependency_property()
        )
}
