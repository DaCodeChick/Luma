//! XAML object model - core data structures for representing XAML documents.

use std::collections::HashMap;
use crate::types::XamlTypeName;
use crate::flags::ElementFlags;

/// A value in XAML (property value, attribute, collection element, etc.).
#[derive(Debug, Clone)]
pub enum XamlValue {
    /// A string value.
    String(String),
    
    /// An integer value.
    Integer(i64),
    
    /// A floating-point value.
    Float(f64),
    
    /// A boolean value.
    Boolean(bool),
    
    /// A null/nil value.
    Null,
    
    /// An element (complex object).
    Element(Box<XamlElement>),
    
    /// A markup extension (e.g., {Binding Path=Name}).
    MarkupExtension {
        /// The extension name (e.g., "Binding", "StaticResource").
        extension_name: String,
        /// Arguments passed to the extension.
        arguments: HashMap<String, XamlValue>,
    },
    
    /// A collection of values.
    Collection(Vec<XamlValue>),
}

impl XamlValue {
    /// Try to extract a string value.
    pub fn as_string(&self) -> Option<&str> {
        match self {
            XamlValue::String(s) => Some(s),
            _ => None,
        }
    }

    /// Try to extract an integer value.
    pub fn as_integer(&self) -> Option<i64> {
        match self {
            XamlValue::Integer(i) => Some(*i),
            _ => None,
        }
    }

    /// Try to extract a boolean value.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            XamlValue::Boolean(b) => Some(*b),
            _ => None,
        }
    }

    /// Check if this is a null value.
    pub fn is_null(&self) -> bool {
        matches!(self, XamlValue::Null)
    }

    /// Try to extract an element.
    pub fn as_element(&self) -> Option<&XamlElement> {
        match self {
            XamlValue::Element(e) => Some(e),
            _ => None,
        }
    }

    /// Try to extract a collection.
    pub fn as_collection(&self) -> Option<&[XamlValue]> {
        match self {
            XamlValue::Collection(c) => Some(c),
            _ => None,
        }
    }
}

/// A node in the XAML tree (element, text, or markup extension).
#[derive(Debug, Clone)]
pub enum XamlNode {
    /// An element node (e.g., <Button>).
    Element(XamlElement),
    
    /// A text node (string content).
    Text(String),
}

impl XamlNode {
    /// Try to extract an element.
    pub fn as_element(&self) -> Option<&XamlElement> {
        match self {
            XamlNode::Element(e) => Some(e),
            _ => None,
        }
    }

    /// Try to extract text.
    pub fn as_text(&self) -> Option<&str> {
        match self {
            XamlNode::Text(t) => Some(t),
            _ => None,
        }
    }
}

/// Represents a XAML element (e.g., <Button Content="Click Me"/>).
#[derive(Debug, Clone)]
pub struct XamlElement {
    /// The type of this element (e.g., Button).
    pub type_name: XamlTypeName,
    
    /// Attributes set directly on the element (e.g., Content="Click Me").
    pub attributes: HashMap<String, XamlValue>,
    
    /// Properties set via property element syntax (e.g., <Button.Content>).
    pub properties: HashMap<String, XamlValue>,
    
    /// Child nodes (content).
    pub children: Vec<XamlNode>,
    
    /// Namespace declarations on this element.
    pub namespaces: HashMap<String, String>,
    
    /// The x:Name or x:Key of this element, if any.
    pub name: Option<String>,
    
    /// The x:Key of this element (if it's in a resource dictionary).
    pub key: Option<String>,
    
    /// Element flags tracking various states.
    pub flags: ElementFlags,
}

impl XamlElement {
    /// Create a new XAML element with the given type.
    pub fn new(type_name: XamlTypeName) -> Self {
        Self {
            type_name,
            attributes: HashMap::new(),
            properties: HashMap::new(),
            children: Vec::new(),
            namespaces: HashMap::new(),
            name: None,
            key: None,
            flags: ElementFlags::empty(),
        }
    }

    /// Set an attribute value.
    pub fn set_attribute(&mut self, name: impl Into<String>, value: XamlValue) {
        self.attributes.insert(name.into(), value);
    }

    /// Get an attribute value.
    pub fn get_attribute(&self, name: &str) -> Option<&XamlValue> {
        self.attributes.get(name)
    }

    /// Set a property value.
    pub fn set_property(&mut self, name: impl Into<String>, value: XamlValue) {
        self.properties.insert(name.into(), value);
    }

    /// Get a property value.
    pub fn get_property(&self, name: &str) -> Option<&XamlValue> {
        self.properties.get(name)
    }

    /// Add a child node.
    pub fn add_child(&mut self, child: XamlNode) {
        self.children.push(child);
        self.flags.insert(ElementFlags::HAS_CHILDREN);
    }

    /// Check if this element has any children.
    pub fn has_children(&self) -> bool {
        self.flags.contains(ElementFlags::HAS_CHILDREN)
    }

    /// Get all child elements (ignoring text nodes).
    pub fn child_elements(&self) -> impl Iterator<Item = &XamlElement> {
        self.children.iter().filter_map(|n| n.as_element())
    }

    /// Get all text content (concatenated).
    pub fn text_content(&self) -> String {
        self.children
            .iter()
            .filter_map(|n| n.as_text())
            .collect::<Vec<_>>()
            .join("")
    }

    /// Declare a namespace prefix.
    pub fn declare_namespace(&mut self, prefix: impl Into<String>, uri: impl Into<String>) {
        self.namespaces.insert(prefix.into(), uri.into());
        self.flags.insert(ElementFlags::HAS_NAMESPACES);
    }

    /// Look up a namespace URI by prefix.
    pub fn resolve_namespace(&self, prefix: &str) -> Option<&str> {
        self.namespaces.get(prefix).map(|s| s.as_str())
    }

    /// Set the x:Name attribute.
    pub fn set_name(&mut self, name: impl Into<String>) {
        self.name = Some(name.into());
        self.flags.insert(ElementFlags::HAS_NAME);
    }

    /// Set the x:Key attribute.
    pub fn set_key(&mut self, key: impl Into<String>) {
        self.key = Some(key.into());
        self.flags.insert(ElementFlags::HAS_KEY);
    }

    /// Check if this element has a specific flag set.
    pub fn has_flag(&self, flag: ElementFlags) -> bool {
        self.flags.contains(flag)
    }

    /// Set an element flag.
    pub fn set_flag(&mut self, flag: ElementFlags) {
        self.flags.insert(flag);
    }

    /// Clear an element flag.
    pub fn clear_flag(&mut self, flag: ElementFlags) {
        self.flags.remove(flag);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::XamlTypeName;

    #[test]
    fn test_element_creation() {
        let type_name = XamlTypeName::new("Test", "Button");
        let element = XamlElement::new(type_name.clone());
        
        assert_eq!(element.type_name, type_name);
        assert!(element.attributes.is_empty());
        assert!(element.children.is_empty());
    }

    #[test]
    fn test_attributes() {
        let type_name = XamlTypeName::new("Test", "Button");
        let mut element = XamlElement::new(type_name);
        
        element.set_attribute("Content", XamlValue::String("Click Me".to_string()));
        element.set_attribute("Width", XamlValue::Integer(100));
        
        assert_eq!(
            element.get_attribute("Content").and_then(|v| v.as_string()),
            Some("Click Me")
        );
        assert_eq!(
            element.get_attribute("Width").and_then(|v| v.as_integer()),
            Some(100)
        );
    }

    #[test]
    fn test_children() {
        let type_name = XamlTypeName::new("Test", "StackPanel");
        let mut element = XamlElement::new(type_name);
        
        element.add_child(XamlNode::Text("Hello".to_string()));
        
        let button_type = XamlTypeName::new("Test", "Button");
        let button = XamlElement::new(button_type);
        element.add_child(XamlNode::Element(button));
        
        assert_eq!(element.children.len(), 2);
        assert_eq!(element.text_content(), "Hello");
        assert_eq!(element.child_elements().count(), 1);
    }
}
