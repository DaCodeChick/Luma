use thiserror::Error;

/// Result type for Luma operations
pub type Result<T> = std::result::Result<T, Error>;

/// Error types for Luma GUI operations
#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to create window: {0}")]
    WindowCreation(String),
    
    #[error("Failed to create widget: {0}")]
    WidgetCreation(String),
    
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),
    
    #[error("Platform error: {0}")]
    Platform(String),
    
    #[error("Application not initialized")]
    NotInitialized,
    
    #[error("Operation failed: {0}")]
    OperationFailed(String),
    
    #[error("Layout error: {0}")]
    LayoutError(String),
}
