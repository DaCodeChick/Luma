//! Markup extension base trait.

use crate::model::XamlValue;
use crate::context::ServiceProvider;
use crate::error::Result;
use std::fmt::Debug;

/// Base trait for XAML markup extensions.
///
/// Markup extensions provide values that are evaluated at parse time or runtime.
/// Examples include {Binding}, {StaticResource}, {Null}, etc.
pub trait MarkupExtension: Debug {
    /// Get the name of this markup extension.
    fn extension_name(&self) -> &str;
    
    /// Provide the value for this markup extension.
    ///
    /// The `context` parameter provides access to services like resource lookup,
    /// type resolution, etc.
    fn provide_value(&self, context: &ServiceProvider) -> Result<XamlValue>;
}
