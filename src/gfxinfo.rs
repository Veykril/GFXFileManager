use winapi::{c_char, c_int, c_ulonglong};
use winapi::SYSTEMTIME;

use ffi::GFXFMInfo;

#[repr(C)]
pub struct GFXInfo {
    pub in_use: c_char,
    pub index: c_int,
    pub number_of_open_files: c_int,
    pub filename: [c_char; 256],
    pub field_0: c_int,
    pub field_1: c_int,
    pub field_2: c_int,
    pub number_of_bytes_processed_total: c_ulonglong,
    pub timestamp: SYSTEMTIME,
    pub pid: c_int,
    pub field_3: c_int,
}

impl GFXInfo {
    pub fn new(index: c_int) -> Self {
        unsafe {
            let mut object = ::std::mem::uninitialized();
            GFXFMInfo(&mut object, index);
            object
        }
    }
}