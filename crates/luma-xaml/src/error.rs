//! Error types for XAML parsing and manipulation.

use std::fmt;
use thiserror::Error;

/// Result type for XAML operations.
pub type Result<T> = std::result::Result<T, XamlError>;

/// Errors that can occur during XAML parsing and processing.
#[derive(Error, Debug)]
pub enum XamlError {
    /// XML parsing error with location information.
    #[error("XML parsing error at line {line}, column {col}: {message}")]
    XmlError {
        /// Line number where error occurred.
        line: usize,
        /// Column number where error occurred.
        col: usize,
        /// Error message.
        message: String,
    },

    /// Unknown type referenced in XAML.
    #[error("Unknown type '{type_name}' at line {line}")]
    UnknownType {
        /// The unknown type name.
        type_name: String,
        /// Line where error occurred.
        line: usize
    },

    /// Unknown property on a type.
    #[error("Unknown property '{property}' on type '{type_name}' at line {line}")]
    UnknownProperty {
        /// The type name.
        type_name: String,
        /// The unknown property name.
        property: String,
        /// Line where error occurred.
        line: usize,
    },

    /// Invalid markup extension syntax.
    #[error("Invalid markup extension syntax at line {line}: {details}")]
    InvalidMarkupExtension {
        /// Line where error occurred.
        line: usize,
        /// Error details.
        details: String
    },

    /// Type mismatch error.
    #[error("Type mismatch at line {line}: expected {expected}, got {actual}")]
    TypeMismatch {
        /// Expected type name.
        expected: String,
        /// Actual type name.
        actual: String,
        /// Line where error occurred.
        line: usize,
    },

    /// Invalid namespace declaration.
    #[error("Invalid namespace declaration at line {line}: {details}")]
    InvalidNamespace {
        /// Line where error occurred.
        line: usize,
        /// Error details.
        details: String
    },

    /// Invalid attribute value.
    #[error("Invalid attribute value for '{attribute}' at line {line}: {details}")]
    InvalidAttributeValue {
        /// The attribute name.
        attribute: String,
        /// Line where error occurred.
        line: usize,
        /// Error details.
        details: String,
    },

    /// Resource not found.
    #[error("Resource '{key}' not found (referenced at line {line})")]
    ResourceNotFound {
        /// The resource key.
        key: String,
        /// Line where error occurred.
        line: usize
    },

    /// IO error.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// UTF-8 encoding error.
    #[error("UTF-8 encoding error: {0}")]
    Utf8(#[from] std::str::Utf8Error),

    /// Quick-XML parsing error.
    #[error("XML parsing error: {0}")]
    QuickXml(#[from] quick_xml::Error),

    /// Generic error with custom message.
    #[error("{message}")]
    Custom {
        /// The error message.
        message: String
    },
}

impl XamlError {
    /// Create a custom error with a message.
    pub fn custom(message: impl Into<String>) -> Self {
        XamlError::Custom {
            message: message.into(),
        }
    }

    /// Get the line number where the error occurred, if available.
    pub fn line(&self) -> Option<usize> {
        match self {
            XamlError::XmlError { line, .. }
            | XamlError::UnknownType { line, .. }
            | XamlError::UnknownProperty { line, .. }
            | XamlError::InvalidMarkupExtension { line, .. }
            | XamlError::TypeMismatch { line, .. }
            | XamlError::InvalidNamespace { line, .. }
            | XamlError::InvalidAttributeValue { line, .. }
            | XamlError::ResourceNotFound { line, .. } => Some(*line),
            _ => None,
        }
    }
}

/// Error location information for better debugging.
#[derive(Debug, Clone, Copy)]
pub struct ErrorLocation {
    /// Line number (1-based).
    pub line: usize,
    /// Column number (1-based).
    pub column: usize,
}

impl ErrorLocation {
    /// Create a new error location.
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}

impl fmt::Display for ErrorLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}
