use luma_core::{Result, Point, Size, Rect, WidgetId, Widget, ListBoxFlags, traits::ListBoxBackend};
use crate::window::Window;
use crate::Win32ListBox;

/// Cross-platform listbox widget
pub struct ListBox {
    backend: Win32ListBox,
    id: WidgetId,
    bounds: Rect,
    on_select_single: Option<Box<dyn FnMut(Option<usize>)>>,
    on_select_multi: Option<Box<dyn FnMut(Vec<usize>)>>,
}

impl ListBox {
    /// Create a listbox builder
    pub fn builder() -> ListBoxBuilder {
        ListBoxBuilder::default()
    }
    
    /// Add an item to the listbox
    pub fn add_item(&mut self, item: &str) -> Result<()> {
        self.backend.add_item(item)
    }
    
    /// Remove an item by index
    pub fn remove_item(&mut self, index: usize) -> Result<()> {
        self.backend.remove_item(index)
    }
    
    /// Clear all items
    pub fn clear(&mut self) -> Result<()> {
        self.backend.clear()
    }
    
    /// Get the number of items
    pub fn item_count(&self) -> Result<usize> {
        self.backend.item_count()
    }
    
    /// Get selected index (for single-select)
    pub fn get_selected_index(&self) -> Result<Option<usize>> {
        self.backend.get_selected_index()
    }
    
    /// Get selected indices (for multi-select)
    pub fn get_selected_indices(&self) -> Result<Vec<usize>> {
        self.backend.get_selected_indices()
    }
    
    /// Set selected index (for single-select)
    pub fn set_selected_index(&mut self, index: Option<usize>) -> Result<()> {
        self.backend.set_selected_index(index)
    }
}

impl Widget for ListBox {
    fn set_bounds(&mut self, bounds: Rect) -> Result<()> {
        self.bounds = bounds;
        self.backend.set_bounds(bounds.x, bounds.y, bounds.width, bounds.height)?;
        Ok(())
    }
    
    fn get_bounds(&self) -> Rect {
        self.bounds
    }
    
    fn id(&self) -> WidgetId {
        self.id
    }
}

/// Builder for creating listboxes
#[derive(Default)]
pub struct ListBoxBuilder {
    items: Vec<String>,
    position: Option<Point>,
    size: Option<Size>,
    flags: Option<ListBoxFlags>,
    on_select_single: Option<Box<dyn FnMut(Option<usize>)>>,
    on_select_multi: Option<Box<dyn FnMut(Vec<usize>)>>,
}

impl ListBoxBuilder {
    /// Create a new listbox builder
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set items (can pass any iterable of string-like types)
    pub fn items<I, S>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.items = items.into_iter().map(|s| s.into()).collect();
        self
    }
    
    /// Add a single item
    pub fn item(mut self, item: impl Into<String>) -> Self {
        self.items.push(item.into());
        self
    }
    
    /// Set the position
    pub fn position(mut self, x: i32, y: i32) -> Self {
        self.position = Some(Point::new(x, y));
        self
    }
    
    /// Set the size
    pub fn size(mut self, width: u32, height: u32) -> Self {
        self.size = Some(Size::new(width, height));
        self
    }
    
    /// Enable multi-select mode
    pub fn multi_select(mut self, enable: bool) -> Self {
        let mut flags = self.flags.unwrap_or_default();
        flags.set(ListBoxFlags::MULTI_SELECT, enable);
        self.flags = Some(flags);
        self
    }
    
    /// Enable sorted mode
    pub fn sorted(mut self, enable: bool) -> Self {
        let mut flags = self.flags.unwrap_or_default();
        flags.set(ListBoxFlags::SORTED, enable);
        self.flags = Some(flags);
        self
    }
    
    /// Set the single-select callback
    pub fn on_select_single<F>(mut self, callback: F) -> Self
    where
        F: FnMut(Option<usize>) + 'static,
    {
        self.on_select_single = Some(Box::new(callback));
        self
    }
    
    /// Set the multi-select callback (automatically enables MULTI_SELECT flag)
    pub fn on_select_multi<F>(mut self, callback: F) -> Self
    where
        F: FnMut(Vec<usize>) + 'static,
    {
        self.on_select_multi = Some(Box::new(callback));
        
        // Automatically enable MULTI_SELECT flag
        let mut flags = self.flags.unwrap_or_default();
        flags.insert(ListBoxFlags::MULTI_SELECT);
        self.flags = Some(flags);
        
        self
    }
    
    /// Build the listbox
    pub fn build(self, parent: &Window) -> Result<ListBox> {
        // Validate: cannot have both callbacks
        if self.on_select_single.is_some() && self.on_select_multi.is_some() {
            return Err(luma_core::Error::InvalidParameter(
                "Cannot specify both on_select_single and on_select_multi".into()
            ));
        }
        
        let pos = self.position.unwrap_or(Point::new(0, 0));
        let size = self.size.unwrap_or(Size::new(200, 150));
        let flags = self.flags.unwrap_or_default();
        
        let parent_hwnd = parent.raw_handle();
        let backend = Win32ListBox::new(parent_hwnd, pos, size, flags)?;
        
        let mut listbox = ListBox {
            backend,
            id: WidgetId::new(),
            bounds: Rect::from_point_size(pos, size),
            on_select_single: self.on_select_single,
            on_select_multi: self.on_select_multi,
        };
        
        // Add initial items
        for item in self.items {
            listbox.add_item(&item)?;
        }
        
        Ok(listbox)
    }
}
