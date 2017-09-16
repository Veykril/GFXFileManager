#![cfg(all(target_os = "windows", target_arch = "x86"))]
#![feature(abi_thiscall)]
extern crate winapi;

use std::ffi::CString;

use winapi::{c_char, c_int, c_ulong};
use winapi::{LPDWORD, LPFILETIME, HMODULE, LONG, DWORD, HWND};

pub mod structures;
pub mod gfxinfo;
pub mod cjarchivefm;
pub mod search_result;
pub mod result_entry;

use structures::{UnknownPair, ErrorHandler, DialogData, ForEachCallback};
use search_result::SearchResult;
use result_entry::ResultEntry;
use gfxinfo::GFXInfo;
use cjarchivefm::CJArchiveFm;

static ERROR_CSTRING_CREATE: &'static str = "Couldn't create CString";

#[allow(improper_ctypes)]//the compiler complains about this but I dont quite understand why since IFileManager is being represented as a C type so it probably has a problem with the double pointer
#[link(name = "GFXFileManager")]
extern "stdcall" {
    fn GFXDllCreateObject(mode: c_int, object: *mut *mut IFileManager, version: c_int) -> c_int;
    fn GFXDllReleaseObject(object: *mut IFileManager) -> c_int;
    fn GFXFMInfo(info: *mut GFXInfo, index: c_int) -> c_int;
}

pub fn gfxfm_info(info: *mut GFXInfo, index: i32) -> c_int {
    unsafe {
        GFXFMInfo(info, index)
    }
}

pub enum Mode {
    CP = 1,
    CW = 2
}

impl From<i32> for Mode {
    fn from(mode: i32) -> Self {
        match mode {
            1 => Mode::CP,
            2 => Mode::CW,
            _ => panic!("Unable to match FileManager mode: {}!", mode),
        }
    }
}


pub struct GFXFileManager {
    _file_manager: *mut IFileManager
}

impl GFXFileManager {
    pub fn new(mode: Mode) -> Self {
        Self {_file_manager: IFileManager::new_ptr(mode as i32)}
    }

    /// Returns the container-mode.
    pub fn mode(&self) -> Mode {
        Mode::from(
            unsafe { ((*(*self._file_manager).vtable).mode)(self._file_manager) }
        )
    }

    /// Sets some configuration
    pub fn config_set(&self, i1: i32, i2: i32) -> i32 {
        unsafe { ((*(*self._file_manager).vtable).config_set)(self._file_manager, i1, i2) }
    }

    /// Gets some configuration
    pub fn config_get(&self, i1: i32, i2: i32) -> i32 {
        unsafe { ((*(*self._file_manager).vtable).config_get)(self._file_manager, i1, i2) }
    }

    /// Create a new container
    ///
    /// # Arguments
    ///
    /// * `filename` - filename of the container
    /// * `password` - password for accessing the new container
    pub fn create_container(&self, filename: &str, password: &str) -> i32 {
        let filename = CString::new(filename).expect(ERROR_CSTRING_CREATE);
        let password = CString::new(password).expect(ERROR_CSTRING_CREATE);
        unsafe { ((*(*self._file_manager).vtable).create_container)(self._file_manager, filename.as_ptr(), password.as_ptr()) }
    }


    /// Open an existing container
    ///
    /// # Arguments
    ///
    /// * `filename` - filename of the container
    /// * `password` - password required for accessing the container
    /// * `mode` - unknown, maybe for read and write access
    pub fn open_container(&self, filename: &str, password: &str, mode: i32) -> i32 {
        let filename = CString::new(filename).expect(ERROR_CSTRING_CREATE);
        let password = CString::new(password).expect(ERROR_CSTRING_CREATE);
        unsafe { ((*(*self._file_manager).vtable).open_container)(self._file_manager, filename.as_ptr(), password.as_ptr(), mode) }
    }

    /// Close the current container
    pub fn close_container(&self) -> i32 {
        unsafe { ((*(*self._file_manager).vtable).close_container)(self._file_manager) }
    }

    /// Returns true if a container is currently open
    pub fn is_open(&self) -> bool {
        unsafe { ((*(*self._file_manager).vtable).is_open)(self._file_manager) != 0 }
    }

    pub fn close_all_files(&self) -> i32 {
        unsafe { ((*(*self._file_manager).vtable).close_all_files)(self._file_manager) }
    }

    /// Returns the MainModule-handle
    pub fn main_module_handle(&self) -> HMODULE {
        unsafe { ((*(*self._file_manager).vtable).main_module_handle)(self._file_manager) }
    }

    pub fn function_9(&self, i1: i32) -> i32 {
        unsafe { ((*(*self._file_manager).vtable).function_9)(self._file_manager, i1) }
    }

    /// Open a file inside the container using a path and returns the file handle or -1
    ///
    /// # Arguments
    ///
    /// * `filename` - filename, relative to current dir or absolute path inside archive
    /// * `access` - 0 for open-existing, 0x80000000 for open and share_read, 0x40000000 for create_always
    /// * `unknown` - not used for original CPFileManager
    pub fn open_file(&self, filename: &str, access: i32, unknown: i32) -> i32 {
        let filename = CString::new(filename).expect(ERROR_CSTRING_CREATE);
        unsafe { ((*(*self._file_manager).vtable).open_file)(self._file_manager, filename.as_ptr(), access, unknown) }
    }

    /// Open a file inside the container using the CJArchiveFm-class and returns the file handle or -1
    ///
    /// # Arguments
    ///
    /// * `fm` - A valid pointer to the CJArchiveFm-class
    /// * `filename` - filename, relative to current dir or absolute path inside archive
    /// * `access` - 0 for open-existing, 0x80000000 for open and share_read, 0x40000000 for create_always
    /// * `unknown` - not used for original CPFileManager
    pub fn open_file_cj(&self, fm: &mut CJArchiveFm, filename: &str, access: i32, unknown: i32) -> i32 {
        let filename = CString::new(filename).expect(ERROR_CSTRING_CREATE);
        unsafe { ((*(*self._file_manager).vtable).open_file_cj)(self._file_manager, fm, filename.as_ptr(), access, unknown) }
    }

    pub fn function_12(&self) -> i32 {
        unsafe { ((*(*self._file_manager).vtable).function_12)(self._file_manager) }
    }

    pub fn function_13(&self) -> i32 {
        unsafe { ((*(*self._file_manager).vtable).function_13)(self._file_manager) }
    }

    pub fn create_file(&self, filename: &str, unknown: i32) -> i32 {
        let filename = CString::new(filename).expect(ERROR_CSTRING_CREATE);
        unsafe { ((*(*self._file_manager).vtable).create_file)(self._file_manager, filename.as_ptr(), unknown) }
    }

    pub fn create_file_cj(&self, fm: *mut CJArchiveFm, filename: &str, unknown: i32) -> i32 {
        let filename = CString::new(filename).expect(ERROR_CSTRING_CREATE);
        unsafe { ((*(*self._file_manager).vtable).create_file_cj)(self._file_manager, fm, filename.as_ptr(), unknown) }
    }

    /// Delete a file by name
    pub fn delete_file(&self, filename: &str) -> i32 {
        let filename = CString::new(filename).expect(ERROR_CSTRING_CREATE);
        unsafe { ((*(*self._file_manager).vtable).delete_file)(self._file_manager, filename.as_ptr()) }
    }

    /// Close file by handle
    pub fn close_file(&self, h_file: i32) -> i32 {
        unsafe { ((*(*self._file_manager).vtable).close_file)(self._file_manager, h_file) }
    }

    /// Read a number of bytes from file
    /// # Arguments
    /// Parameter:
    /// * `h_file` - Any handle or pointer identifiying this file
    /// * `lp_buffer` - pointer to reserved memory for read operation
    /// * `bytes_to_read` - size of lp_buffer
    /// * `bytes_read` - pointer to memory, will contain the number of bytes read from the file
    pub fn read(&self, h_file: i32, lp_buffer: *mut c_char, bytes_to_read: i32, bytes_read: *mut u32) -> i32 {
        unsafe { ((*(*self._file_manager).vtable).read)(self._file_manager, h_file, lp_buffer, bytes_to_read, bytes_read) }
    }

    /// Write a number of bytes to file
    /// # Arguments
    /// Parameter:
    /// * `h_file` - Any handle or pointer identifiying this file
    /// * `lp_buffer` - pointer to reserved memory for write operation
    /// * `bytes_to_write` - size of lp_buffer
    /// * `bytes_written` - pointer to memory, will contain the number of bytes written the file
    pub fn write(&self, h_file: i32, lp_buffer: *const c_char, bytes_to_write: i32, bytes_written: *mut u32) -> i32 {
        unsafe { ((*(*self._file_manager).vtable).write)(self._file_manager, h_file, lp_buffer, bytes_to_write, bytes_written) }
    }

    pub fn cmd_line_path(&self) -> CString {
        unsafe {
            let charptr: *mut c_char = ((*(*self._file_manager).vtable).cmd_line_path)(self._file_manager);
            CString::from_raw(charptr)
        }
    }

    pub fn cmd_line_exe(&self) -> CString {
        unsafe {
            let charptr: *mut c_char = ((*(*self._file_manager).vtable).cmd_line_exe)(self._file_manager);
            CString::from_raw(charptr)
        }
    }

    pub fn get_unknown(&self, unknown: *mut UnknownPair) -> *mut UnknownPair {
        unsafe {
            ((*(*self._file_manager).vtable).get_unknown)(self._file_manager, unknown)
        }
    }

    pub fn set_unknown(&self, a: i32, b: i32) -> i32 {
        unsafe {
            ((*(*self._file_manager).vtable).set_unknown)(self._file_manager, a, b)
        }
    }

    pub fn create_directory(&self, name: &str) -> i32 {
        let name = CString::new(name).expect(ERROR_CSTRING_CREATE);
        unsafe {
            ((*(*self._file_manager).vtable).create_dir)(self._file_manager, name.as_ptr())
        }
    }

    pub fn delete_directory(&self, name: &str) -> i32 {
        let name = CString::new(name).expect(ERROR_CSTRING_CREATE);
        unsafe {
            ((*(*self._file_manager).vtable).delete_dir)(self._file_manager, name.as_ptr())
        }
    }

    pub fn reset_directory(&self) -> i32 {
        unsafe {
            ((*(*self._file_manager).vtable).reset_dir)(self._file_manager)
        }
    }

    pub fn change_directory(&self, name: &str) -> i32 {
        let name = CString::new(name).expect(ERROR_CSTRING_CREATE);
        unsafe {
            ((*(*self._file_manager).vtable).change_dir)(self._file_manager, name.as_ptr())
        }
    }

    pub fn get_directory_name(&self) -> Result<String, std::string::FromUtf8Error> {
        let mut buf = vec![0u8;200];
        let ptr = buf.as_mut_ptr();
        unsafe {
            let len = ((*(*self._file_manager).vtable).get_dir_name)(self._file_manager, 200, ptr as *mut i8);
            //let string = CStr::from_ptr(ptr).to_string();
            buf.truncate(len as usize);
            String::from_utf8(buf)
        }
    }

    pub fn set_virtual_path(&self, path: *const c_char) -> i32 {
        unsafe {
            ((*(*self._file_manager).vtable).set_virtual_path)(self._file_manager, path)
        }
    }

    pub fn get_virtual_path(&self, dest: *mut c_char) -> i32 {
        unsafe {
            ((*(*self._file_manager).vtable).get_virtual_path)(self._file_manager, dest)
        }
    }

    pub fn find_first_file(&self, search: *mut SearchResult, pattern: *const c_char, entry: *mut ResultEntry) -> *mut SearchResult {
        unsafe {
            ((*(*self._file_manager).vtable).find_first_file)(self._file_manager, search, pattern, entry)
        }
    }

    pub fn find_next_file(&self, search: *mut SearchResult, entry: *mut ResultEntry) -> i32 {
        unsafe {
            ((*(*self._file_manager).vtable).find_next_file)(self._file_manager, search, entry)
        }
    }

    pub fn find_close(&self, search: *mut SearchResult) -> i32 {
        unsafe {
            ((*(*self._file_manager).vtable).close_search_result)(self._file_manager, search)
        }
    }

    pub fn file_name_from_handle(&self, h_file: i32, dest: *mut c_char, count: usize) -> i32 {
        unsafe {
            ((*(*self._file_manager).vtable).file_name_from_handle)(self._file_manager, h_file, dest, count)
        }
    }

    pub fn get_file_size(&self, h_file: i32, file_size: LPDWORD) -> i32 {
        unsafe {
            ((*(*self._file_manager).vtable).get_file_size)(self._file_manager, h_file, file_size)
        }
    }

    pub fn get_file_time(&self, h_file: i32, creation_time: LPFILETIME, last_write_time: LPFILETIME) -> bool {
        unsafe {
            ((*(*self._file_manager).vtable).get_file_time)(self._file_manager, h_file, creation_time, last_write_time)
        }
    }

    pub fn set_file_time(&self, h_file: i32, creation_time: LPFILETIME, last_write_time: LPFILETIME) -> bool {
        unsafe {
            ((*(*self._file_manager).vtable).set_file_time)(self._file_manager, h_file, creation_time, last_write_time)
        }
    }

    pub fn seek(&self, h_file: i32, distance_to_move: LONG, move_method: DWORD) -> i32{
        unsafe {
            ((*(*self._file_manager).vtable).seek)(self._file_manager, h_file, distance_to_move, move_method)
        }
    }

    pub fn get_hwnd(&self) -> HWND {
        unsafe {
            ((*(*self._file_manager).vtable).get_hwnd)(self._file_manager)
        }
    }

    pub fn set_hwnd(&self, hwnd: HWND) -> i32 {
        unsafe {
            ((*(*self._file_manager).vtable).set_hwnd)(self._file_manager, hwnd)
        }
    }

    pub fn register_error_handler(&self, callback: ErrorHandler) -> i32 {
        unsafe {
            ((*(*self._file_manager).vtable).register_error_handler)(self._file_manager, callback)
        }
    }

    pub fn import_directory(&self, srcdir: &str, dstdir: &str, dir_name: &str, create_target_dir: bool) -> i32 {
        let srcdir = CString::new(srcdir).expect(ERROR_CSTRING_CREATE);
        let dstdir = CString::new(dstdir).expect(ERROR_CSTRING_CREATE);
        let dir_name = CString::new(dir_name).expect(ERROR_CSTRING_CREATE);
        unsafe {
            ((*(*self._file_manager).vtable).import_dir)(self._file_manager, srcdir.as_ptr(), dstdir.as_ptr(), dir_name.as_ptr(), create_target_dir)
        }
    }

    pub fn import_file(&self, srcdir: &str, dstdir: &str, filename: &str, create_target_dir: bool) -> i32 {
        let srcdir = CString::new(srcdir).expect(ERROR_CSTRING_CREATE);
        let dstdir = CString::new(dstdir).expect(ERROR_CSTRING_CREATE);
        let filename = CString::new(filename).expect(ERROR_CSTRING_CREATE);
        unsafe {
            ((*(*self._file_manager).vtable).import_file)(self._file_manager, srcdir.as_ptr(), dstdir.as_ptr(), filename.as_ptr(), create_target_dir)
        }
    }

    pub fn export_directory(&self, srcdir: &str, dstdir: &str, dir_name: &str, create_target_dir: bool) -> i32 {
        let srcdir = CString::new(srcdir).expect(ERROR_CSTRING_CREATE);
        let dstdir = CString::new(dstdir).expect(ERROR_CSTRING_CREATE);
        let dir_name = CString::new(dir_name).expect(ERROR_CSTRING_CREATE);
        unsafe {
            ((*(*self._file_manager).vtable).export_dir)(self._file_manager, srcdir.as_ptr(), dstdir.as_ptr(), dir_name.as_ptr(), create_target_dir)
        }
    }

    pub fn export_file(&self, srcdir: &str, dstdir: &str, filename: &str, create_target_dir: bool) -> i32 {
        let srcdir = CString::new(srcdir).expect(ERROR_CSTRING_CREATE);
        let dstdir = CString::new(dstdir).expect(ERROR_CSTRING_CREATE);
        let filename = CString::new(filename).expect(ERROR_CSTRING_CREATE);
        unsafe {
            ((*(*self._file_manager).vtable).export_file)(self._file_manager, srcdir.as_ptr(), dstdir.as_ptr(), filename.as_ptr(), create_target_dir)
        }
    }

    pub fn file_exists(&self, name: &str, flags: i32) -> i32 {
        let name = CString::new(name).unwrap();
        unsafe {
            ((*(*self._file_manager).vtable).file_exists)(self._file_manager, name.as_ptr(), flags)
        }
    }

    pub fn show_dialog(&self, data: &mut DialogData) -> i32 {
        unsafe {
            ((*(*self._file_manager).vtable).show_dialog)(self._file_manager, data)
        }
    }

    pub fn for_each_entry_in_container(&self, callback: ForEachCallback, filter: *const c_char, userstate: *mut ::std::os::raw::c_void) -> i32 {
        unsafe {
            ((*(*self._file_manager).vtable).for_each_entry_in_container)(self._file_manager, callback, filter, userstate)
        }
    }

    pub fn update_current_directory(&self) -> i32 {
        unsafe {
            ((*(*self._file_manager).vtable).update_current_dir)(self._file_manager)
        }
    }

    pub fn function_50(&self, i1: i32) -> i32 {
        unsafe {
            ((*(*self._file_manager).vtable).function_50)(self._file_manager, i1)
        }
    }

    pub fn get_version(&self) -> i32 {
        unsafe {
            ((*(*self._file_manager).vtable).get_version)(self._file_manager)
        }
    }

    pub fn check_version(&self, version: i32) -> i32 {
        unsafe {
            ((*(*self._file_manager).vtable).check_version)(self._file_manager, version)
        }
    }

    pub fn unlock(&self) -> i32 {
        unsafe {
            ((*(*self._file_manager).vtable).unlock)(self._file_manager)
        }
    }

    pub fn lock(&self, i1: i32) -> i32 {
        unsafe {
            ((*(*self._file_manager).vtable).lock)(self._file_manager, i1)
        }
    }
}


#[repr(C)]
struct VTable {
    mode: extern "thiscall" fn(*mut IFileManager) -> c_int,
    config_set: extern "thiscall" fn(*mut IFileManager, c_int, c_int) -> c_int,
    config_get: extern "thiscall" fn(*mut IFileManager, c_int, c_int) -> c_int,
    create_container: extern "thiscall" fn(*mut IFileManager, *const c_char, *const c_char) -> c_int,
    open_container: extern "thiscall" fn(*mut IFileManager, *const c_char, *const c_char, c_int) -> c_int,
    close_container: extern "thiscall" fn(*mut IFileManager) -> c_int,
    is_open: extern "thiscall" fn(*mut IFileManager) -> c_int,
    close_all_files: extern "thiscall" fn(*mut IFileManager) -> c_int,
    main_module_handle: extern "thiscall" fn(*mut IFileManager) -> HMODULE,
    function_9: extern "thiscall" fn(*mut IFileManager, c_int) -> c_int,
    open_file_cj: extern "thiscall" fn(*mut IFileManager, *mut CJArchiveFm, *const c_char, c_int, c_int) -> c_int,
    open_file: extern "thiscall" fn(*mut IFileManager, *const c_char, c_int, c_int) -> c_int,
    function_12: extern "thiscall" fn(*mut IFileManager) -> c_int,
    function_13: extern "thiscall" fn(*mut IFileManager) -> c_int,
    create_file_cj: extern "thiscall" fn(*mut IFileManager, *mut CJArchiveFm, *const c_char, c_int) -> c_int,
    create_file: extern "thiscall" fn(*mut IFileManager, *const c_char, c_int) -> c_int,
    delete_file: extern "thiscall" fn(*mut IFileManager, *const c_char,) -> c_int,
    close_file: extern "thiscall" fn(*mut IFileManager, c_int) -> c_int,
    read: extern "thiscall" fn(*mut IFileManager, c_int, *mut c_char, c_int, *mut c_ulong) -> c_int,
    write: extern "thiscall" fn(*mut IFileManager, c_int, *const c_char, c_int, *mut c_ulong) -> c_int,
    cmd_line_path: extern "thiscall" fn(*mut IFileManager) -> *mut c_char,
    cmd_line_exe: extern "thiscall" fn(*mut IFileManager) -> *mut c_char,
    get_unknown: extern "thiscall" fn(*mut IFileManager, *mut UnknownPair) -> *mut UnknownPair,
    set_unknown: extern "thiscall" fn(*mut IFileManager, c_int, c_int) -> c_int,
    create_dir: extern "thiscall" fn(*mut IFileManager, *const c_char) -> c_int,
    delete_dir: extern "thiscall" fn(*mut IFileManager, *const c_char) -> c_int,
    reset_dir: extern "thiscall" fn(*mut IFileManager) -> c_int,
    change_dir: extern "thiscall" fn(*mut IFileManager, *const c_char) -> c_int,
    get_dir_name: extern "thiscall" fn(*mut IFileManager, usize, *mut c_char) -> c_int,
    set_virtual_path: extern "thiscall" fn(*mut IFileManager, *const c_char) -> c_int,
    get_virtual_path: extern "thiscall" fn(*mut IFileManager, *mut c_char) -> c_int,
    find_first_file: extern "thiscall" fn(*mut IFileManager, *mut SearchResult, *const c_char, *mut ResultEntry) -> *mut SearchResult,
    find_next_file: extern "thiscall" fn(*mut IFileManager, *mut SearchResult, *mut ResultEntry) -> c_int,
    close_search_result: extern "thiscall" fn(*mut IFileManager, *mut SearchResult) -> c_int,
    file_name_from_handle: extern "thiscall" fn(*mut IFileManager, c_int, *mut c_char, usize) -> c_int,
    get_file_size: extern "thiscall" fn(*mut IFileManager, c_int, LPDWORD) -> c_int,
    get_file_time: extern "thiscall" fn(*mut IFileManager, c_int, LPFILETIME, LPFILETIME) -> bool,
    set_file_time: extern "thiscall" fn(*mut IFileManager, c_int, LPFILETIME, LPFILETIME) -> bool,
    seek: extern "thiscall" fn(*mut IFileManager, c_int, LONG, DWORD) -> c_int,
    get_hwnd: extern "thiscall" fn(*mut IFileManager) -> HWND,
    set_hwnd: extern "thiscall" fn(*mut IFileManager, HWND) -> c_int,
    register_error_handler: extern "thiscall" fn(*mut IFileManager, ErrorHandler) -> c_int,
    import_dir: extern "thiscall" fn(*mut IFileManager, *const c_char, *const c_char, *const c_char, bool) -> c_int,
    import_file: extern "thiscall" fn(*mut IFileManager, *const c_char, *const c_char, *const c_char, bool) -> c_int,
    export_dir: extern "thiscall" fn(*mut IFileManager, *const c_char, *const c_char, *const c_char, bool) -> c_int,
    export_file: extern "thiscall" fn(*mut IFileManager, *const c_char, *const c_char, *const c_char, bool) -> c_int,
    file_exists: extern "thiscall" fn(*mut IFileManager, *mut c_char, c_int) -> c_int,
    show_dialog: extern "thiscall" fn(*mut IFileManager, *mut DialogData) -> c_int,
    for_each_entry_in_container: extern "thiscall" fn(*mut IFileManager, ForEachCallback, *const c_char, *mut ::std::os::raw::c_void) -> c_int,
    update_current_dir: extern "thiscall" fn(*mut IFileManager) -> c_int,
    function_50: extern "thiscall" fn(*mut IFileManager, c_int) -> c_int,
    get_version: extern "thiscall" fn(*mut IFileManager) -> c_int,
    check_version: extern "thiscall" fn(*mut IFileManager, c_int) -> c_int,
    lock: extern "thiscall" fn(*mut IFileManager, c_int) -> c_int,
    unlock: extern "thiscall" fn(*mut IFileManager) -> c_int
}

#[repr(C)]
struct IFileManager {
    vtable: *const VTable,
}

impl IFileManager {
    fn new_ptr(mode: c_int) -> *mut IFileManager {
        let mut obj = ::std::ptr::null_mut();
        unsafe { GFXDllCreateObject(mode, &mut obj, 0x1007); }
        obj
    }
}

impl Drop for IFileManager {
    fn drop(&mut self) {
        unsafe {
            GFXDllReleaseObject(self);
        }
    }
}