use std::path::PathBuf;
use std::fs;

fn main() {
    let path = PathBuf::from("lib/");
    println!("cargo:rustc-link-search=native={}", fs::canonicalize(&path).unwrap().to_str().unwrap());
    println!("cargo:rustc-link-lib=dylib=GFXFileManager");
}