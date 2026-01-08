//! Type registry for managing XAML types and namespace mappings.

use crate::types::{XamlType, XamlTypeName};
use std::collections::HashMap;

/// Registry of XAML types and namespace mappings.
pub struct TypeRegistry {
    /// Map from full type name to type metadata.
    types: HashMap<String, Box<dyn XamlType>>,
    
    /// Map from namespace prefix to URI.
    namespaces: HashMap<String, String>,
    
    /// Map from namespace URI to prefix.
    reverse_namespaces: HashMap<String, String>,
}

impl TypeRegistry {
    /// Create a new empty type registry.
    pub fn new() -> Self {
        Self {
            types: HashMap::new(),
            namespaces: HashMap::new(),
            reverse_namespaces: HashMap::new(),
        }
    }

    /// Register a XAML type.
    pub fn register_type(&mut self, xaml_type: Box<dyn XamlType>) {
        let key = xaml_type.name().full_name();
        self.types.insert(key, xaml_type);
    }

    /// Look up a type by name.
    pub fn lookup_type(&self, name: &XamlTypeName) -> Option<&dyn XamlType> {
        self.types.get(&name.full_name()).map(|b| b.as_ref())
    }

    /// Register a namespace mapping.
    pub fn register_namespace(&mut self, prefix: impl Into<String>, uri: impl Into<String>) {
        let prefix = prefix.into();
        let uri = uri.into();
        self.reverse_namespaces.insert(uri.clone(), prefix.clone());
        self.namespaces.insert(prefix, uri);
    }

    /// Resolve a namespace prefix to its URI.
    pub fn resolve_namespace(&self, prefix: &str) -> Option<&str> {
        self.namespaces.get(prefix).map(|s| s.as_str())
    }

    /// Get the prefix for a namespace URI.
    pub fn get_prefix(&self, uri: &str) -> Option<&str> {
        self.reverse_namespaces.get(uri).map(|s| s.as_str())
    }

    /// Get all registered namespace prefixes.
    pub fn namespace_prefixes(&self) -> impl Iterator<Item = &str> {
        self.namespaces.keys().map(|s| s.as_str())
    }

    /// Get all registered types.
    pub fn types(&self) -> impl Iterator<Item = &dyn XamlType> {
        self.types.values().map(|b| b.as_ref())
    }
    
    /// Get all properties for a type, including inherited properties.
    pub fn get_all_properties(&self, type_name: &XamlTypeName) -> Vec<&crate::types::XamlProperty> {
        let mut properties = Vec::new();
        let mut current_type_name = Some(type_name.clone());
        
        while let Some(ref name) = current_type_name {
            if let Some(xaml_type) = self.lookup_type(name) {
                // Add properties from this type
                properties.extend(xaml_type.properties().iter());
                
                // Move to base type
                current_type_name = xaml_type.base_type().cloned();
            } else {
                break;
            }
        }
        
        properties
    }
}

impl Default for TypeRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::BasicXamlType;

    #[test]
    fn test_registry() {
        let mut registry = TypeRegistry::new();
        
        let type_name = XamlTypeName::new("Test", "Button");
        let xaml_type = BasicXamlType::new(type_name.clone());
        registry.register_type(Box::new(xaml_type));
        
        assert!(registry.lookup_type(&type_name).is_some());
    }

    #[test]
    fn test_namespaces() {
        let mut registry = TypeRegistry::new();
        
        registry.register_namespace("test", "http://test.com/xaml");
        
        assert_eq!(registry.resolve_namespace("test"), Some("http://test.com/xaml"));
        assert_eq!(registry.get_prefix("http://test.com/xaml"), Some("test"));
    }
}
