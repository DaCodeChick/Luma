//! luma-xaml: Pure Rust XAML parser supporting WinUI 3, WPF, and generic XAML.
//!
//! This crate provides a complete XAML parsing solution with support for:
//! - Multiple XAML dialects (WinUI 3, WPF, generic)
//! - Type system with metadata
//! - Markup extensions ({Binding}, {StaticResource}, etc.)
//! - Resource dictionaries
//! - Namespace handling
//!
//! # Example
//!
//! ```rust,ignore
//! use luma_xaml::{XamlParser, dialects::winui3};
//!
//! // Create a parser with WinUI 3 type registry
//! let registry = winui3::create_type_registry();
//! let parser = XamlParser::new(registry);
//!
//! // Parse a XAML file
//! let document = parser.parse_file("MainWindow.xaml")?;
//!
//! // Access the parsed structure
//! println!("Root element: {}", document.root.type_name);
//! ```

#![deny(missing_docs)]
#![warn(rust_2018_idioms)]

pub mod error;
pub mod flags;
pub mod model;
pub mod types;
pub mod markup;
pub mod dialects;
pub mod parser;
pub mod reader;
pub mod context;

// Re-export commonly used types
pub use error::{XamlError, Result, ErrorLocation};
pub use flags::{ParserFlags, ElementFlags, PropertyFlags};
pub use model::{XamlElement, XamlNode, XamlValue, XamlDocument};
pub use types::{XamlTypeName, XamlType, XamlProperty, TypeRegistry};
pub use markup::{MarkupExtension, StaticResourceExtension, BindingExtension, NullExtension, TypeExtension};
pub use parser::{XamlParser, ParserSettings};
pub use context::ServiceProvider;

/// Prelude module for convenient imports.
pub mod prelude {
    pub use crate::error::{XamlError, Result};
    pub use crate::flags::{ParserFlags, ElementFlags, PropertyFlags};
    pub use crate::model::{XamlElement, XamlNode, XamlValue, XamlDocument};
    pub use crate::types::{XamlTypeName, XamlType, XamlProperty, TypeRegistry};
    pub use crate::parser::{XamlParser, ParserSettings};
}
