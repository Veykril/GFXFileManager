use winapi::ctypes::c_int;

use crate::file_manager::IFileManager;
use crate::gfxinfo::GFXInfo;

#[link(name = "GFXFileManager", kind = "raw-dylib", import_name_type = "undecorated")]
extern "stdcall" {
    pub(crate) fn GFXDllCreateObject(
        mode: c_int,
        object: *mut *mut IFileManager,
        version: c_int,
    ) -> c_int;
    pub(crate) fn GFXDllReleaseObject(object: *mut IFileManager) -> c_int;
    pub(crate) fn GFXFMInfo(info: *mut GFXInfo, index: c_int) -> c_int;
}
