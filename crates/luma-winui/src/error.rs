//! Error types for WinUI backend.

use thiserror::Error;

/// Result type for WinUI operations.
pub type Result<T> = std::result::Result<T, WinUIError>;

/// Errors that can occur in the WinUI backend.
#[derive(Error, Debug)]
pub enum WinUIError {
    /// Windows API error.
    #[error("Windows API error: {0}")]
    Windows(#[from] windows::core::Error),

    /// WinUI runtime initialization failed.
    #[error("Failed to initialize WinUI runtime: {0}")]
    RuntimeInitialization(String),

    /// Failed to create a window.
    #[error("Failed to create window: {0}")]
    WindowCreation(String),

    /// Failed to create a widget.
    #[error("Failed to create widget: {0}")]
    WidgetCreation(String),

    /// XAML parsing error (when xaml-support feature is enabled).
    #[cfg(feature = "xaml-support")]
    #[error("XAML error: {0}")]
    Xaml(#[from] luma_xaml::XamlError),

    /// Luma core error.
    #[error("Luma core error: {0}")]
    Core(#[from] luma_core::Error),

    /// Generic error with custom message.
    #[error("{message}")]
    Custom { message: String },
}

impl WinUIError {
    /// Create a custom error with a message.
    pub fn custom(message: impl Into<String>) -> Self {
        WinUIError::Custom {
            message: message.into(),
        }
    }
}
