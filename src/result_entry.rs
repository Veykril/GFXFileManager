use std::os::raw::{c_char, c_int};
use winapi::{FILETIME, WIN32_FIND_DATAA};


pub const ENTRY_FOLDER: c_char = 1;
pub const ENTRY_FILE: c_char = 2;


#[repr(C)]
#[allow(non_snake_case)]
pub struct ResultEntry {
    pub low_date_time: c_int,
    pub file_time: FILETIME,
    pub high_date_time: c_int,
    pub field_10: c_int,
    pub field_14: c_int,
    pub field_18: c_int,
    pub field_1C: c_int,
    pub size: c_int,
    pub typ: c_char,
    pub filename: [c_char; 89],
    pub find_datqa: WIN32_FIND_DATAA,
}