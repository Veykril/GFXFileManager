use std::io::{Initializer, Read, Result, Seek, SeekFrom, Write};

use winapi::c_int;
use winapi::{FILETIME, LPFILETIME};

use file_manager::GFXFileManager;

pub struct File<'a> {
    handle: c_int,
    file_manager: &'a GFXFileManager,
}

impl<'a> File<'a> {
    pub fn new(file_manager: &'a GFXFileManager, handle: c_int) -> File {
        File {
            handle,
            file_manager,
        }
    }

    #[inline(always)]
    pub(crate) fn handle(&self) -> c_int {
        self.handle
    }

    pub fn len(&self) -> u64 {
        self.file_manager.get_file_size(self) as u64
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the creation and last write time of this file
    pub fn file_time(&self) -> (FILETIME, FILETIME) {
        unsafe {
            let mut creation_time = ::std::mem::uninitialized();
            let mut last_write_time = ::std::mem::uninitialized();
            self.file_manager.get_file_time(self, &mut creation_time, &mut last_write_time);
            (creation_time, last_write_time)
        }
    }

    pub fn set_file_time(&self, creation_time: LPFILETIME, last_write_time: LPFILETIME) {
        self.file_manager.set_file_time(self, creation_time, last_write_time);
    }

    pub fn name(&self) -> String {
        self.file_manager.file_name_from_handle(self).unwrap()
    }
}

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

impl<'a> Write for File<'a> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let len = buf.len();
        let mut bytes_written = 0;
        self.file_manager.write(self, buf, len as i32, &mut bytes_written);
        Ok(bytes_written as usize)
    }

    /// Flushing a file is not possible as far as I know
    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

impl<'a> Seek for File<'a> {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        let (move_method, distance_to_move) = {
            match pos {
                SeekFrom::Start(u) => {
                    (0, u as i32)
                },
                SeekFrom::Current(i) => {
                    (1, i as i32)
                },
                SeekFrom::End(i) => {
                    (2, i as i32)
                },
            }
        };
        Ok(self.file_manager.seek(self, distance_to_move, move_method) as u64)
    }
}

impl<'a> Drop for File<'a> {
    fn drop(&mut self) {
        self.file_manager.close_file(self);
    }
}
