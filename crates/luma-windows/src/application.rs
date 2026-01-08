use windows::Win32::UI::WindowsAndMessaging::*;
use luma_core::{Result, traits::ApplicationBackend};
use once_cell::sync::Lazy;
use std::sync::Mutex;

/// Global application instance
static APP_RUNNING: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));

/// Win32 application backend
pub struct Win32Application {
    running: bool,
}

impl ApplicationBackend for Win32Application {
    fn new() -> Result<Self> {
        tracing::info!("Initializing Win32 application");
        
        Ok(Self {
            running: false,
        })
    }
    
    fn run(&mut self) -> Result<()> {
        self.running = true;
        *APP_RUNNING.lock().unwrap() = true;
        
        tracing::info!("Starting Win32 message loop");
        
        unsafe {
            let mut msg = MSG::default();
            
            while GetMessageW(&mut msg, None, 0, 0).as_bool() {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }
        
        self.running = false;
        *APP_RUNNING.lock().unwrap() = false;
        
        tracing::info!("Win32 message loop ended");
        
        Ok(())
    }
    
    fn quit(&mut self) -> Result<()> {
        unsafe {
            PostQuitMessage(0);
        }
        self.running = false;
        Ok(())
    }
}

impl Win32Application {
    /// Check if the application is running
    pub fn is_running() -> bool {
        *APP_RUNNING.lock().unwrap()
    }
}
