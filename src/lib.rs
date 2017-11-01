#![cfg(all(target_os = "windows", target_arch = "x86"))]
#![feature(abi_thiscall)]
#![feature(read_initializer)]
extern crate winapi;

pub mod file_manager;
pub mod cjarchivefm;
pub mod gfxinfo;
mod ffi;
pub mod dialog;
pub mod search_result;
pub mod result_entry;
pub mod gfxfile;