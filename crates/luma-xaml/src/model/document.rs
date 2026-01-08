//! XAML document representation.

use crate::model::XamlElement;
use crate::model::XamlValue;
use std::collections::HashMap;

/// A parsed XAML document.
#[derive(Debug, Clone)]
pub struct XamlDocument {
    /// The root element of the document.
    pub root: XamlElement,
    
    /// Resources defined in the document (from <Resources> sections).
    pub resources: HashMap<String, XamlValue>,
}

impl XamlDocument {
    /// Create a new XAML document with the given root element.
    pub fn new(root: XamlElement) -> Self {
        Self {
            root,
            resources: HashMap::new(),
        }
    }

    /// Add a resource to the document's resource dictionary.
    pub fn add_resource(&mut self, key: impl Into<String>, value: XamlValue) {
        self.resources.insert(key.into(), value);
    }

    /// Look up a resource by key.
    pub fn get_resource(&self, key: &str) -> Option<&XamlValue> {
        self.resources.get(key)
    }

    /// Check if a resource exists.
    pub fn has_resource(&self, key: &str) -> bool {
        self.resources.contains_key(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::XamlTypeName;

    #[test]
    fn test_document_creation() {
        let type_name = XamlTypeName::new("Test", "Window");
        let root = XamlElement::new(type_name);
        let doc = XamlDocument::new(root);
        
        assert!(doc.resources.is_empty());
        assert_eq!(doc.root.type_name.name, "Window");
    }

    #[test]
    fn test_resources() {
        let type_name = XamlTypeName::new("Test", "Window");
        let root = XamlElement::new(type_name);
        let mut doc = XamlDocument::new(root);
        
        doc.add_resource("MyBrush", XamlValue::String("#FF0000".to_string()));
        
        assert!(doc.has_resource("MyBrush"));
        assert_eq!(
            doc.get_resource("MyBrush").and_then(|v| v.as_string()),
            Some("#FF0000")
        );
    }
}
