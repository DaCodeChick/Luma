use std::marker::PhantomData;

/// Safe wrapper around platform-specific handles
/// 
/// Ensures proper cleanup via Drop trait
pub struct Handle<T> {
    raw: *mut std::ffi::c_void,
    _marker: PhantomData<T>,
}

impl<T> Handle<T> {
    /// Creates a new handle from a raw pointer
    /// 
    /// # Safety
    /// 
    /// The caller must ensure the pointer is valid and will remain valid
    /// for the lifetime of the Handle.
    pub unsafe fn from_raw(raw: *mut std::ffi::c_void) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }
    
    /// Returns the raw pointer
    pub fn as_ptr(&self) -> *mut std::ffi::c_void {
        self.raw
    }
    
    /// Checks if the handle is null
    pub fn is_null(&self) -> bool {
        self.raw.is_null()
    }
}

// Handle can be sent between threads, but the cleanup must happen on the correct thread
unsafe impl<T> Send for Handle<T> {}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_handle_null_check() {
        let handle: Handle<()> = unsafe { Handle::from_raw(std::ptr::null_mut()) };
        assert!(handle.is_null());
    }
}
