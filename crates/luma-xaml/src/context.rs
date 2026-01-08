//! Service provider for markup extension evaluation.

use crate::model::XamlValue;
use crate::types::TypeRegistry;
use std::collections::HashMap;

/// Service provider for markup extension evaluation.
///
/// Provides access to services like resource lookup, type resolution, etc.
pub struct ServiceProvider {
    /// Resources available in the current context.
    resources: HashMap<String, XamlValue>,
    
    /// Type registry for type resolution.
    type_registry: Option<TypeRegistry>,
}

impl ServiceProvider {
    /// Create a new empty service provider.
    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
            type_registry: None,
        }
    }

    /// Create a service provider with a type registry.
    pub fn with_type_registry(type_registry: TypeRegistry) -> Self {
        Self {
            resources: HashMap::new(),
            type_registry: Some(type_registry),
        }
    }

    /// Add a resource to the provider.
    pub fn add_resource(&mut self, key: impl Into<String>, value: XamlValue) {
        self.resources.insert(key.into(), value);
    }

    /// Look up a resource by key.
    pub fn get_resource(&self, key: &str) -> Option<XamlValue> {
        self.resources.get(key).cloned()
    }

    /// Get the type registry.
    pub fn type_registry(&self) -> Option<&TypeRegistry> {
        self.type_registry.as_ref()
    }
}

impl Default for ServiceProvider {
    fn default() -> Self {
        Self::new()
    }
}
