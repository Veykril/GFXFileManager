use std::os::raw::{c_char, c_int};
use winapi::{FILETIME, WIN32_FIND_DATAA};


#[repr(C)]
#[allow(non_snake_case)]
pub struct ResultEntry {
    low_date_time: c_int,
    file_time: FILETIME,
    high_date_time: c_int,
    field_10: c_int,
    field_14: c_int,
    field_18: c_int,
    field_1C: c_int,
    size: c_int,
    typ: c_char,
    filename: [c_char; 89],
    find_datqa: WIN32_FIND_DATAA,
}