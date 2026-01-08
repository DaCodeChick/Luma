// Main public API for Luma GUI framework

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(windows)] {
        pub(crate) use luma_windows::*;
    } else if #[cfg(target_os = "macos")] {
        compile_error!("macOS support not yet implemented");
    } else if #[cfg(target_os = "linux")] {
        compile_error!("Linux support not yet implemented");
    } else {
        compile_error!("Unsupported platform. Supported platforms: Windows, macOS (future), Linux (future)");
    }
}

pub mod application;
pub mod window;
pub mod widgets;
pub mod prelude;

// Re-export main types at crate root for convenience
pub use application::Application;
pub use window::{Window, WindowBuilder};

// Re-export core types for convenience
pub use luma_core::{
    Error, Result,
    Point, Size, Rect,
    WidgetId, WindowId,
    WindowFlags, ButtonFlags, ListBoxFlags,
    Alignment, Padding, LayoutConstraints,
    BoxLayout, LayoutDirection,
};
