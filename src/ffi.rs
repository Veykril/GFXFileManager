use winapi::c_int;

use file_manager::IFileManager;
use gfxinfo::GFXInfo;

#[link(name = "GFXFileManager")]
extern "stdcall" {
    pub(crate) fn GFXDllCreateObject(mode: c_int, object: *mut *mut IFileManager, version: c_int) -> c_int;
    pub(crate) fn GFXDllReleaseObject(object: *mut IFileManager) -> c_int;
    pub(crate) fn GFXFMInfo(info: *mut GFXInfo, index: c_int) -> c_int;
}
