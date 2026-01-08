use windows::Win32::Foundation::*;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use luma_core::{Result, Error, Point, Size, traits::LabelBackend};
use crate::utils::{to_wide_string, is_valid_hwnd};

/// Win32 label backend (STATIC control)
pub struct Win32Label {
    hwnd: HWND,
}

impl LabelBackend for Win32Label {
    fn new(
        parent_hwnd: *mut std::ffi::c_void,
        text: &str,
        pos: Point,
        size: Size,
    ) -> Result<Self> {
        tracing::debug!(
            "Creating Win32 label: text='{}', pos=({}, {}), size={}x{}",
            text,
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
            let label_text = to_wide_string(text);
            
            // STATIC control with left-aligned text
            // SS_LEFT = 0x0, so we can omit it or use 0
            let style = WS_CHILD | WS_VISIBLE;
            
            let hwnd = CreateWindowExW(
                WINDOW_EX_STYLE(0),
                windows::core::w!("STATIC"),
                windows::core::PCWSTR(label_text.as_ptr()),
                style,
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
                return Err(Error::WidgetCreation("Label creation failed".into()));
            }
            
            tracing::debug!("Label created successfully: HWND={:?}", hwnd);
            
            Ok(Self { hwnd })
        }
    }
    
    fn set_text(&mut self, text: &str) -> Result<()> {
        unsafe {
            let wide_text = to_wide_string(text);
            SetWindowTextW(self.hwnd, windows::core::PCWSTR(wide_text.as_ptr()))
                .map_err(|e| Error::OperationFailed(format!("SetWindowTextW failed: {}", e)))?;
        }
        Ok(())
    }
    
    fn set_bounds(&mut self, x: i32, y: i32, width: u32, height: u32) -> Result<()> {
        unsafe {
            SetWindowPos(
                self.hwnd,
                HWND::default(),
                x,
                y,
                width as i32,
                height as i32,
                SWP_NOZORDER | SWP_NOACTIVATE,
            ).map_err(|e| Error::OperationFailed(format!("SetWindowPos failed: {}", e)))?;
        }
        Ok(())
    }
}

impl Win32Label {
    /// Get the raw HWND handle
    pub fn hwnd(&self) -> HWND {
        self.hwnd
    }
}

impl Drop for Win32Label {
    fn drop(&mut self) {
        tracing::debug!("Destroying label: HWND={:?}", self.hwnd);
        unsafe {
            let _ = DestroyWindow(self.hwnd);
        }
    }
}
