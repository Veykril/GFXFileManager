#![cfg(all(target_os = "windows", target_arch = "x86"))]
#![feature(abi_thiscall)]
#![feature(read_initializer)]
extern crate winapi;

pub mod cjarchivefm;
pub mod dialog;
pub mod file_manager;
pub mod gfxfile;
pub mod gfxinfo;
pub mod result_entry;
pub mod search_result;

pub use cjarchivefm::CJArchiveFm;
pub use dialog::DialogData;
pub use file_manager::{Access, CallbackState, GFXFileManager, Mode, UnknownPair};
pub use file_manager::{ErrorHandler, ForEachCallback};
pub use gfxfile::File;
pub use gfxinfo::GFXInfo;
pub use result_entry::{Entry, ResultEntry};
pub use search_result::SearchResult;

mod ffi;
