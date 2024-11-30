use winapi::ctypes::{c_char, c_int, c_uchar};

use crate::file_manager::IFileManager;

#[repr(C)]
pub struct CJArchiveFm {
    //this structure needs testing as well as the functions that use it
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

impl Drop for CJArchiveFm {
    fn drop(&mut self) {
        (self.destructor)(self)
    }
}
