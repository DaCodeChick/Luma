//! WPF XAML dialect support.

use crate::types::TypeRegistry;

/// Create a type registry pre-populated with WPF types.
pub fn create_type_registry() -> TypeRegistry {
    let mut registry = TypeRegistry::new();
    
    // Register WPF namespaces
    registry.register_namespace(
        "",
        "http://schemas.microsoft.com/winfx/2006/xaml/presentation"
    );
    registry.register_namespace(
        "x",
        "http://schemas.microsoft.com/winfx/2006/xaml"
    );
    
    // TODO: Register WPF types
    
    registry
}
