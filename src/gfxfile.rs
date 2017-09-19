use GFXFileManager;
use std::os::raw::c_int;


pub struct File<'a> {
    pub handle: c_int,
    file_manager: &'a GFXFileManager,
}

impl<'a> File<'a> {
    pub fn file_size(&self) -> i32 {
        self.file_manager.get_file_size(self)
    }
}

pub fn new<'a>(file_manager: &'a GFXFileManager, handle: c_int) -> File {
    File {
        handle: handle,
        file_manager: file_manager,
    }
}
//todo implement Read trait

impl<'a> Drop for File<'a> {
    fn drop(&mut self) {
        self.file_manager.close_file(self.handle);
    }
}