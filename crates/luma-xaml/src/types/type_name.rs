//! XAML type name representation with namespace and generic support.

use std::fmt;

/// Represents a XAML type name with namespace and optional type arguments.
///
/// # Examples
///
/// ```
/// use luma_xaml::XamlTypeName;
///
/// // Simple type: Button
/// let button = XamlTypeName::new("Microsoft.UI.Xaml.Controls", "Button");
///
/// // Generic type: List<String>
/// let string_type = XamlTypeName::new("System", "String");
/// let list = XamlTypeName::with_type_args("System.Collections.Generic", "List", vec![string_type]);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct XamlTypeName {
    /// The namespace (e.g., "Microsoft.UI.Xaml.Controls")
    pub namespace: String,
    /// The type name (e.g., "Button")
    pub name: String,
    /// Generic type arguments (if any)
    pub type_args: Vec<XamlTypeName>,
}

impl XamlTypeName {
    /// Create a new XAML type name without generic arguments.
    pub fn new(namespace: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            namespace: namespace.into(),
            name: name.into(),
            type_args: Vec::new(),
        }
    }

    /// Create a new XAML type name with generic type arguments.
    pub fn with_type_args(
        namespace: impl Into<String>,
        name: impl Into<String>,
        type_args: Vec<XamlTypeName>,
    ) -> Self {
        Self {
            namespace: namespace.into(),
            name: name.into(),
            type_args,
        }
    }

    /// Get the fully qualified name (namespace.name).
    pub fn full_name(&self) -> String {
        if self.namespace.is_empty() {
            self.name.clone()
        } else {
            format!("{}.{}", self.namespace, self.name)
        }
    }

    /// Check if this type has generic arguments.
    pub fn is_generic(&self) -> bool {
        !self.type_args.is_empty()
    }

    /// Get the number of generic type arguments.
    pub fn arity(&self) -> usize {
        self.type_args.len()
    }
}

impl fmt::Display for XamlTypeName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.full_name())?;
        
        if !self.type_args.is_empty() {
            write!(f, "<")?;
            for (i, arg) in self.type_args.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", arg)?;
            }
            write!(f, ">")?;
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_type() {
        let type_name = XamlTypeName::new("MyNamespace", "MyType");
        assert_eq!(type_name.namespace, "MyNamespace");
        assert_eq!(type_name.name, "MyType");
        assert_eq!(type_name.full_name(), "MyNamespace.MyType");
        assert!(!type_name.is_generic());
    }

    #[test]
    fn test_generic_type() {
        let string_type = XamlTypeName::new("System", "String");
        let list_type = XamlTypeName::with_type_args(
            "System.Collections.Generic",
            "List",
            vec![string_type],
        );
        
        assert_eq!(list_type.full_name(), "System.Collections.Generic.List");
        assert!(list_type.is_generic());
        assert_eq!(list_type.arity(), 1);
    }

    #[test]
    fn test_display() {
        let string_type = XamlTypeName::new("System", "String");
        let int_type = XamlTypeName::new("System", "Int32");
        let dict_type = XamlTypeName::with_type_args(
            "System.Collections.Generic",
            "Dictionary",
            vec![string_type, int_type],
        );
        
        assert_eq!(
            dict_type.to_string(),
            "System.Collections.Generic.Dictionary<System.String, System.Int32>"
        );
    }
}
