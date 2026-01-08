//! XAML object model - elements, nodes, values, and documents.

pub mod element;
pub mod document;

pub use element::{XamlElement, XamlNode, XamlValue};
pub use document::XamlDocument;
