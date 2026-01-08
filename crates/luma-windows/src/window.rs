use windows::Win32::Foundation::*;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::Win32::Graphics::Gdi::*;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use luma_core::{Result, Error, WindowFlags, traits::WindowBackend, Container, Size};
use crate::utils::{to_wide_string, is_valid_hwnd};
use once_cell::sync::OnceCell;
use std::sync::Mutex;
use std::collections::HashMap;

/// Window class name for Luma windows
const WINDOW_CLASS_NAME: &str = "LumaWindow";

/// Ensure the window class is registered (only once)
static WINDOW_CLASS_REGISTERED: OnceCell<()> = OnceCell::new();

/// Wrapper to make raw pointer Send (unsafe but necessary for Win32 callback)
struct LayoutPtr(*mut dyn Container);
unsafe impl Send for LayoutPtr {}

/// Global map of HWND to layout for handling WM_SIZE
static WINDOW_LAYOUTS: OnceCell<Mutex<HashMap<isize, LayoutPtr>>> = OnceCell::new();

fn get_layouts_map() -> &'static Mutex<HashMap<isize, LayoutPtr>> {
    WINDOW_LAYOUTS.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Wrapper to make callback pointer Send
struct CallbackPtr(*mut dyn FnMut());
unsafe impl Send for CallbackPtr {}

/// Global map of widget HWND to callback for handling WM_COMMAND
static WIDGET_CALLBACKS: OnceCell<Mutex<HashMap<isize, CallbackPtr>>> = OnceCell::new();

fn get_callbacks_map() -> &'static Mutex<HashMap<isize, CallbackPtr>> {
    WIDGET_CALLBACKS.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Register a callback for a widget HWND
pub fn register_callback(hwnd: isize, callback: *mut dyn FnMut()) {
    let mut map = get_callbacks_map().lock().unwrap();
    map.insert(hwnd, CallbackPtr(callback));
    tracing::debug!("Registered callback for widget HWND={:?}", hwnd);
}

/// Unregister a callback for a widget HWND
pub fn unregister_callback(hwnd: isize) {
    let mut map = get_callbacks_map().lock().unwrap();
    map.remove(&hwnd);
    tracing::debug!("Unregistered callback for widget HWND={:?}", hwnd);
}

/// Win32 window backend
pub struct Win32Window {
    hwnd: HWND,
}

impl Win32Window {
    /// Register a layout for this window (for resize handling)
    pub fn set_layout_ptr(&self, layout: *mut dyn Container) {
        let mut map = get_layouts_map().lock().unwrap();
        map.insert(self.hwnd.0, LayoutPtr(layout));
        tracing::debug!("Registered layout for HWND={:?}", self.hwnd);
    }
    
    /// Unregister the layout for this window
    pub fn clear_layout_ptr(&self) {
        let mut map = get_layouts_map().lock().unwrap();
        map.remove(&self.hwnd.0);
        tracing::debug!("Unregistered layout for HWND={:?}", self.hwnd);
    }
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
    
    fn get_client_size(&self) -> Result<luma_core::Size> {
        unsafe {
            let mut rect = RECT::default();
            if GetClientRect(self.hwnd, &mut rect).is_ok() {
                let width = (rect.right - rect.left) as u32;
                let height = (rect.bottom - rect.top) as u32;
                Ok(luma_core::Size::new(width, height))
            } else {
                Err(Error::OperationFailed("GetClientRect failed".into()))
            }
        }
    }
}

impl Drop for Win32Window {
    fn drop(&mut self) {
        tracing::debug!("Destroying Win32 window: HWND={:?}", self.hwnd);
        // Clean up layout registration
        self.clear_layout_ptr();
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
        WM_SIZE => {
            // Handle window resize - re-layout all widgets
            let width = (lparam.0 & 0xFFFF) as u32;
            let height = ((lparam.0 >> 16) & 0xFFFF) as u32;
            
            // Get the layout for this window and trigger re-layout
            if let Ok(map) = get_layouts_map().lock() {
                if let Some(layout_ptr) = map.get(&hwnd.0) {
                    if !layout_ptr.0.is_null() {
                        let layout = &mut *layout_ptr.0;
                        let new_size = Size::new(width, height);
                        
                        if let Err(e) = layout.layout(new_size) {
                            tracing::error!("Layout failed during resize: {}", e);
                        }
                    }
                }
            }
            
            DefWindowProcW(hwnd, msg, wparam, lparam)
        }
        WM_COMMAND => {
            // Handle button clicks, checkbox changes, listbox selections
            // HIWORD(wparam) = notification code, LOWORD(wparam) = control ID
            // lparam = control HWND
            let control_hwnd = HWND(lparam.0 as isize);
            let notification_code = ((wparam.0 >> 16) & 0xFFFF) as u32;
            
            tracing::debug!(
                "WM_COMMAND: control_hwnd={:?}, notification_code={}",
                control_hwnd,
                notification_code
            );
            
            // Look up and invoke callback
            if let Ok(mut map) = get_callbacks_map().lock() {
                if let Some(callback_ptr) = map.get_mut(&control_hwnd.0) {
                    if !callback_ptr.0.is_null() {
                        // Safety: Callback pointer is valid as long as widget exists
                        // Widget Drop implementations must unregister callbacks
                        let callback = &mut *callback_ptr.0;
                        callback();
                    }
                }
            }
            
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
