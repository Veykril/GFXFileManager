use std::ffi::CStr;

use winapi::ctypes::{c_char, c_int};
use winapi::shared::minwindef::FILETIME;
use winapi::um::minwinbase::WIN32_FIND_DATAA;

pub enum Entry {
    Directory = 1,
    File = 2,
}

impl From<i8> for Entry {
    fn from(i: i8) -> Entry {
        match i {
            1 => Entry::Directory,
            2 => Entry::File,
            _ => panic!("Invalid Entry type"),
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct ResultEntry {
    pub low_date_time: c_int,
    file_time: FILETIME,
    pub high_date_time: c_int,
    pub field_10: c_int,
    pub field_14: c_int,
    pub field_18: c_int,
    pub field_1C: c_int,
    size: c_int,
    typ: c_char,
    filename: [c_char; 89],
    find_dataa: WIN32_FIND_DATAA,
}

impl ResultEntry {
    pub fn filename(&self) -> Result<&str, ::std::str::Utf8Error> {
        let cstring = unsafe { CStr::from_ptr(self.filename.as_ptr()) };
        cstring.to_str()
    }

    pub fn filename_as_ptr(&self) -> *const c_char {
        self.filename.as_ptr()
    }

    pub fn typ(&self) -> Entry {
        Entry::from(self.typ)
    }

    pub fn size(&self) -> c_int {
        self.size
    }

    pub fn file_time(&self) -> FILETIME {
        self.file_time
    }

    pub fn find_dataa(&self) -> WIN32_FIND_DATAA {
        self.find_dataa
    }
}
