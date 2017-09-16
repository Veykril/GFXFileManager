use winapi::{c_char, c_int, c_ulonglong};
use winapi::SYSTEMTIME;

#[repr(C)]
pub struct GFXInfo {
    in_use: c_char,
    index: c_int,
    number_of_open_files: c_int,
    filename: [c_char; 256],
    field_0: c_int,
    field_1: c_int,
    field_2: c_int,
    number_of_bytes_processed_total: c_ulonglong,
    timestamp: SYSTEMTIME,
    pid: c_int,
    field_3: c_int,
}