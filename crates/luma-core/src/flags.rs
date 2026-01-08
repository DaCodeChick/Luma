use bitflags::bitflags;

bitflags! {
    /// Window style flags
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct WindowFlags: u32 {
        /// Window can be resized
        const RESIZABLE = 0b0000_0001;
        /// Window has minimize button
        const MINIMIZABLE = 0b0000_0010;
        /// Window has maximize button
        const MAXIMIZABLE = 0b0000_0100;
        /// Window has close button
        const CLOSABLE = 0b0000_1000;
        /// Window has title bar
        const TITLED = 0b0001_0000;
        /// Window stays on top of other windows
        const ALWAYS_ON_TOP = 0b0010_0000;
    }
}

impl Default for WindowFlags {
    fn default() -> Self {
        // Standard window: resizable, minimizable, maximizable, closable, with title
        Self::RESIZABLE | Self::MINIMIZABLE | Self::MAXIMIZABLE | Self::CLOSABLE | Self::TITLED
    }
}

bitflags! {
    /// Button style flags
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ButtonFlags: u32 {
        /// Button is the default button (activated by Enter)
        const DEFAULT = 0b0001;
        /// Button can be toggled (push/unpush state)
        const TOGGLE = 0b0010;
    }
}

impl Default for ButtonFlags {
    fn default() -> Self {
        Self::empty()
    }
}

bitflags! {
    /// ListBox style flags
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ListBoxFlags: u32 {
        /// Allow selecting multiple items
        const MULTI_SELECT = 0b0001;
        /// Sort items alphabetically
        const SORTED = 0b0010;
        /// Show vertical scrollbar when needed
        const VSCROLL = 0b0100;
        /// Show horizontal scrollbar when needed
        const HSCROLL = 0b1000;
    }
}

impl Default for ListBoxFlags {
    fn default() -> Self {
        Self::VSCROLL // Default: vertical scroll only, single-select
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_window_flags_default() {
        let flags = WindowFlags::default();
        assert!(flags.contains(WindowFlags::RESIZABLE));
        assert!(flags.contains(WindowFlags::TITLED));
    }
    
    #[test]
    fn test_button_flags() {
        let mut flags = ButtonFlags::empty();
        flags.insert(ButtonFlags::DEFAULT);
        assert!(flags.contains(ButtonFlags::DEFAULT));
        assert!(!flags.contains(ButtonFlags::TOGGLE));
    }
    
    #[test]
    fn test_listbox_flags() {
        let flags = ListBoxFlags::MULTI_SELECT | ListBoxFlags::SORTED;
        assert!(flags.contains(ListBoxFlags::MULTI_SELECT));
        assert!(flags.contains(ListBoxFlags::SORTED));
    }
}
