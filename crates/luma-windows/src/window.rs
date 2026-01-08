use windows::Win32::Foundation::*;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::Win32::Graphics::Gdi::*;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use luma_core::{Result, Error, WindowFlags, traits::WindowBackend};
use crate::utils::{to_wide_string, is_valid_hwnd};
use once_cell::sync::OnceCell;

/// Window class name for Luma windows
const WINDOW_CLASS_NAME: &str = "LumaWindow";

/// Ensure the window class is registered (only once)
static WINDOW_CLASS_REGISTERED: OnceCell<()> = OnceCell::new();

/// Win32 window backend
pub struct Win32Window {
    hwnd: HWND,
}

impl WindowBackend for Win32Window {
    fn new(title: &str, width: u32, height: u32, flags: WindowFlags) -> Result<Self> {
        tracing::info!("Creating Win32 window: title='{}', size={}x{}", title, width, height);
        
        // Ensure window class is registered
        WINDOW_CLASS_REGISTERED.get_or_try_init(|| register_window_class())?;
        
        unsafe {
            let hinstance = GetModuleHandleW(None).map_err(|e| {
                Error::Platform(format!("Failed to get module handle: {}", e))
            })?;
            
            let class_name = to_wide_string(WINDOW_CLASS_NAME);
            let window_title = to_wide_string(title);
            
            let style = window_flags_to_style(flags);
            
            let hwnd = CreateWindowExW(
                WINDOW_EX_STYLE(0),
                windows::core::PCWSTR(class_name.as_ptr()),
                windows::core::PCWSTR(window_title.as_ptr()),
                style,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                width as i32,
                height as i32,
                None,
                None,
                hinstance,
                None,
            );
            
            if !is_valid_hwnd(hwnd) {
                return Err(Error::WindowCreation("CreateWindowExW failed".into()));
            }
            
            tracing::debug!("Win32 window created successfully: HWND={:?}", hwnd);
            
            Ok(Self { hwnd })
        }
    }
    
    fn set_title(&mut self, title: &str) -> Result<()> {
        unsafe {
            let wide_title = to_wide_string(title);
            SetWindowTextW(self.hwnd, windows::core::PCWSTR(wide_title.as_ptr()))
                .map_err(|e| Error::OperationFailed(format!("SetWindowTextW failed: {}", e)))?;
        }
        Ok(())
    }
    
    fn set_size(&mut self, width: u32, height: u32) -> Result<()> {
        unsafe {
            SetWindowPos(
                self.hwnd,
                None,
                0,
                0,
                width as i32,
                height as i32,
                SWP_NOMOVE | SWP_NOZORDER,
            ).map_err(|e| Error::OperationFailed(format!("SetWindowPos failed: {}", e)))?;
        }
        Ok(())
    }
    
    fn show(&mut self) -> Result<()> {
        unsafe {
            ShowWindow(self.hwnd, SW_SHOW);
            UpdateWindow(self.hwnd);
        }
        tracing::debug!("Window shown");
        Ok(())
    }
    
    fn hide(&mut self) -> Result<()> {
        unsafe {
            ShowWindow(self.hwnd, SW_HIDE);
        }
        Ok(())
    }
    
    fn raw_handle(&self) -> *mut std::ffi::c_void {
        self.hwnd.0 as *mut std::ffi::c_void
    }
}

impl Drop for Win32Window {
    fn drop(&mut self) {
        tracing::debug!("Destroying Win32 window: HWND={:?}", self.hwnd);
        unsafe {
            let _ = DestroyWindow(self.hwnd);
        }
    }
}

/// Register the window class for Luma windows
fn register_window_class() -> Result<()> {
    tracing::debug!("Registering window class: {}", WINDOW_CLASS_NAME);
    
    unsafe {
        let hinstance = GetModuleHandleW(None).map_err(|e| {
            Error::Platform(format!("Failed to get module handle: {}", e))
        })?;
        
        let class_name = to_wide_string(WINDOW_CLASS_NAME);
        
        let wc = WNDCLASSW {
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(window_proc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: hinstance.into(),
            hIcon: LoadIconW(None, IDI_APPLICATION).ok().unwrap_or_default(),
            hCursor: LoadCursorW(None, IDC_ARROW).ok().unwrap_or_default(),
            hbrBackground: HBRUSH((COLOR_WINDOW.0 + 1) as isize),
            lpszMenuName: windows::core::PCWSTR::null(),
            lpszClassName: windows::core::PCWSTR(class_name.as_ptr()),
        };
        
        let atom = RegisterClassW(&wc);
        if atom == 0 {
            return Err(Error::Platform("RegisterClassW failed".into()));
        }
        
        tracing::debug!("Window class registered successfully");
        Ok(())
    }
}

/// Window procedure for handling messages
unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_DESTROY => {
            tracing::debug!("WM_DESTROY received");
            PostQuitMessage(0);
            LRESULT(0)
        }
        WM_CLOSE => {
            tracing::debug!("WM_CLOSE received");
            let _ = DestroyWindow(hwnd);
            LRESULT(0)
        }
        WM_PAINT => {
            let mut ps = PAINTSTRUCT::default();
            let hdc = BeginPaint(hwnd, &mut ps);
            // Paint background
            FillRect(hdc, &ps.rcPaint, HBRUSH((COLOR_WINDOW.0 + 1) as isize));
            EndPaint(hwnd, &ps);
            LRESULT(0)
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

/// Convert WindowFlags to Win32 WINDOW_STYLE
fn window_flags_to_style(flags: WindowFlags) -> WINDOW_STYLE {
    let mut style = WS_OVERLAPPEDWINDOW;
    
    if !flags.contains(WindowFlags::RESIZABLE) {
        style &= !WS_THICKFRAME;
    }
    if !flags.contains(WindowFlags::MINIMIZABLE) {
        style &= !WS_MINIMIZEBOX;
    }
    if !flags.contains(WindowFlags::MAXIMIZABLE) {
        style &= !WS_MAXIMIZEBOX;
    }
    if !flags.contains(WindowFlags::CLOSABLE) {
        style &= !(WS_SYSMENU);
    }
    if !flags.contains(WindowFlags::TITLED) {
        style = WS_POPUP | WS_BORDER;
    }
    
    style
}
