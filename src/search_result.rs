use winapi::{c_int, c_char, c_uchar};
use winapi::HANDLE;

//todo wrapper class that automagically calls find_close() on drop

#[repr(C)]
#[allow(non_snake_case)]
pub struct SearchResult {
    pub success: c_uchar,
    pub field_4: c_int,
    pub field_8: c_int,
    pub field_C: c_int,
    pub field_10: c_int,
    pub field_14: c_int,
    pub field_18: c_int,
    pub field_1C: c_int,
    pub field_20: c_int,
    pub field_24: c_int,
    pub field_28: c_int,
    pub field_2C: c_int,
    pub field_30: c_int,
    pub field_34: c_int,
    pub field_38: c_int,
    pub field_3C: c_int,
    pub gap40: [c_char; 840],
    pub hFind: HANDLE,
    pub field_38C: c_int
}