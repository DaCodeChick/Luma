// Core types and traits for Luma GUI framework

pub mod error;
pub mod geometry;
pub mod ids;
pub mod handle;
pub mod traits;
pub mod flags;
pub mod layout;

// Re-export commonly used types
pub use error::{Error, Result};
pub use geometry::{Point, Size, Rect};
pub use ids::{WidgetId, WindowId};
pub use handle::Handle;
pub use flags::{WindowFlags, ButtonFlags, ListBoxFlags};
pub use layout::{
    Alignment, Padding, LayoutConstraints,
    Container, Widget,
    BoxLayout, LayoutDirection,
};
