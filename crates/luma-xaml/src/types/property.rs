//! XAML property metadata.

use crate::types::XamlTypeName;
use crate::flags::PropertyFlags;

/// Metadata about a XAML property.
#[derive(Debug, Clone)]
pub struct XamlProperty {
    /// The property name.
    pub name: String,
    
    /// The property type.
    pub type_name: XamlTypeName,
    
    /// Property flags.
    pub flags: PropertyFlags,
}

impl XamlProperty {
    /// Create a new XAML property.
    pub fn new(name: impl Into<String>, type_name: XamlTypeName) -> Self {
        Self {
            name: name.into(),
            type_name,
            flags: PropertyFlags::empty(),
        }
    }

    /// Mark this as an attached property.
    pub fn attached(mut self) -> Self {
        self.flags.insert(PropertyFlags::ATTACHED);
        self
    }

    /// Mark this as read-only.
    pub fn readonly(mut self) -> Self {
        self.flags.insert(PropertyFlags::READONLY);
        self
    }

    /// Mark this as a dependency property.
    pub fn dependency_property(mut self) -> Self {
        self.flags.insert(PropertyFlags::DEPENDENCY_PROPERTY);
        self
    }

    /// Mark this as a collection property.
    pub fn collection(mut self) -> Self {
        self.flags.insert(PropertyFlags::COLLECTION);
        self
    }

    /// Mark this as the content property.
    pub fn content_property(mut self) -> Self {
        self.flags.insert(PropertyFlags::CONTENT_PROPERTY);
        self
    }

    /// Check if this is an attached property.
    pub fn is_attached(&self) -> bool {
        self.flags.contains(PropertyFlags::ATTACHED)
    }

    /// Check if this is read-only.
    pub fn is_readonly(&self) -> bool {
        self.flags.contains(PropertyFlags::READONLY)
    }

    /// Check if this is a dependency property.
    pub fn is_dependency_property(&self) -> bool {
        self.flags.contains(PropertyFlags::DEPENDENCY_PROPERTY)
    }

    /// Check if this is a collection property.
    pub fn is_collection(&self) -> bool {
        self.flags.contains(PropertyFlags::COLLECTION)
    }

    /// Check if this is the content property.
    pub fn is_content_property(&self) -> bool {
        self.flags.contains(PropertyFlags::CONTENT_PROPERTY)
    }

    /// Check if a specific flag is set.
    pub fn has_flag(&self, flag: PropertyFlags) -> bool {
        self.flags.contains(flag)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_property_creation() {
        let type_name = XamlTypeName::new("System", "String");
        let property = XamlProperty::new("Text", type_name.clone());
        
        assert_eq!(property.name, "Text");
        assert_eq!(property.type_name, type_name);
        assert!(!property.is_attached());
        assert!(!property.is_readonly());
    }

    #[test]
    fn test_attached_property() {
        let type_name = XamlTypeName::new("System", "Int32");
        let property = XamlProperty::new("Row", type_name).attached();
        
        assert!(property.is_attached());
        assert!(property.has_flag(PropertyFlags::ATTACHED));
    }

    #[test]
    fn test_multiple_flags() {
        let type_name = XamlTypeName::new("System", "String");
        let property = XamlProperty::new("Items", type_name)
            .collection()
            .readonly()
            .content_property();
        
        assert!(property.is_collection());
        assert!(property.is_readonly());
        assert!(property.is_content_property());
    }
}
