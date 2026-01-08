//! WinUI 3 XAML dialect support.

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
    
    // TODO: Register WinUI 3 types
    // registry.register_type(Box::new(ButtonType::new()));
    // registry.register_type(Box::new(TextBlockType::new()));
    // etc.
    
    registry
}
