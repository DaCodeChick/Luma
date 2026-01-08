//! Markup extensions (e.g., {Binding}, {StaticResource}, {Null}).

pub mod extension;
pub mod builtin;

pub use extension::MarkupExtension;
pub use builtin::{StaticResourceExtension, BindingExtension, NullExtension, TypeExtension};
