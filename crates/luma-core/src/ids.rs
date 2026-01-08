use std::sync::atomic::{AtomicU64, Ordering};

/// Unique identifier for a widget
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WidgetId(u64);

impl WidgetId {
    /// Generate a new unique widget ID
    pub fn new() -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        Self(COUNTER.fetch_add(1, Ordering::Relaxed))
    }
    
    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

impl Default for WidgetId {
    fn default() -> Self {
        Self::new()
    }
}

/// Unique identifier for a window
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WindowId(u64);

impl WindowId {
    /// Generate a new unique window ID
    pub fn new() -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        Self(COUNTER.fetch_add(1, Ordering::Relaxed))
    }
    
    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

impl Default for WindowId {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_widget_id_uniqueness() {
        let id1 = WidgetId::new();
        let id2 = WidgetId::new();
        assert_ne!(id1, id2);
    }
    
    #[test]
    fn test_window_id_uniqueness() {
        let id1 = WindowId::new();
        let id2 = WindowId::new();
        assert_ne!(id1, id2);
    }
}
