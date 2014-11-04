rlibc
=====

A bare minimum "libc" for Rust crates that do not want to rely on libc itself.
This crate provides functions which LLVM often lowers intrinsic calls to and
will be required to link correctly.
