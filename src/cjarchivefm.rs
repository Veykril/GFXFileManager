use std::os::raw::{c_char, c_int, c_uchar};
use IFileManager;

#[repr(C)]
pub struct CJArchiveFm {
    destructor: extern "thiscall" fn(*mut CJArchiveFm),
    p_filemanager: *mut IFileManager,
    h_file: c_int,
    field_0: c_int,
    field_1: c_int,
    is_write_mode: c_uchar,
    field_2: c_uchar,
    padding: [c_char; 0xA],
    p_current: *mut c_char,
    p_end: *mut c_char,
    buffer: [c_char; 4096],
}