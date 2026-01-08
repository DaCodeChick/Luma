//! Built-in markup extensions.

use crate::markup::MarkupExtension;
use crate::model::XamlValue;
use crate::context::ServiceProvider;
use crate::error::{Result, XamlError};
use crate::types::XamlTypeName;

/// {StaticResource Key} markup extension.
#[derive(Debug, Clone)]
pub struct StaticResourceExtension {
    /// The resource key to look up.
    pub key: String,
}

impl MarkupExtension for StaticResourceExtension {
    fn extension_name(&self) -> &str {
        "StaticResource"
    }

    fn provide_value(&self, context: &ServiceProvider) -> Result<XamlValue> {
        context
            .get_resource(&self.key)
            .ok_or_else(|| XamlError::ResourceNotFound {
                key: self.key.clone(),
                line: 0, // TODO: Track line numbers through context
            })
    }
}

/// {Binding Path} markup extension.
#[derive(Debug, Clone)]
pub struct BindingExtension {
    /// The binding path.
    pub path: String,
    
    /// The binding mode (OneWay, TwoWay, etc.).
    pub mode: Option<String>,
    
    /// The binding source.
    pub source: Option<String>,
}

impl MarkupExtension for BindingExtension {
    fn extension_name(&self) -> &str {
        "Binding"
    }

    fn provide_value(&self, _context: &ServiceProvider) -> Result<XamlValue> {
        // For now, we just return a placeholder
        // In a full implementation, this would set up data binding
        Ok(XamlValue::String(format!("{{Binding {}}}", self.path)))
    }
}

/// {x:Null} markup extension.
#[derive(Debug, Clone)]
pub struct NullExtension;

impl MarkupExtension for NullExtension {
    fn extension_name(&self) -> &str {
        "Null"
    }

    fn provide_value(&self, _context: &ServiceProvider) -> Result<XamlValue> {
        Ok(XamlValue::Null)
    }
}

/// {x:Type TypeName} markup extension.
#[derive(Debug, Clone)]
pub struct TypeExtension {
    /// The type name.
    pub type_name: XamlTypeName,
}

impl MarkupExtension for TypeExtension {
    fn extension_name(&self) -> &str {
        "Type"
    }

    fn provide_value(&self, _context: &ServiceProvider) -> Result<XamlValue> {
        // Return the type name as a string for now
        Ok(XamlValue::String(self.type_name.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_null_extension() {
        let ext = NullExtension;
        let context = ServiceProvider::new();
        let value = ext.provide_value(&context).unwrap();
        assert!(value.is_null());
    }
}
