use winapi::{c_int, c_char};
use winapi::HWND;

use result_entry::ResultEntry;

pub type ErrorHandler = extern "thiscall" fn(HWND, *const c_char, *const c_char) -> bool;
pub type ForEachCallback = extern "thiscall" fn(CallbackState, ResultEntry, *mut ::std::os::raw::c_void) -> bool;

pub enum CallbackState {
    Init = 0,
    EnterDir = 1,
    LeaveDir = 2,
    File = 3
}

#[repr(C)]
pub struct DialogData {
    hwnd: HWND,
    mode: c_int,
    filter: *const c_char,
    selected_dir: *mut c_char,
    selected_file: *mut c_char,
    unknown: c_int,
}

#[repr(C)]
pub struct UnknownPair {
    a: c_int,
    b: c_int
}
