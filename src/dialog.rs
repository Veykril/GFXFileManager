use winapi::ctypes::{c_char, c_int};
use winapi::shared::windef::HWND;

#[repr(C)]
pub struct DialogData {
    pub hwnd: HWND,
    pub mode: c_int,
    pub filter: *const c_char,
    pub selected_dir: *mut c_char,
    pub selected_file: *mut c_char,
    pub unknown: c_int,
}
