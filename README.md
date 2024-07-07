# win32-ffi-test

### This project compares FFI overhead when calling Win32 APIs for window creation.

#### C++ (for reference):
1. 39 ms
2. 39 ms
3. 39 ms
4. 40 ms
5. 39 ms

#### Rust (windows-sys crate):
1. 40 ms
2. 39 ms
3. 39 ms
4. 40 ms
5. 39 ms

Thanks to Rust's [zero-cost FFI calls for C code](https://blog.rust-lang.org/2015/04/24/Rust-Once-Run-Everywhere.html), the performance is identical to native C/C++ code
