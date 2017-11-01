use std::ffi::CStr;
use std::ffi::CString;
use std::ptr::null_mut;
use std::string::FromUtf8Error;

use winapi::{c_char, c_int, c_long, c_ulong};
use winapi::{DWORD, HMODULE, HWND, LPDWORD, LPFILETIME};

use ffi::GFXDllCreateObject;
use ffi::GFXDllReleaseObject;

use cjarchivefm::CJArchiveFm;
use dialog::DialogData;
use gfxfile::File;
use result_entry::ResultEntry;
use search_result::SearchResult;

const OBJECT_VERSION: c_int = 0x1007;
static ERROR_CSTRING_CREATE: &'static str = "Couldn't create CString";

macro_rules! cstring {
    ($str: expr) => { CString::new($str).expect(ERROR_CSTRING_CREATE) };
}

pub type ForEachCallback = extern "thiscall" fn(CallbackState, ResultEntry, *mut ::std::os::raw::c_void) -> bool;
pub type ErrorHandler = extern "thiscall" fn(HWND, *const c_char, *const c_char) -> bool;

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
    ShareRead = 0x80000000,
    CreateAlways = 0x40000000,
}

impl From<u32> for Access {
    fn from(mode: u32) -> Self {
        match mode {
            0 => Access::OpenExisting,
            0x80000000 => Access::ShareRead,
            0x40000000 => Access::CreateAlways,
            _ => panic!("Unable to match Access mode: {}!", mode),
        }
    }
}

#[derive(Debug)]
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
        Self {
            _file_manager: IFileManager::new_ptr(mode as i32, OBJECT_VERSION)
        }
    }

    pub fn new_with_version(mode: Mode, version: c_int) -> Self {
        Self {
            _file_manager: IFileManager::new_ptr(mode as i32, version)
        }
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

    /// Gets some configuration, most likely just crashes the application atm
    pub fn config_get(&self, i1: i32, i2: i32) -> i32 {
        unsafe { ((*(*self._file_manager).vtable).config_get)(self._file_manager, i1, i2) }
    }

    /// Create a new container
    ///
    /// # Arguments
    ///
    /// * `filename` - filename of the container
    /// * `password` - password for accessing the new container
    pub fn create_container(&self, filename: &str, password: &str) -> bool {
        let filename = cstring!(filename);
        let password = cstring!(password);
        unsafe { ((*(*self._file_manager).vtable).create_container)(self._file_manager, filename.as_ptr(), password.as_ptr()) != 0 }
    }


    /// Open an existing container
    ///
    /// # Arguments
    ///
    /// * `filename` - filename of the container
    /// * `password` - password required for accessing the container
    /// * `mode` - unknown, maybe for read and write access
    pub fn open_container(&self, filename: &str, password: &str, mode: i32) -> bool {
        let filename = cstring!(filename);
        let password = cstring!(password);
        unsafe { ((*(*self._file_manager).vtable).open_container)(self._file_manager, filename.as_ptr(), password.as_ptr(), mode) != 0 }
    }

    /// Close the current container
    pub fn close_container(&self) -> bool {
        unsafe { ((*(*self._file_manager).vtable).close_container)(self._file_manager) != 0 }
    }

    /// Returns true if a container is currently open
    pub fn is_open(&self) -> bool {
        unsafe { ((*(*self._file_manager).vtable).is_open)(self._file_manager) != 0 }
    }

    /// This function shouldn't be called since all file handles will be closed when they are dropped anyways
    fn close_all_files(&self) -> i32 {
        unsafe { ((*(*self._file_manager).vtable).close_all_files)(self._file_manager) }
    }

    /// Returns the MainModule-handle
    pub fn main_module_handle(&self) -> HMODULE {
        unsafe { ((*(*self._file_manager).vtable).main_module_handle)(self._file_manager) }
    }

    pub fn function_9(&self, i1: i32) -> i32 {
        unsafe { ((*(*self._file_manager).vtable).function_9)(self._file_manager, i1) }
    }

    /// Open a file inside the container using a path and returns a File object
    ///
    /// # Arguments
    ///
    /// * `filename` - filename, relative to current dir or absolute path inside archive
    /// * `access` - 0 for open-existing, 0x80000000 for open and share_read, 0x40000000 for create_always
    /// * `unknown` - not used for original CPFileManager
    pub fn open_file(&self, filename: &str, access: Access, unknown: i32) -> File {
        let filename = cstring!(filename);
        File::new(self, unsafe { ((*(*self._file_manager).vtable).open_file)(self._file_manager, filename.as_ptr(), access as i32, unknown)})
    }

    /// Open a file inside the container using the CJArchiveFm-class and returns a File object
    ///
    /// # Arguments
    ///
    /// * `fm` - A valid pointer to the CJArchiveFm-class
    /// * `filename` - filename, relative to current dir or absolute path inside archive
    /// * `access` - 0 for open-existing, 0x80000000 for open and share_read, 0x40000000 for create_always
    /// * `unknown` - not used for original CPFileManager
    pub fn open_file_cj(&self, fm: &mut CJArchiveFm, filename: &str, access: Access, unknown: i32) -> File {
        let filename = cstring!(filename);
        File::new(self, unsafe { ((*(*self._file_manager).vtable).open_file_cj)(self._file_manager, fm, filename.as_ptr(), access as i32, unknown) })
    }

    pub fn function_12(&self) -> i32 {
        unsafe { ((*(*self._file_manager).vtable).function_12)(self._file_manager) }
    }

    pub fn function_13(&self) -> i32 {
        unsafe { ((*(*self._file_manager).vtable).function_13)(self._file_manager) }
    }

    /// Create a file inside the container and returns a File object
    ///
    /// # Arguments
    ///
    /// * `filename` - filename, relative to current dir or absolute path inside archive
    /// * `unknown`
    pub fn create_file(&self, filename: &str, unknown: i32) -> File {
        let filename = cstring!(filename);
        File::new(self, unsafe { ((*(*self._file_manager).vtable).create_file)(self._file_manager, filename.as_ptr(), unknown) })
    }


    /// Create a file inside the container using the CJArchiveFm-class and returns a File object
    ///
    /// # Arguments
    ///
    /// * `fm` - A valid pointer to the CJArchiveFm-class
    /// * `filename` - filename, relative to current dir or absolute path inside archive
    /// * `unknown`
    pub fn create_file_cj(&self, fm: &mut CJArchiveFm, filename: &str, unknown: i32) -> File {
        let filename = cstring!(filename);
        File::new(self, unsafe { ((*(*self._file_manager).vtable).create_file_cj)(self._file_manager, fm, filename.as_ptr(), unknown) })
    }

    /// Delete a file by name
    pub fn delete_file(&self, filename: &str) -> i32 {
        let filename = cstring!(filename);
        unsafe { ((*(*self._file_manager).vtable).delete_file)(self._file_manager, filename.as_ptr()) }
    }

    /// Close file by handle, called by File only to manage its lifetime
    pub(crate) fn close_file(&self, file: &File) -> i32 {
        unsafe { ((*(*self._file_manager).vtable).close_file)(self._file_manager, file.handle) }
    }

    /// Read a number of bytes from file
    /// # Arguments
    /// Parameter:
    /// * `h_file` - Any handle or pointer identifiying this file
    /// * `lp_buffer` - pointer to reserved memory for read operation
    /// * `bytes_to_read` - size of lp_buffer
    /// * `bytes_read` - pointer to memory, will contain the number of bytes read from the file
    pub(crate) fn read(&self, file: &File, lp_buffer: &mut [u8], bytes_to_read: i32, bytes_read: *mut u32) -> i32 {
        unsafe { ((*(*self._file_manager).vtable).read)(self._file_manager, file.handle, lp_buffer.as_mut_ptr() as *mut i8, bytes_to_read, bytes_read) }
    }

    /// Write a number of bytes to file
    /// # Arguments
    /// Parameter:
    /// * `h_file` - Any handle or pointer identifiying this file
    /// * `lp_buffer` - pointer to reserved memory for write operation
    /// * `bytes_to_write` - size of lp_buffer
    /// * `bytes_written` - pointer to memory, will contain the number of bytes written the file
    pub(crate) fn write(&self, file: &File, lp_buffer: &[u8], bytes_to_write: i32, bytes_written: *mut u32) -> i32 {
        unsafe { ((*(*self._file_manager).vtable).write)(self._file_manager, file.handle, lp_buffer.as_ptr() as *const i8, bytes_to_write, bytes_written) }
    }

    pub fn cmd_line_path<'a>(&self) -> &'a CStr {
        unsafe {
            let charptr: *const c_char = ((*(*self._file_manager).vtable).cmd_line_path)(self._file_manager);
            CStr::from_ptr(charptr)
        }
    }

    pub fn cmd_line_exe<'a>(&self) -> &'a CStr {
        unsafe {
            let charptr: *const c_char = ((*(*self._file_manager).vtable).cmd_line_exe)(self._file_manager);
            CStr::from_ptr(charptr)
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

    /// Creates directory in the current pk2
    pub fn create_directory(&self, name: &str) -> bool {
        let name = cstring!(name);
        unsafe {
            ((*(*self._file_manager).vtable).create_dir)(self._file_manager, name.as_ptr()) != 0
        }
    }

    /// Deletes directory in the current pk2
    pub fn delete_directory(&self, name: &str) -> bool {
        let name = cstring!(name);
        unsafe {
            ((*(*self._file_manager).vtable).delete_dir)(self._file_manager, name.as_ptr()) != 0
        }
    }

    /// Resets the current working directory in the current pk2
    pub fn reset_directory(&self) -> bool {
        unsafe {
            ((*(*self._file_manager).vtable).reset_dir)(self._file_manager) != 0
        }
    }

    /// Changes the current working directory
    pub fn change_directory(&self, name: &str) -> bool {
        let name = cstring!(name);
        unsafe {
            ((*(*self._file_manager).vtable).change_dir)(self._file_manager, name.as_ptr()) != 0
        }
    }

    pub fn get_directory_name(&self) -> Result<String, FromUtf8Error> {
        unsafe {
            let mut buf = Vec::with_capacity(255);
            buf.set_len(255);
            let ptr = buf.as_mut_ptr();
            let len = ((*(*self._file_manager).vtable).get_dir_name)(self._file_manager, 200, ptr as *mut i8);
            buf.truncate(len as usize);
            String::from_utf8(buf)
        }
    }

    pub fn set_virtual_path(&self, path: &str) -> bool {
        let path = cstring!(path);
        unsafe {
            ((*(*self._file_manager).vtable).set_virtual_path)(self._file_manager, path.as_ptr()) != 0
        }
    }

    pub fn get_virtual_path(&self) -> Result<String, FromUtf8Error> {
        unsafe {
            let mut buf = Vec::with_capacity(255);
            buf.set_len(255);
            ((*(*self._file_manager).vtable).get_virtual_path)(self._file_manager, buf.as_mut_ptr() as *mut i8);
            if let Some(null_pos) = buf.iter().position(|&x| x == 0) {
                buf.truncate(null_pos);
            }
            String::from_utf8(buf)
        }
    }

    pub fn find_first_file(&self, search: &mut SearchResult, pattern: &str, entry: &mut ResultEntry) -> *mut SearchResult {
        let pattern = cstring!(pattern);
        unsafe {
            ((*(*self._file_manager).vtable).find_first_file)(self._file_manager, search, pattern.as_ptr(), entry)
        }
    }

    pub fn find_next_file(&self, search: &mut SearchResult, entry: &mut ResultEntry) -> i32 {
        unsafe {
            ((*(*self._file_manager).vtable).find_next_file)(self._file_manager, search, entry)
        }
    }

    pub fn find_close(&self, search: &mut SearchResult) -> i32 {
        unsafe {
            ((*(*self._file_manager).vtable).close_search_result)(self._file_manager, search)
        }
    }

    #[allow(dead_code)]
    pub(crate) fn file_name_from_handle(&self, file: &File, count: usize) -> Result<String, FromUtf8Error> {
        unsafe {
            let mut buf = Vec::with_capacity(255);
            buf.set_len(255);
            ((*(*self._file_manager).vtable).file_name_from_handle)(self._file_manager, file.handle, buf.as_mut_ptr() as *mut i8, count);
            if let Some(null_pos) = buf.iter().position(|&x| x == 0) {
                buf.truncate(null_pos);
            }
            String::from_utf8(buf)
        }
    }

    pub(crate) fn get_file_size(&self, file: &File) -> i32 {
        unsafe {
            ((*(*self._file_manager).vtable).get_file_size)(self._file_manager, file.handle, null_mut())
        }
    }

    pub(crate) fn get_file_time(&self, file: &File, creation_time: LPFILETIME, last_write_time: LPFILETIME) -> bool {
        unsafe {
            ((*(*self._file_manager).vtable).get_file_time)(self._file_manager, file.handle, creation_time, last_write_time)
        }
    }

    pub(crate) fn set_file_time(&self, file: &File, creation_time: LPFILETIME, last_write_time: LPFILETIME) -> bool {
        unsafe {
            ((*(*self._file_manager).vtable).set_file_time)(self._file_manager, file.handle, creation_time, last_write_time)
        }
    }

    pub(crate) fn seek(&self, file: &File, distance_to_move: c_long, move_method: DWORD) -> i32{
        unsafe {
            ((*(*self._file_manager).vtable).seek)(self._file_manager, file.handle, distance_to_move, move_method)
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

    pub fn for_each_entry_in_container(&self, callback: ForEachCallback, filter: &str, userstate: *mut ::std::os::raw::c_void) -> i32 {
        let filter = cstring!(filter);
        unsafe {
            ((*(*self._file_manager).vtable).for_each_entry_in_container)(self._file_manager, callback, filter.as_ptr(), userstate)
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

    pub fn unlock(&self) -> bool {
        unsafe {
            ((*(*self._file_manager).vtable).unlock)(self._file_manager) != 0
        }
    }

    pub fn lock(&self, i1: i32) -> bool {
        unsafe {
            ((*(*self._file_manager).vtable).lock)(self._file_manager, i1) != 0
        }
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
    find_first_file: extern "thiscall" fn(*mut IFileManager, *mut SearchResult, *const c_char, *mut ResultEntry) -> *mut SearchResult,
    find_next_file: extern "thiscall" fn(*mut IFileManager, *mut SearchResult, *mut ResultEntry) -> c_int,
    close_search_result: extern "thiscall" fn(*mut IFileManager, *mut SearchResult) -> c_int,
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
    for_each_entry_in_container: extern "thiscall" fn(*mut IFileManager, ForEachCallback, *const c_char, *mut ::std::os::raw::c_void) -> c_int,
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
        unsafe { GFXDllCreateObject(mode, &mut obj, version); }
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