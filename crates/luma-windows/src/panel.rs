use windows::Win32::Foundation::*;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use luma_core::{Result, Error, Point, Size, traits::PanelBackend};
use crate::utils::is_valid_hwnd;

/// Win32 panel (container) backend
pub struct Win32Panel {
    hwnd: HWND,
}

impl PanelBackend for Win32Panel {
    fn new(
        parent_hwnd: *mut std::ffi::c_void,
        pos: Point,
        size: Size,
    ) -> Result<Self> {
        tracing::debug!(
            "Creating Win32 panel: pos=({}, {}), size={}x{}",
            pos.x,
            pos.y,
            size.width,
            size.height
        );
        
        unsafe {
            let hinstance = GetModuleHandleW(None).map_err(|e| {
                Error::Platform(format!("Failed to get module handle: {}", e))
            })?;
            
            let parent = HWND(parent_hwnd as isize);
            
            // Create a static control as a container
            let hwnd = CreateWindowExW(
                WINDOW_EX_STYLE(0),
                windows::core::w!("STATIC"),
                windows::core::PCWSTR::null(),
                WS_CHILD | WS_VISIBLE,
                pos.x,
                pos.y,
                size.width as i32,
                size.height as i32,
                parent,
                None,
                hinstance,
                None,
            );
            
            if !is_valid_hwnd(hwnd) {
                return Err(Error::WidgetCreation("Panel creation failed".into()));
            }
            
            tracing::debug!("Panel created successfully: HWND={:?}", hwnd);
            
            Ok(Self { hwnd })
        }
    }
    
    fn raw_handle(&self) -> *mut std::ffi::c_void {
        self.hwnd.0 as *mut std::ffi::c_void
    }
    
    fn set_bounds(&mut self, x: i32, y: i32, width: u32, height: u32) -> Result<()> {
        unsafe {
            SetWindowPos(
                self.hwnd,
                None,
                x,
                y,
                width as i32,
                height as i32,
                SWP_NOZORDER,
            ).map_err(|e| Error::OperationFailed(format!("SetWindowPos failed: {}", e)))?;
        }
        Ok(())
    }
}

impl Drop for Win32Panel {
    fn drop(&mut self) {
        tracing::debug!("Destroying panel: HWND={:?}", self.hwnd);
        unsafe {
            let _ = DestroyWindow(self.hwnd);
        }
    }
}
