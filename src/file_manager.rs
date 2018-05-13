use std::ffi::{CString, CStr};
use std::ptr::null_mut;
use std::string::FromUtf8Error;
use std::convert::TryFrom;

use winapi::{c_char, c_int, c_long, c_ulong, c_void};
use winapi::{DWORD, HMODULE, HWND, LPDWORD, LPFILETIME};

use ffi::{GFXDllCreateObject, GFXDllReleaseObject};

use cjarchivefm::CJArchiveFm;
use dialog::DialogData;
use gfxfile::File;
use result_entry::ResultEntry;
use search_result::{SearchResult, GFXSearchResult};

const OBJECT_VERSION: c_int = 0x1007;

macro_rules! cstring {
    ($str: expr) => { CString::new($str).unwrap() };
}

macro_rules! vtable_call {
    ($_self:ident, $name:ident$(, $arg:expr)*) => {
        unsafe { ((*(*$_self._file_manager).vtable).$name)($_self._file_manager, $($arg),*) }
    };
}

pub type ForEachCallback = extern "cdecl" fn(CallbackState, ResultEntry, *mut c_void) -> ();
pub type ErrorHandler = extern "cdecl" fn(HWND, *const c_char, *const c_char) -> c_int;

extern "cdecl" fn err_dummy(_: HWND, _: *const c_char, _: *const c_char) -> c_int { 1 }

#[repr(i32)]
pub enum CallbackState {
    Init = 0,
    EnterDir = 1,
    LeaveDir = 2,
    File = 3
}

#[repr(C)]
pub struct UnknownPair(c_int, c_int);

#[allow(overflowing_literals)]
pub enum Access {
    OpenExisting = 0,
    ShareRead = 0x8000_0000,
    CreateAlways = 0x4000_0000,
}

impl TryFrom<u32> for Access {
    type Error = ();
    fn try_from(mode: u32) -> Result<Self, Self::Error> {
        match mode {
            0 => Ok(Access::OpenExisting),
            0x8000_0000 => Ok(Access::ShareRead),
            0x4000_0000 => Ok(Access::CreateAlways),
            _ => Err(())
        }
    }
}

#[derive(Debug)]
pub enum Mode {
    CP = 1,
    CW = 2
}

impl TryFrom<i32> for Mode {
    type Error = ();
    fn try_from(mode: i32) -> Result<Self, Self::Error> {
        match mode {
            1 => Ok(Mode::CP),
            2 => Ok(Mode::CW),
            _ => Err(())
        }
    }
}

pub struct GFXFileManager {
    _file_manager: *mut IFileManager
}

impl GFXFileManager {
    pub fn new(mode: Mode) -> Self {
        Self::new_with_version(mode, OBJECT_VERSION)
    }

    pub fn new_with_version(mode: Mode, version: c_int) -> Self {
        Self {
            _file_manager: IFileManager::new_ptr(mode as i32, version)
        }
    }

    pub fn disable_err_msg_box(&self) {
        self.register_error_handler(err_dummy);
    }

    /// Returns the container-mode.
    pub fn mode(&self) -> Mode {
        Mode::try_from(
            vtable_call!(self, mode)
        ).unwrap()
    }

    /// Sets some configuration
    pub fn config_set(&self, i1: i32, i2: i32) -> i32 {
        vtable_call!(self, config_set, i1, i2)
    }

    /// Gets some configuration, most likely just crashes the application atm
    pub fn config_get(&self, i1: i32, i2: i32) -> i32 {
        vtable_call!(self, config_get, i1, i2)
    }

    /// Creates a new container and opens it
    ///
    /// # Arguments
    ///
    /// * filename - Filename of the container
    /// * password - Password for accessing the new container
    pub fn create_container(&self, filename: &str, password: &str) -> bool {
        let filename = cstring!(filename);
        let password = cstring!(password);
        vtable_call!(self, create_container, filename.as_ptr(), password.as_ptr()) != 0
    }

    /// Opens an existing container
    ///
    /// # Arguments
    ///
    /// * filename - Filename of the container
    /// * password - Password required for accessing the container
    /// * mode - unknown, maybe for read and write access
    pub fn open_container(&self, filename: &str, password: &str, mode: i32) -> bool {
        let filename = cstring!(filename);
        let password = cstring!(password);
        vtable_call!(self, open_container, filename.as_ptr(), password.as_ptr(), mode) != 0
    }

    /// Closes the current container
    pub fn close_container(&self) -> bool {
        vtable_call!(self, close_container) != 0
    }

    /// Returns true if a container is currently open
    pub fn is_open(&self) -> bool {
        vtable_call!(self, is_open) != 0
    }

    /// This function shouldn't be called since all file handles will be closed when they are
    /// dropped given the implementation of the filehandle wrapper
    fn close_all_files(&self) -> i32 {
        vtable_call!(self, close_all_files)
    }

    /// Returns the MainModule-handle
    pub fn main_module_handle(&self) -> HMODULE {
        vtable_call!(self, main_module_handle)
    }

    pub fn function_9(&self, i1: i32) -> i32 {
        vtable_call!(self, function_9, i1)
    }

    /// Opens a file inside the container using a path and returns a File object
    ///
    /// # Arguments
    ///
    /// * filename - Filename, relative to current dir or absolute path inside archive
    /// * unknown - Not used for original CPFileManager
    pub fn open_file(&self, filename: &str, access: Access, unknown: i32) -> ::std::io::Result<File> {
        let filename = cstring!(filename);
        let res = vtable_call!(self, open_file, filename.as_ptr(), access as i32, unknown);
        if res == -1 {
            Err(::std::io::Error::new(::std::io::ErrorKind::NotFound, ""))
        } else {
            Ok(File::new(self, res))
        }
    }

    /// Opens a file inside the container using the CJArchiveFm-class and returns a File object
    ///
    /// # Arguments
    ///
    /// * fm - A mutable reference to a CJArchiveFm
    /// * filename - Filename, relative to current dir or absolute path inside archive
    /// * unknown - not used for original CPFileManager
    pub fn open_file_cj(&self, fm: &mut CJArchiveFm, filename: &str, access: Access, unknown: i32) -> File {
        let filename = cstring!(filename);
        File::new(self, vtable_call!(self, open_file_cj, fm, filename.as_ptr(), access as i32, unknown))
    }

    pub fn function_12(&self) -> i32 {
        vtable_call!(self, function_12)
    }

    pub fn function_13(&self) -> i32 {
        vtable_call!(self, function_13)
    }

    /// Creates a file inside the container and returns a File object
    ///
    /// # Arguments
    ///
    /// * filename - Filename, relative to current dir or absolute path inside archive
    /// * unknown
    pub fn create_file(&self, filename: &str, unknown: i32) -> File {
        let filename = cstring!(filename);
        File::new(self, vtable_call!(self, create_file, filename.as_ptr(), unknown))
    }


    /// Creates a file inside the container using the CJArchiveFm-class and returns a File object
    ///
    /// # Arguments
    ///
    /// * fm - A mutable reference to a CJArchiveFm
    /// * filename - Filename, relative to current dir or absolute path inside archive
    /// * unknown
    pub fn create_file_cj(&self, fm: &mut CJArchiveFm, filename: &str, unknown: i32) -> File {
        let filename = cstring!(filename);
        File::new(self, vtable_call!(self, create_file_cj, fm, filename.as_ptr(), unknown))
    }

    /// Deletes a file by name
    pub fn delete_file(&self, filename: &str) -> i32 {
        let filename = cstring!(filename);
        vtable_call!(self, delete_file, filename.as_ptr())
    }

    /// Closes file by handle, not public because our handle wrapper manages its lifetime itself
    pub(crate) fn close_file(&self, file: &File) -> i32 {
        vtable_call!(self, close_file, file.handle())
    }

    /// Reads a number of bytes from a file
    pub(crate) fn read(&self, file: &File, lp_buffer: &mut [u8], bytes_to_read: i32, bytes_read: *mut u32) -> i32 {
        vtable_call!(self, read, file.handle(), lp_buffer.as_mut_ptr() as *mut i8, bytes_to_read, bytes_read)
    }

    /// Writes a number of bytes to file
    pub(crate) fn write(&self, file: &File, lp_buffer: &[u8], bytes_to_write: i32, bytes_written: *mut u32) -> i32 {
        vtable_call!(self, write, file.handle(), lp_buffer.as_ptr() as *const i8, bytes_to_write, bytes_written)
    }

    pub fn cmd_line_path<'a>(&self) -> &'a CStr {
        let charptr: *const c_char = vtable_call!(self, cmd_line_path);
        unsafe { CStr::from_ptr(charptr) }
    }

    pub fn cmd_line_exe<'a>(&self) -> &'a CStr {
        let charptr: *const c_char = vtable_call!(self, cmd_line_exe);
        unsafe { CStr::from_ptr(charptr) }
    }

    pub fn get_unknown(&self, unknown: *mut UnknownPair) -> *mut UnknownPair {
        vtable_call!(self, get_unknown, unknown)
    }

    pub fn set_unknown(&self, a: i32, b: i32) -> i32 {
        vtable_call!(self, set_unknown, a, b)
    }

    /// Creates directory in the current pk2
    pub fn create_directory(&self, name: &str) -> bool {
        let name = cstring!(name);
        vtable_call!(self, create_dir, name.as_ptr()) != 0
    }

    /// Deletes directory in the current pk2
    pub fn delete_directory(&self, name: &str) -> bool {
        let name = cstring!(name);
        vtable_call!(self, delete_dir, name.as_ptr()) != 0
    }

    /// Resets the current working directory in the current pk2
    pub fn reset_directory(&self) -> bool {
        vtable_call!(self, reset_dir) != 0
    }

    /// Changes the current working directory
    pub fn change_directory(&self, name: &str) -> bool {
        let name = cstring!(name);
        vtable_call!(self, change_dir, name.as_ptr()) != 0
    }

    /// Returns the current directory's name or an utf8 error
    pub fn get_directory_name(&self) -> Result<String, FromUtf8Error> {
        let mut buf = Vec::with_capacity(255);
        unsafe { buf.set_len(255) };
        let ptr = buf.as_mut_ptr();
        let len = vtable_call!(self, get_dir_name, 200, ptr as *mut i8);
        buf.truncate(len as usize);
        String::from_utf8(buf)
    }

    pub fn set_virtual_path(&self, path: &str) -> bool {
        let path = cstring!(path);
        vtable_call!(self, set_virtual_path, path.as_ptr()) != 0
    }

    pub fn get_virtual_path(&self) -> Result<String, FromUtf8Error> {
        let mut buf = Vec::with_capacity(255);
        unsafe { buf.set_len(255) };
        vtable_call!(self, get_virtual_path, buf.as_mut_ptr() as *mut i8);
        if let Some(null_pos) = buf.iter().position(|&x| x == 0) {
            buf.truncate(null_pos);
        }
        String::from_utf8(buf)
    }

    pub fn find_first_file(&self, search: &mut SearchResult, pattern: &str, entry: &mut ResultEntry) {
        let pattern = cstring!(pattern);
        vtable_call!(self, find_first_file, search.inner_mut(), pattern.as_ptr(), entry);
    }

    pub fn find_next_file(&self, search: &mut SearchResult, entry: &mut ResultEntry) -> i32 {
        vtable_call!(self, find_next_file, search.inner_mut(), entry)
    }

    pub(crate) fn find_close(&self, search: &mut GFXSearchResult) -> i32 {
        vtable_call!(self, close_search_result, search)
    }

    pub(crate) fn file_name_from_handle(&self, file: &File) -> Result<String, FromUtf8Error> {
        let mut buf = Vec::with_capacity(512);
        unsafe { buf.set_len(512) };
        vtable_call!(self, file_name_from_handle, file.handle(), buf.as_mut_ptr() as *mut i8, buf.len());
        if let Some(null_pos) = buf.iter().position(|&x| x == 0) {
            buf.truncate(null_pos);
        }
        String::from_utf8(buf)
    }

    pub(crate) fn get_file_size(&self, file: &File) -> i32 {
        vtable_call!(self, get_file_size, file.handle(), null_mut())
    }

    pub(crate) fn get_file_time(&self, file: &File, creation_time: LPFILETIME, last_write_time: LPFILETIME) -> bool {
        vtable_call!(self, get_file_time, file.handle(), creation_time, last_write_time)
    }

    pub(crate) fn set_file_time(&self, file: &File, creation_time: LPFILETIME, last_write_time: LPFILETIME) -> bool {
        vtable_call!(self, set_file_time, file.handle(), creation_time, last_write_time)
    }

    pub(crate) fn seek(&self, file: &File, distance_to_move: c_long, move_method: DWORD) -> i32{
        vtable_call!(self, seek, file.handle(), distance_to_move, move_method)
    }

    pub fn get_hwnd(&self) -> HWND {
        vtable_call!(self, get_hwnd)
    }

    pub fn set_hwnd(&self, hwnd: HWND) -> i32 {
        vtable_call!(self, set_hwnd, hwnd)
    }

    pub fn register_error_handler(&self, callback: ErrorHandler) -> i32 {
        vtable_call!(self, register_error_handler, callback)
    }

    pub fn import_directory(&self, srcdir: &str, dstdir: &str, dir_name: &str, create_target_dir: bool) -> i32 {
        let srcdir = cstring!(srcdir);
        let dstdir = cstring!(dstdir);
        let dir_name = cstring!(dir_name);
        vtable_call!(self, import_dir, srcdir.as_ptr(), dstdir.as_ptr(), dir_name.as_ptr(), create_target_dir)
    }

    pub fn import_file(&self, srcdir: &str, dstdir: &str, filename: &str, create_target_dir: bool) -> i32 {
        let srcdir = cstring!(srcdir);
        let dstdir = cstring!(dstdir);
        let filename = cstring!(filename);
        vtable_call!(self, import_file, srcdir.as_ptr(), dstdir.as_ptr(), filename.as_ptr(), create_target_dir)
    }

    pub fn export_directory(&self, srcdir: &str, dstdir: &str, dir_name: &str, create_target_dir: bool) -> i32 {
        let srcdir = cstring!(srcdir);
        let dstdir = cstring!(dstdir);
        let dir_name = cstring!(dir_name);
        vtable_call!(self, export_dir, srcdir.as_ptr(), dstdir.as_ptr(), dir_name.as_ptr(), create_target_dir)
    }

    pub fn export_file(&self, srcdir: &str, dstdir: &str, filename: &str, create_target_dir: bool) -> i32 {
        let srcdir = cstring!(srcdir);
        let dstdir = cstring!(dstdir);
        let filename = cstring!(filename);
        vtable_call!(self, export_file, srcdir.as_ptr(), dstdir.as_ptr(), filename.as_ptr(), create_target_dir)
    }

    pub fn file_exists(&self, name: &str, flags: i32) -> i32 {
        let name = cstring!(name);
        vtable_call!(self, file_exists, name.as_ptr(), flags)
    }

    pub fn show_dialog(&self, data: &mut DialogData) -> i32 {
        vtable_call!(self, show_dialog, data)
    }

    pub fn for_each_entry_in_container(&self, callback: ForEachCallback, filter: &str, userstate: *mut c_void) -> i32 {
        let filter = cstring!(filter);
        vtable_call!(self, for_each_entry_in_container, callback, filter.as_ptr(), userstate)
    }

    pub fn update_current_directory(&self) -> i32 {
        vtable_call!(self, update_current_dir)
    }

    pub fn function_50(&self, i1: i32) -> i32 {
        vtable_call!(self, function_50, i1)
    }

    pub fn get_version(&self) -> i32 {
        vtable_call!(self, get_version)
    }

    pub fn check_version(&self, version: i32) -> i32 {
        vtable_call!(self, check_version, version)
    }

    pub fn unlock(&self) -> bool {
        vtable_call!(self, unlock) != 0
    }

    pub fn lock(&self, i1: i32) -> bool {
        vtable_call!(self, lock, i1) != 0
    }
}

impl Drop for GFXFileManager {
    fn drop(&mut self) {
        self.close_all_files();
        if self.is_open() {
            self.close_container();
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
    find_first_file: extern "thiscall" fn(*mut IFileManager, *mut GFXSearchResult, *const c_char, *mut ResultEntry) -> *mut GFXSearchResult,
    find_next_file: extern "thiscall" fn(*mut IFileManager, *mut GFXSearchResult, *mut ResultEntry) -> c_int,
    close_search_result: extern "thiscall" fn(*mut IFileManager, *mut GFXSearchResult) -> c_int,
    file_name_from_handle: extern "thiscall" fn(*mut IFileManager, c_int, *mut c_char, usize) -> c_int,
    get_file_size: extern "thiscall" fn(*mut IFileManager, c_int, LPDWORD) -> c_int,
    get_file_time: extern "thiscall" fn(*mut IFileManager, c_int, LPFILETIME, LPFILETIME) -> bool,
    set_file_time: extern "thiscall" fn(*mut IFileManager, c_int, LPFILETIME, LPFILETIME) -> bool,
    seek: extern "thiscall" fn(*mut IFileManager, c_int, c_long, DWORD) -> c_int,
    get_hwnd: extern "thiscall" fn(*mut IFileManager) -> HWND,
    set_hwnd: extern "thiscall" fn(*mut IFileManager, HWND) -> c_int,
    register_error_handler: extern "thiscall" fn(*mut IFileManager, ErrorHandler) -> c_int,
    import_dir: extern "thiscall" fn(*mut IFileManager, *const c_char, *const c_char, *const c_char, bool) -> c_int,
    import_file: extern "thiscall" fn(*mut IFileManager, *const c_char, *const c_char, *const c_char, bool) -> c_int,
    export_dir: extern "thiscall" fn(*mut IFileManager, *const c_char, *const c_char, *const c_char, bool) -> c_int,
    export_file: extern "thiscall" fn(*mut IFileManager, *const c_char, *const c_char, *const c_char, bool) -> c_int,
    file_exists: extern "thiscall" fn(*mut IFileManager, *const c_char, c_int) -> c_int,
    show_dialog: extern "thiscall" fn(*mut IFileManager, *mut DialogData) -> c_int,
    for_each_entry_in_container: extern "thiscall" fn(*mut IFileManager, ForEachCallback, *const c_char, *mut c_void) -> c_int,
    update_current_dir: extern "thiscall" fn(*mut IFileManager) -> c_int,
    function_50: extern "thiscall" fn(*mut IFileManager, c_int) -> c_int,
    get_version: extern "thiscall" fn(*mut IFileManager) -> c_int,
    check_version: extern "thiscall" fn(*mut IFileManager, c_int) -> c_int,
    lock: extern "thiscall" fn(*mut IFileManager, c_int) -> c_int,
    unlock: extern "thiscall" fn(*mut IFileManager) -> c_int
}

#[repr(C)]
pub(crate) struct IFileManager {
    vtable: *const VTable,
}

impl IFileManager {
    fn new_ptr(mode: c_int, version: c_int) -> *mut IFileManager {
        let mut obj = null_mut();
        unsafe { GFXDllCreateObject(mode, &mut obj, version) };
        obj
    }
}

impl Drop for IFileManager {
    fn drop(&mut self) {
        unsafe { GFXDllReleaseObject(self) };
    }
}
