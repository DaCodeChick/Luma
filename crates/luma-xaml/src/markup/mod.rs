//! Markup extensions (e.g., {Binding}, {StaticResource}, {Null}).

pub mod extension;
pub mod builtin;
pub mod parser;

pub use extension::MarkupExtension;
pub use builtin::{StaticResourceExtension, BindingExtension, NullExtension, TypeExtension};
pub use parser::{parse_markup_extension, ParsedMarkupExtension};
