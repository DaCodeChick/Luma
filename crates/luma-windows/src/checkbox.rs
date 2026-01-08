use windows::Win32::Foundation::*;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use luma_core::{Result, Error, Point, Size, traits::CheckBoxBackend};
use crate::utils::{to_wide_string, is_valid_hwnd};

// Button styles and states
const BS_AUTOCHECKBOX: u32 = 0x0003;

// Button state constants
const BST_UNCHECKED: u32 = 0x0000;
const BST_CHECKED: u32 = 0x0001;

/// Win32 checkbox backend (BUTTON control with BS_AUTOCHECKBOX style)
pub struct Win32CheckBox {
    hwnd: HWND,
}

impl CheckBoxBackend for Win32CheckBox {
    fn new(
        parent_hwnd: *mut std::ffi::c_void,
        label: &str,
        pos: Point,
        size: Size,
        checked: bool,
    ) -> Result<Self> {
        tracing::debug!(
            "Creating Win32 checkbox: label='{}', pos=({}, {}), size={}x{}, checked={}",
            label,
            pos.x,
            pos.y,
            size.width,
            size.height,
            checked
        );
        
        unsafe {
            let hinstance = GetModuleHandleW(None).map_err(|e| {
                Error::Platform(format!("Failed to get module handle: {}", e))
            })?;
            
            let parent = HWND(parent_hwnd as isize);
            let checkbox_text = to_wide_string(label);
            
            // BS_AUTOCHECKBOX automatically toggles on click
            let style = WS_CHILD | WS_VISIBLE | WS_TABSTOP | 
                       WINDOW_STYLE(BS_AUTOCHECKBOX as u32);
            
            let hwnd = CreateWindowExW(
                WINDOW_EX_STYLE(0),
                windows::core::w!("BUTTON"),
                windows::core::PCWSTR(checkbox_text.as_ptr()),
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
                return Err(Error::WidgetCreation("CheckBox creation failed".into()));
            }
            
            // Set initial checked state
            if checked {
                SendMessageW(hwnd, BM_SETCHECK, WPARAM(BST_CHECKED as usize), LPARAM(0));
            }
            
            tracing::debug!("CheckBox created successfully: HWND={:?}", hwnd);
            
            Ok(Self { hwnd })
        }
    }
    
    fn is_checked(&self) -> Result<bool> {
        unsafe {
            let state = SendMessageW(self.hwnd, BM_GETCHECK, WPARAM(0), LPARAM(0));
            Ok(state.0 as u32 == BST_CHECKED)
        }
    }
    
    fn set_checked(&mut self, checked: bool) -> Result<()> {
        unsafe {
            let check_state = if checked { BST_CHECKED } else { BST_UNCHECKED };
            SendMessageW(
                self.hwnd,
                BM_SETCHECK,
                WPARAM(check_state as usize),
                LPARAM(0),
            );
        }
        Ok(())
    }
    
    fn set_label(&mut self, label: &str) -> Result<()> {
        unsafe {
            let wide_label = to_wide_string(label);
            SetWindowTextW(self.hwnd, windows::core::PCWSTR(wide_label.as_ptr()))
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

impl Win32CheckBox {
    /// Get the raw HWND handle
    pub fn hwnd(&self) -> HWND {
        self.hwnd
    }
}

impl Drop for Win32CheckBox {
    fn drop(&mut self) {
        tracing::debug!("Destroying checkbox: HWND={:?}", self.hwnd);
        unsafe {
            let _ = DestroyWindow(self.hwnd);
        }
    }
}
