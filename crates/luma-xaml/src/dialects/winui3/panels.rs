//! Layout panels for WinUI 3.

use crate::types::{BasicXamlType, XamlProperty, XamlTypeName};
use super::types::*;

/// StackPanel - arranges child elements in a single line.
pub fn stack_panel_type() -> BasicXamlType {
    BasicXamlType::new(XamlTypeName::new(WINUI3_NAMESPACE, "StackPanel"))
        .with_base_type(panel_type())
        .with_property(
            XamlProperty::new("Orientation", orientation_type())
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("Spacing", double_type())
                .dependency_property()
        )
}

/// Grid - defines a flexible grid area.
pub fn grid_type() -> BasicXamlType {
    BasicXamlType::new(XamlTypeName::new(WINUI3_NAMESPACE, "Grid"))
        .with_base_type(panel_type())
        .with_property(
            XamlProperty::new("RowDefinitions", object_type())
                .collection()
                .readonly()
        )
        .with_property(
            XamlProperty::new("ColumnDefinitions", object_type())
                .collection()
                .readonly()
        )
        // Attached properties
        .with_property(
            XamlProperty::new("Row", int32_type())
                .attached()
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("Column", int32_type())
                .attached()
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("RowSpan", int32_type())
                .attached()
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("ColumnSpan", int32_type())
                .attached()
                .dependency_property()
        )
}

/// RowDefinition - defines a row in a Grid.
pub fn row_definition_type() -> BasicXamlType {
    BasicXamlType::new(XamlTypeName::new(WINUI3_NAMESPACE, "RowDefinition"))
        .with_base_type(XamlTypeName::new(WINUI3_NAMESPACE, "DependencyObject"))
        .with_property(
            XamlProperty::new("Height", object_type())
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("MinHeight", double_type())
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("MaxHeight", double_type())
                .dependency_property()
        )
}

/// ColumnDefinition - defines a column in a Grid.
pub fn column_definition_type() -> BasicXamlType {
    BasicXamlType::new(XamlTypeName::new(WINUI3_NAMESPACE, "ColumnDefinition"))
        .with_base_type(XamlTypeName::new(WINUI3_NAMESPACE, "DependencyObject"))
        .with_property(
            XamlProperty::new("Width", object_type())
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("MinWidth", double_type())
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("MaxWidth", double_type())
                .dependency_property()
        )
}

/// Canvas - provides absolute positioning.
pub fn canvas_type() -> BasicXamlType {
    BasicXamlType::new(XamlTypeName::new(WINUI3_NAMESPACE, "Canvas"))
        .with_base_type(panel_type())
        // Attached properties
        .with_property(
            XamlProperty::new("Left", double_type())
                .attached()
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("Top", double_type())
                .attached()
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("ZIndex", int32_type())
                .attached()
                .dependency_property()
        )
}

/// RelativePanel - positions elements relative to each other.
pub fn relative_panel_type() -> BasicXamlType {
    BasicXamlType::new(XamlTypeName::new(WINUI3_NAMESPACE, "RelativePanel"))
        .with_base_type(panel_type())
        // Attached properties for alignment
        .with_property(
            XamlProperty::new("Above", object_type())
                .attached()
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("Below", object_type())
                .attached()
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("LeftOf", object_type())
                .attached()
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("RightOf", object_type())
                .attached()
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("AlignLeftWithPanel", boolean_type())
                .attached()
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("AlignRightWithPanel", boolean_type())
                .attached()
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("AlignTopWithPanel", boolean_type())
                .attached()
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("AlignBottomWithPanel", boolean_type())
                .attached()
                .dependency_property()
        )
}

/// ScrollViewer - provides scrollable content area.
pub fn scroll_viewer_type() -> BasicXamlType {
    BasicXamlType::new(XamlTypeName::new(WINUI3_NAMESPACE, "ScrollViewer"))
        .with_base_type(content_control_type())
        .with_property(
            XamlProperty::new("HorizontalScrollBarVisibility", object_type())
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("VerticalScrollBarVisibility", object_type())
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("HorizontalScrollMode", object_type())
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("VerticalScrollMode", object_type())
                .dependency_property()
        )
}

/// ViewBox - scales content to fill available space.
pub fn viewbox_type() -> BasicXamlType {
    BasicXamlType::new(XamlTypeName::new(WINUI3_NAMESPACE, "Viewbox"))
        .with_base_type(framework_element_type())
        .with_property(
            XamlProperty::new("Child", object_type())
                .dependency_property()
                .content_property()
        )
        .with_property(
            XamlProperty::new("Stretch", object_type())
                .dependency_property()
        )
        .with_content_property("Child")
}
