//! Top-level window and page types.

use crate::types::{BasicXamlType, XamlProperty, XamlTypeName};
use super::types::*;

/// Window - represents an application window.
pub fn window_type() -> BasicXamlType {
    BasicXamlType::new(XamlTypeName::new(WINUI3_NAMESPACE, "Window"))
        .with_base_type(XamlTypeName::new(WINUI3_NAMESPACE, "DependencyObject"))
        .with_property(
            XamlProperty::new("Title", string_type())
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("Content", object_type())
                .dependency_property()
                .content_property()
        )
        .with_property(
            XamlProperty::new("SystemBackdrop", object_type())
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("ExtendsContentIntoTitleBar", boolean_type())
                .dependency_property()
        )
        .with_content_property("Content")
}

/// Page - represents a navigable page.
pub fn page_type() -> BasicXamlType {
    BasicXamlType::new(XamlTypeName::new(WINUI3_NAMESPACE, "Page"))
        .with_base_type(framework_element_type())
        .with_property(
            XamlProperty::new("Content", object_type())
                .dependency_property()
                .content_property()
        )
        .with_property(
            XamlProperty::new("Background", brush_type())
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("TopAppBar", object_type())
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("BottomAppBar", object_type())
                .dependency_property()
        )
        .with_content_property("Content")
}

/// Frame - hosts pages and manages navigation.
pub fn frame_type() -> BasicXamlType {
    BasicXamlType::new(XamlTypeName::new(WINUI3_NAMESPACE, "Frame"))
        .with_base_type(content_control_type())
        .with_property(
            XamlProperty::new("SourcePageType", object_type())
                .dependency_property()
        )
        .with_property(
            XamlProperty::new("BackStack", object_type())
                .readonly()
        )
        .with_property(
            XamlProperty::new("ForwardStack", object_type())
                .readonly()
        )
}

/// UserControl - represents a reusable UI component.
pub fn user_control_type() -> BasicXamlType {
    BasicXamlType::new(XamlTypeName::new(WINUI3_NAMESPACE, "UserControl"))
        .with_base_type(control_type())
        .with_property(
            XamlProperty::new("Content", object_type())
                .dependency_property()
                .content_property()
        )
        .with_content_property("Content")
}
