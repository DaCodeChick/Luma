//! XAML type metadata trait and implementations.

use crate::types::{XamlTypeName, XamlProperty};

/// Trait representing a XAML type with metadata.
pub trait XamlType {
    /// Get the name of this type.
    fn name(&self) -> &XamlTypeName;
    
    /// Get the base type (parent class), if any.
    fn base_type(&self) -> Option<&XamlTypeName>;
    
    /// Get all properties defined on this type.
    fn properties(&self) -> &[XamlProperty];
    
    /// Check if this type is a collection type.
    fn is_collection(&self) -> bool;
    
    /// Get the content property name (property that accepts direct content), if any.
    /// For example, StackPanel's content property is "Children".
    fn content_property(&self) -> Option<&str>;
    
    /// Check if this type can be instantiated.
    fn is_instantiable(&self) -> bool {
        true
    }
    
    /// Check if this is an abstract type.
    fn is_abstract(&self) -> bool {
        false
    }
}

/// A basic implementation of XamlType for custom types.
#[derive(Debug, Clone)]
pub struct BasicXamlType {
    /// The type name.
    pub name: XamlTypeName,
    
    /// The base type.
    pub base_type: Option<XamlTypeName>,
    
    /// Properties on this type.
    pub properties: Vec<XamlProperty>,
    
    /// Whether this is a collection type.
    pub is_collection: bool,
    
    /// The content property name.
    pub content_property: Option<String>,
    
    /// Whether this type is abstract.
    pub is_abstract: bool,
}

impl BasicXamlType {
    /// Create a new basic XAML type.
    pub fn new(name: XamlTypeName) -> Self {
        Self {
            name,
            base_type: None,
            properties: Vec::new(),
            is_collection: false,
            content_property: None,
            is_abstract: false,
        }
    }

    /// Set the base type.
    pub fn with_base_type(mut self, base_type: XamlTypeName) -> Self {
        self.base_type = Some(base_type);
        self
    }

    /// Add a property to this type.
    pub fn with_property(mut self, property: XamlProperty) -> Self {
        self.properties.push(property);
        self
    }

    /// Mark this as a collection type.
    pub fn as_collection(mut self) -> Self {
        self.is_collection = true;
        self
    }

    /// Set the content property.
    pub fn with_content_property(mut self, property: impl Into<String>) -> Self {
        self.content_property = Some(property.into());
        self
    }

    /// Mark this type as abstract.
    pub fn as_abstract(mut self) -> Self {
        self.is_abstract = true;
        self
    }
}

impl XamlType for BasicXamlType {
    fn name(&self) -> &XamlTypeName {
        &self.name
    }

    fn base_type(&self) -> Option<&XamlTypeName> {
        self.base_type.as_ref()
    }

    fn properties(&self) -> &[XamlProperty] {
        &self.properties
    }

    fn is_collection(&self) -> bool {
        self.is_collection
    }

    fn content_property(&self) -> Option<&str> {
        self.content_property.as_deref()
    }

    fn is_abstract(&self) -> bool {
        self.is_abstract
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_xaml_type() {
        let type_name = XamlTypeName::new("Test", "MyControl");
        let xaml_type = BasicXamlType::new(type_name.clone());
        
        assert_eq!(xaml_type.name(), &type_name);
        assert!(!xaml_type.is_collection());
        assert!(!xaml_type.is_abstract());
    }

    #[test]
    fn test_collection_type() {
        let type_name = XamlTypeName::new("Test", "MyCollection");
        let xaml_type = BasicXamlType::new(type_name)
            .as_collection()
            .with_content_property("Items");
        
        assert!(xaml_type.is_collection());
        assert_eq!(xaml_type.content_property(), Some("Items"));
    }
}
