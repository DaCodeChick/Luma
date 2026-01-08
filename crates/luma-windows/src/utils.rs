use windows::core::PCWSTR;
use windows::Win32::Foundation::HWND;

/// Convert a Rust string to a wide (UTF-16) string for Windows APIs
pub fn to_wide_string(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

/// Convert a wide string slice to a Rust String
pub fn from_wide_string(wide: &[u16]) -> String {
    String::from_utf16_lossy(wide)
}

/// Convert a wide string pointer to a Rust String
/// 
/// # Safety
/// The pointer must be valid and null-terminated
pub unsafe fn from_wide_ptr(ptr: PCWSTR) -> String {
    let len = (0..).take_while(|&i| *ptr.0.offset(i) != 0).count();
    let slice = std::slice::from_raw_parts(ptr.0, len);
    String::from_utf16_lossy(slice)
}

/// Check if an HWND is valid (non-null)
pub fn is_valid_hwnd(hwnd: HWND) -> bool {
    hwnd.0 != 0
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_to_wide_string() {
        let wide = to_wide_string("Hello");
        assert_eq!(wide.len(), 6); // 5 chars + null terminator
        assert_eq!(wide[0], 'H' as u16);
        assert_eq!(wide[5], 0); // null terminator
    }
    
    #[test]
    fn test_from_wide_string() {
        let wide: Vec<u16> = vec!['H' as u16, 'i' as u16];
        let s = from_wide_string(&wide);
        assert_eq!(s, "Hi");
    }
}
