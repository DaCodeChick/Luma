//! luma-winui: WinUI 3 backend for Luma GUI framework.
//!
//! This crate provides a WinUI 3 (Windows App SDK) backend for the Luma framework.
//! It implements Luma's backend traits using modern Windows UI controls.
//!
//! # Status
//!
//! **⚠️ Experimental**: This is an experimental backend in early proof-of-concept phase.
//! Stability is not guaranteed. The Win32 backend (`luma-windows`) is the recommended
//! production backend.
//!
//! # Requirements
//!
//! - Windows 10 version 1809 or later
//! - Windows App SDK runtime
//!
//! # Example
//!
//! ```rust,ignore
//! use luma_winui::{WinUIRuntime, WinUIWindow};
//!
//! // Initialize WinUI runtime
//! let runtime = WinUIRuntime::initialize()?;
//!
//! // Create a window (programmatic API)
//! let window = WinUIWindow::new("Hello WinUI", 800, 600)?;
//! window.show()?;
//! ```

#![warn(rust_2018_idioms)]

pub mod error;
pub mod runtime;
pub mod window;
pub mod application;
pub mod widgets;
pub mod layout;
pub mod utils;

#[cfg(feature = "xaml-support")]
pub mod xaml;

// Re-export commonly used types
pub use error::{WinUIError, Result};
pub use runtime::WinUIRuntime;
pub use window::WinUIWindow;
pub use application::WinUIApplication;

/// Prelude module for convenient imports.
pub mod prelude {
    pub use crate::error::{WinUIError, Result};
    pub use crate::runtime::WinUIRuntime;
    pub use crate::window::WinUIWindow;
    pub use crate::application::WinUIApplication;
}
