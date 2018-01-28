# GFXFileManager

This is a rust dll wrapper for the GFXFileManager.dll that is used by Silkroad Online for working with their pk2-container file format.

Made available thanks to [Florian0's research](https://github.com/florian0/GFXFileManager)

## Work in progress

It still contains bugs, there are probably some functions that arent wrapped correctly, or maybe some that are swapped by accident. 
The structure of my implementation is constantly changing because I am slowly trying to make everything more rustlike, so be aware that this crate is likely to break a lot.

## Build information

This library only builds on 32-bit windows target because the dll itself is 32-bit, so make sure to build your project by running `cargo build --target=i686-pc-windows-msvc`