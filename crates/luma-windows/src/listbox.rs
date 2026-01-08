use windows::Win32::Foundation::*;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use luma_core::{Result, Error, Point, Size, ListBoxFlags, traits::ListBoxBackend};
use crate::utils::{to_wide_string, from_wide_string, is_valid_hwnd};

// ListBox constants and messages
const LB_ADDSTRING: u32 = 0x0180;
const LB_DELETESTRING: u32 = 0x0182;
const LB_RESETCONTENT: u32 = 0x0184;
const LB_GETCOUNT: u32 = 0x018B;
const LB_GETCURSEL: u32 = 0x0188;
const LB_SETCURSEL: u32 = 0x0186;
const LB_GETSELCOUNT: u32 = 0x0190;
const LB_GETSELITEMS: u32 = 0x0191;
const LB_GETTEXTLEN: u32 = 0x018A;
const LB_GETTEXT: u32 = 0x0189;
const LB_ERR: i32 = -1;
const LB_ERRSPACE: i32 = -2;

// ListBox styles
const LBS_NOTIFY: u32 = 0x0001;
const LBS_SORT: u32 = 0x0002;
const LBS_MULTIPLESEL: u32 = 0x0008;

/// Win32 listbox backend
pub struct Win32ListBox {
    hwnd: HWND,
    flags: ListBoxFlags,
}

impl ListBoxBackend for Win32ListBox {
    fn new(
        parent_hwnd: *mut std::ffi::c_void,
        pos: Point,
        size: Size,
        flags: ListBoxFlags,
    ) -> Result<Self> {
        tracing::debug!(
            "Creating Win32 listbox: pos=({}, {}), size={}x{}, flags={:?}",
            pos.x,
            pos.y,
            size.width,
            size.height,
            flags
        );
        
        unsafe {
            let hinstance = GetModuleHandleW(None).map_err(|e| {
                Error::Platform(format!("Failed to get module handle: {}", e))
            })?;
            
            let parent = HWND(parent_hwnd as isize);
            let style = listbox_flags_to_style(flags);
            
            let hwnd = CreateWindowExW(
                WINDOW_EX_STYLE(WS_EX_CLIENTEDGE.0), // Sunken border
                windows::core::w!("LISTBOX"),
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
                return Err(Error::WidgetCreation("ListBox creation failed".into()));
            }
            
            tracing::debug!("ListBox created successfully: HWND={:?}", hwnd);
            
            Ok(Self { hwnd, flags })
        }
    }
    
    fn add_item(&mut self, item: &str) -> Result<()> {
        unsafe {
            let wide_item = to_wide_string(item);
            let result = SendMessageW(
                self.hwnd,
                LB_ADDSTRING,
                WPARAM(0),
                LPARAM(wide_item.as_ptr() as isize),
            );
            
            if result.0 == LB_ERR as isize || result.0 == LB_ERRSPACE as isize {
                return Err(Error::OperationFailed("Failed to add item to listbox".into()));
            }
        }
        Ok(())
    }
    
    fn remove_item(&mut self, index: usize) -> Result<()> {
        unsafe {
            let result = SendMessageW(
                self.hwnd,
                LB_DELETESTRING,
                WPARAM(index),
                LPARAM(0),
            );
            
            if result.0 == LB_ERR as isize {
                return Err(Error::InvalidParameter(format!("Invalid index: {}", index)));
            }
        }
        Ok(())
    }
    
    fn clear(&mut self) -> Result<()> {
        unsafe {
            SendMessageW(self.hwnd, LB_RESETCONTENT, WPARAM(0), LPARAM(0));
        }
        Ok(())
    }
    
    fn item_count(&self) -> Result<usize> {
        unsafe {
            let count = SendMessageW(self.hwnd, LB_GETCOUNT, WPARAM(0), LPARAM(0));
            if count.0 == LB_ERR as isize {
                return Err(Error::OperationFailed("Failed to get item count".into()));
            }
            Ok(count.0 as usize)
        }
    }
    
    fn get_selected_index(&self) -> Result<Option<usize>> {
        if self.flags.contains(ListBoxFlags::MULTI_SELECT) {
            return Err(Error::OperationFailed(
                "Use get_selected_indices() for multi-select listbox".into()
            ));
        }
        
        unsafe {
            let index = SendMessageW(self.hwnd, LB_GETCURSEL, WPARAM(0), LPARAM(0));
            if index.0 == LB_ERR as isize {
                Ok(None) // No selection
            } else {
                Ok(Some(index.0 as usize))
            }
        }
    }
    
    fn get_selected_indices(&self) -> Result<Vec<usize>> {
        if !self.flags.contains(ListBoxFlags::MULTI_SELECT) {
            // For single-select, return 0 or 1 items
            return match self.get_selected_index()? {
                Some(idx) => Ok(vec![idx]),
                None => Ok(vec![]),
            };
        }
        
        unsafe {
            // Get number of selected items
            let sel_count = SendMessageW(self.hwnd, LB_GETSELCOUNT, WPARAM(0), LPARAM(0));
            if sel_count.0 == LB_ERR as isize {
                return Err(Error::OperationFailed("Failed to get selection count".into()));
            }
            
            if sel_count.0 == 0 {
                return Ok(vec![]);
            }
            
            // Get selected indices
            let mut indices: Vec<i32> = vec![0; sel_count.0 as usize];
            let result = SendMessageW(
                self.hwnd,
                LB_GETSELITEMS,
                WPARAM(sel_count.0 as usize),
                LPARAM(indices.as_mut_ptr() as isize),
            );
            
            if result.0 == LB_ERR as isize {
                return Err(Error::OperationFailed("Failed to get selected indices".into()));
            }
            
            Ok(indices.iter().map(|&i| i as usize).collect())
        }
    }
    
    fn set_selected_index(&mut self, index: Option<usize>) -> Result<()> {
        if self.flags.contains(ListBoxFlags::MULTI_SELECT) {
            return Err(Error::OperationFailed(
                "Use selection methods specific to multi-select for multi-select listbox".into()
            ));
        }
        
        unsafe {
            let wparam = match index {
                Some(idx) => WPARAM(idx),
                None => WPARAM(usize::MAX), // LB_ERR (-1) to deselect
            };
            
            let result = SendMessageW(self.hwnd, LB_SETCURSEL, wparam, LPARAM(0));
            
            if result.0 == LB_ERR as isize && index.is_some() {
                return Err(Error::InvalidParameter(format!("Invalid index: {:?}", index)));
            }
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

impl Win32ListBox {
    /// Get the raw HWND handle
    pub fn hwnd(&self) -> HWND {
        self.hwnd
    }
    
    /// Get item text by index
    pub fn get_item_text(&self, index: usize) -> Result<String> {
        unsafe {
            // Get text length
            let len = SendMessageW(
                self.hwnd,
                LB_GETTEXTLEN,
                WPARAM(index),
                LPARAM(0),
            );
            
            if len.0 == LB_ERR as isize {
                return Err(Error::InvalidParameter(format!("Invalid index: {}", index)));
            }
            
            if len.0 == 0 {
                return Ok(String::new());
            }
            
            // Get text
            let mut buffer: Vec<u16> = vec![0; (len.0 + 1) as usize];
            let result = SendMessageW(
                self.hwnd,
                LB_GETTEXT,
                WPARAM(index),
                LPARAM(buffer.as_mut_ptr() as isize),
            );
            
            if result.0 == LB_ERR as isize {
                return Err(Error::OperationFailed("Failed to get item text".into()));
            }
            
            buffer.truncate(len.0 as usize);
            Ok(from_wide_string(&buffer))
        }
    }
}

impl Drop for Win32ListBox {
    fn drop(&mut self) {
        tracing::debug!("Destroying listbox: HWND={:?}", self.hwnd);
        unsafe {
            let _ = DestroyWindow(self.hwnd);
        }
    }
}

/// Convert ListBoxFlags to Win32 listbox style
fn listbox_flags_to_style(flags: ListBoxFlags) -> WINDOW_STYLE {
    let mut style = WS_CHILD | WS_VISIBLE | WS_TABSTOP | WS_BORDER | 
                    WINDOW_STYLE(LBS_NOTIFY as u32); // Send notifications
    
    if flags.contains(ListBoxFlags::MULTI_SELECT) {
        style |= WINDOW_STYLE(LBS_MULTIPLESEL as u32);
    }
    
    if flags.contains(ListBoxFlags::SORTED) {
        style |= WINDOW_STYLE(LBS_SORT as u32);
    }
    
    if flags.contains(ListBoxFlags::VSCROLL) {
        style |= WS_VSCROLL;
    }
    
    if flags.contains(ListBoxFlags::HSCROLL) {
        style |= WS_HSCROLL;
    }
    
    style
}
