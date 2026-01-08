use windows::Win32::Foundation::*;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use luma_core::{Result, Error, Point, Size, ButtonFlags, traits::ButtonBackend};
use crate::utils::{to_wide_string, is_valid_hwnd};

/// Win32 button backend
pub struct Win32Button {
    hwnd: HWND,
}

impl ButtonBackend for Win32Button {
    fn new(
        parent_hwnd: *mut std::ffi::c_void,
        label: &str,
        pos: Point,
        size: Size,
        flags: ButtonFlags,
    ) -> Result<Self> {
        tracing::debug!(
            "Creating Win32 button: label='{}', pos=({}, {}), size={}x{}",
            label,
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
            let button_text = to_wide_string(label);
            let style = button_flags_to_style(flags);
            
            let hwnd = CreateWindowExW(
                WINDOW_EX_STYLE(0),
                windows::core::w!("BUTTON"),
                windows::core::PCWSTR(button_text.as_ptr()),
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
                return Err(Error::WidgetCreation("Button creation failed".into()));
            }
            
            tracing::debug!("Button created successfully: HWND={:?}", hwnd);
            
            Ok(Self { hwnd })
        }
    }
    
    fn set_label(&mut self, label: &str) -> Result<()> {
        unsafe {
            let wide_label = to_wide_string(label);
            SetWindowTextW(self.hwnd, windows::core::PCWSTR(wide_label.as_ptr()))
                .map_err(|e| Error::OperationFailed(format!("SetWindowTextW failed: {}", e)))?;
        }
        Ok(())
    }
    
    fn set_enabled(&mut self, enabled: bool) -> Result<()> {
        // TODO: Implement EnableWindow once we figure out the correct import
        // For now, this is a no-op
        tracing::warn!("set_enabled not yet implemented: {}", enabled);
        Ok(())
    }
}

impl Win32Button {
    /// Get the raw HWND handle
    pub fn hwnd(&self) -> HWND {
        self.hwnd
    }
}

impl Drop for Win32Button {
    fn drop(&mut self) {
        tracing::debug!("Destroying button: HWND={:?}", self.hwnd);
        unsafe {
            let _ = DestroyWindow(self.hwnd);
        }
    }
}

/// Convert ButtonFlags to Win32 button style
fn button_flags_to_style(flags: ButtonFlags) -> WINDOW_STYLE {
    let mut style = WS_CHILD | WS_VISIBLE | WS_TABSTOP | WINDOW_STYLE(BS_PUSHBUTTON as u32);
    
    if flags.contains(ButtonFlags::DEFAULT) {
        style |= WINDOW_STYLE(BS_DEFPUSHBUTTON as u32);
    }
    if flags.contains(ButtonFlags::TOGGLE) {
        style &= !WINDOW_STYLE(BS_PUSHBUTTON as u32);
        style |= WINDOW_STYLE(BS_AUTOCHECKBOX as u32);
    }
    
    style
}
