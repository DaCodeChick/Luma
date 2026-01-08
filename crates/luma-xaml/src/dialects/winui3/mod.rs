//! WinUI 3 XAML dialect support.

mod types;
mod base;
mod controls;
mod panels;
mod windows;

use crate::types::TypeRegistry;

/// Create a type registry pre-populated with WinUI 3 types.
///
/// This registry includes common WinUI 3 controls like Button, TextBlock, StackPanel, etc.
pub fn create_type_registry() -> TypeRegistry {
    let mut registry = TypeRegistry::new();
    
    // Register WinUI 3 namespaces
    registry.register_namespace(
        "",
        "http://schemas.microsoft.com/winfx/2006/xaml/presentation"
    );
    registry.register_namespace(
        "x",
        "http://schemas.microsoft.com/winfx/2006/xaml"
    );
    
    // Register base types
    registry.register_type(Box::new(base::dependency_object_type()));
    registry.register_type(Box::new(base::ui_element_type_def()));
    registry.register_type(Box::new(base::framework_element_type_def()));
    registry.register_type(Box::new(base::panel_type_def()));
    registry.register_type(Box::new(base::control_type_def()));
    registry.register_type(Box::new(base::content_control_type_def()));
    
    // Register controls
    registry.register_type(Box::new(controls::button_type()));
    registry.register_type(Box::new(controls::text_block_type()));
    registry.register_type(Box::new(controls::text_box_type()));
    registry.register_type(Box::new(controls::check_box_type()));
    registry.register_type(Box::new(controls::radio_button_type()));
    registry.register_type(Box::new(controls::toggle_switch_type()));
    registry.register_type(Box::new(controls::slider_type()));
    registry.register_type(Box::new(controls::progress_bar_type()));
    registry.register_type(Box::new(controls::image_type()));
    registry.register_type(Box::new(controls::border_type()));
    registry.register_type(Box::new(controls::rectangle_type()));
    registry.register_type(Box::new(controls::ellipse_type()));
    
    // Register panels
    registry.register_type(Box::new(panels::stack_panel_type()));
    registry.register_type(Box::new(panels::grid_type()));
    registry.register_type(Box::new(panels::row_definition_type()));
    registry.register_type(Box::new(panels::column_definition_type()));
    registry.register_type(Box::new(panels::canvas_type()));
    registry.register_type(Box::new(panels::relative_panel_type()));
    registry.register_type(Box::new(panels::scroll_viewer_type()));
    registry.register_type(Box::new(panels::viewbox_type()));
    
    // Register window types
    registry.register_type(Box::new(windows::window_type()));
    registry.register_type(Box::new(windows::page_type()));
    registry.register_type(Box::new(windows::frame_type()));
    registry.register_type(Box::new(windows::user_control_type()));
    
    registry
}
