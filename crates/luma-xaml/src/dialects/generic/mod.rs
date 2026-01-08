//! Generic XAML dialect support (minimal types).

use crate::types::TypeRegistry;

/// Create a minimal type registry for generic XAML.
pub fn create_type_registry() -> TypeRegistry {
    let mut registry = TypeRegistry::new();
    
    // Register minimal namespaces
    registry.register_namespace(
        "x",
        "http://schemas.microsoft.com/winfx/2006/xaml"
    );
    
    // TODO: Register basic types (string, int, object, etc.)
    
    registry
}
