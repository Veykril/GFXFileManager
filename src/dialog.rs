use winapi::{c_char, c_int};
use winapi::HWND;

#[repr(C)]
pub struct DialogData {
    hwnd: HWND,
    mode: c_int,
    filter: *const c_char,
    selected_dir: *mut c_char,
    selected_file: *mut c_char,
    unknown: c_int,
}