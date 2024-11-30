use std::fs;
use std::path::PathBuf;

fn main() {
    let path = PathBuf::from("lib/");
    println!(
        "cargo:rustc-link-search=native={}",
        fs::canonicalize(&path).unwrap().to_str().unwrap()
    );
    println!("cargo:rustc-link-lib=dylib=GFXFileManager");
}
