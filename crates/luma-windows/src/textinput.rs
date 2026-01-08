use windows::Win32::Foundation::*;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use luma_core::{Result, Error, Point, Size, traits::TextInputBackend};
use crate::utils::{to_wide_string, from_wide_string, is_valid_hwnd};

// Edit control styles
const ES_LEFT: u32 = 0x0000;
const ES_AUTOHSCROLL: u32 = 0x0080;
const ES_READONLY: u32 = 0x0800;

// Edit control messages
const EM_SETREADONLY: u32 = 0x00CF;

/// Win32 text input backend (EDIT control)
pub struct Win32TextInput {
    hwnd: HWND,
}

impl TextInputBackend for Win32TextInput {
    fn new(
        parent_hwnd: *mut std::ffi::c_void,
        pos: Point,
        size: Size,
        read_only: bool,
    ) -> Result<Self> {
        tracing::debug!(
            "Creating Win32 text input: pos=({}, {}), size={}x{}, read_only={}",
            pos.x,
            pos.y,
            size.width,
            size.height,
            read_only
        );
        
        unsafe {
            let hinstance = GetModuleHandleW(None).map_err(|e| {
                Error::Platform(format!("Failed to get module handle: {}", e))
            })?;
            
            let parent = HWND(parent_hwnd as isize);
            
            // EDIT control with ES_LEFT (left-aligned), ES_AUTOHSCROLL (auto-scroll)
            let mut style = WS_CHILD | WS_VISIBLE | WS_TABSTOP | WS_BORDER | 
                            WINDOW_STYLE(ES_LEFT as u32 | ES_AUTOHSCROLL as u32);
            
            if read_only {
                style |= WINDOW_STYLE(ES_READONLY as u32);
            }
            
            let hwnd = CreateWindowExW(
                WINDOW_EX_STYLE(WS_EX_CLIENTEDGE.0), // Sunken border
                windows::core::w!("EDIT"),
                windows::core::PCWSTR::null(),
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
                return Err(Error::WidgetCreation("TextInput creation failed".into()));
            }
            
            tracing::debug!("TextInput created successfully: HWND={:?}", hwnd);
            
            Ok(Self { hwnd })
        }
    }
    
    fn get_text(&self) -> Result<String> {
        unsafe {
            let len = GetWindowTextLengthW(self.hwnd);
            if len == 0 {
                return Ok(String::new());
            }
            
            let mut buffer: Vec<u16> = vec![0; (len + 1) as usize];
            let actual_len = GetWindowTextW(self.hwnd, &mut buffer);
            
            if actual_len == 0 {
                return Ok(String::new());
            }
            
            buffer.truncate(actual_len as usize);
            Ok(from_wide_string(&buffer))
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
    
    fn set_read_only(&mut self, read_only: bool) -> Result<()> {
        unsafe {
            SendMessageW(
                self.hwnd,
                EM_SETREADONLY,
                WPARAM(if read_only { 1 } else { 0 }),
                LPARAM(0),
            );
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

impl Win32TextInput {
    /// Get the raw HWND handle
    pub fn hwnd(&self) -> HWND {
        self.hwnd
    }
}

impl Drop for Win32TextInput {
    fn drop(&mut self) {
        tracing::debug!("Destroying text input: HWND={:?}", self.hwnd);
        unsafe {
            let _ = DestroyWindow(self.hwnd);
        }
    }
}
