use winapi::ctypes::{c_char, c_int, c_uchar};
use winapi::shared::ntdef::HANDLE;

use crate::file_manager::GFXFileManager;

pub struct SearchResult<'a> {
    inner: GFXSearchResult,
    file_manager: &'a GFXFileManager,
}

impl SearchResult<'_> {
    pub(crate) fn inner_mut(&mut self) -> &mut GFXSearchResult {
        &mut self.inner
    }

    pub fn success(&self) -> bool {
        self.inner.success != 0
    }

    pub fn h_find(&self) -> HANDLE {
        self.inner.hFind
    }
}

impl Drop for SearchResult<'_> {
    fn drop(&mut self) {
        self.file_manager.find_close(self.inner_mut());
    }
}

#[repr(C)]
#[allow(non_snake_case)]
pub(crate) struct GFXSearchResult {
    success: c_uchar,
    field_4: c_int,
    field_8: c_int,
    field_C: c_int,
    field_10: c_int,
    field_14: c_int,
    field_18: c_int,
    field_1C: c_int,
    field_20: c_int,
    field_24: c_int,
    field_28: c_int,
    field_2C: c_int,
    field_30: c_int,
    field_34: c_int,
    field_38: c_int,
    field_3C: c_int,
    gap40: [c_char; 840],
    hFind: HANDLE,
    field_38C: c_int,
}
