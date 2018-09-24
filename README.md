rlibc
=====

**DEPRECATED**: This crate used to provide a Rust implementation of some `libc`
functions such as `memcpy` / `memmove` / `memset`, required when developing
freestanding applications.

The [compiler-builtins] crate now provides optimized versions of these functions
when enabling the optional `mem` feature. It also provides a lot of other useful
functions required by bare metal applications. It should be used instead of this one.

The old version of this crate is still browsable by looking in [Git's history][old].
No further development will happen here.

[compiler-builtins]: https://github.com/rust-lang-nursery/compiler-builtins
[old]: https://github.com/alexcrichton/rlibc/commit/defb486e765846417a8e73329e8c5196f1dca49a
