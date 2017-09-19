use GFXFileManager;
use winapi::c_int;

use std::io::{Read, Result, Initializer};

pub struct File<'a> {
    pub handle: c_int,
    file_manager: &'a GFXFileManager,
}

impl<'a> File<'a> {
    pub fn file_size(&self) -> i32 {
        self.file_manager.get_file_size(self)
    }
}

//seeking is not implemented correctly
impl<'a> Read for File<'a> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let len = buf.len();
        let mut bytes_read = 0;
        self.file_manager.read(self, buf, len as i32, &mut bytes_read);
        Ok(bytes_read as usize)
    }

    #[inline]
    unsafe fn initializer(&self) -> Initializer {
        Initializer::nop()
    }
}

pub fn new<'a>(file_manager: &'a GFXFileManager, handle: c_int) -> File {
    File {
        handle: handle,
        file_manager: file_manager,
    }
}

impl<'a> Drop for File<'a> {
    fn drop(&mut self) {
        self.file_manager.close_file(self.handle);
    }
}