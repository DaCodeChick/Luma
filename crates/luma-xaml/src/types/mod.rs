//! XAML type system - types, properties, and type metadata.

pub mod type_name;
pub mod xaml_type;
pub mod property;
pub mod registry;

pub use type_name::XamlTypeName;
pub use xaml_type::{XamlType, BasicXamlType};
pub use property::XamlProperty;
pub use registry::TypeRegistry;
