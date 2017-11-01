// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A bare-metal library supplying functions rustc may lower code to
//!
//! This library is not intended for general use, and is superseded by a system
//! libc if one is available. In a freestanding context, however, common
//! functions such as memset, memcpy, etc are not implemented. This library
//! provides an implementation of these functions which are either required by
//! libcore or called by rustc implicitly.
//!
//! This library is never included by default, and must be manually included if
//! necessary. It is an error to include this library when also linking with
//! the system libc library.

#![cfg_attr(feature = "nightly", feature(asm))]

#![no_std]

// This library defines the builtin functions, so it would be a shame for
// LLVM to optimize these function calls to themselves!
#![no_builtins]

// NOTE `linkage = weak` doesn't work for Windows (COFF) or MacOS (MachO). It seems it only works
// for ELF objects.
#![cfg_attr(all(feature = "weak", not(windows), not(target_os = "macos")), feature(linkage))]

#[cfg_attr(all(feature = "weak", not(windows), not(target_os = "macos")), linkage = "weak")]
#[cfg(not(all(target_arch="x86_64", feature = "nightly")))]
#[no_mangle]
pub unsafe extern fn memcpy(dest: *mut u8, src: *const u8,
                            n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *dest.offset(i as isize) = *src.offset(i as isize);
        i += 1;
    }
    return dest;
}

#[cfg(all(target_arch="x86_64", feature = "nightly"))]
#[no_mangle]
pub unsafe extern fn memcpy(dest: *mut u8, src: *const u8,
                            n: usize) -> *mut u8 {
    let mut _i: usize;
	let mut _j: usize;
	let mut _k: usize;

	asm!(
		"cld; rep movsq; movq $4, %rcx; andq $$7, %rcx; rep movsb\n\t"
		: "={rcx}"(_i), "={rdx}"(_j), "={rsi}"(_k)
		: "0"(n/8), "r"(n), "1"(dest), "2"(src) : "memory","cc");

    return dest;
}

#[cfg_attr(all(feature = "weak", not(windows), not(target_os = "macos")), linkage = "weak")]
#[cfg(not(all(target_arch="x86_64", feature = "nightly")))]
#[no_mangle]
pub unsafe extern fn memmove(dest: *mut u8, src: *const u8,
                             n: usize) -> *mut u8 {
    if src < dest as *const u8 { // copy from end
        let mut i = n;
        while i != 0 {
            i -= 1;
            *dest.offset(i as isize) = *src.offset(i as isize);
        }
    } else { // copy from beginning
        let mut i = 0;
        while i < n {
            *dest.offset(i as isize) = *src.offset(i as isize);
            i += 1;
        }
    }
    return dest;
}

#[cfg_attr(all(feature = "weak", not(windows), not(target_os = "macos")), linkage = "weak")]
#[cfg(all(target_arch="x86_64", feature = "nightly"))]
#[no_mangle]
pub unsafe extern fn memmove(dest: *mut u8, src: *const u8,
                             n: usize) -> *mut u8 {
	let mut _i: usize;
	let mut _j: usize;
	let mut _k: usize;

    if src < dest as *const u8 { // copy from end
		if n >= 8 {
			asm!(
				"std; rep movsq; movq $4, %rcx; andq $$7, %rcx; rep movsb; cld"
				: "={rcx}"(_i), "={rdx}"(_j), "={rsi}"(_k)
				: "0"(n/8), "r"(n), "1"(dest.offset((n-8) as isize)), "2"(src.offset((n-8) as isize)) : "memory","cc");
		} else if n > 0 {
			asm!(
				"std; rep movsb; cld"
				: "={rcx}"(_i), "={rdx}"(_j), "={rsi}"(_k)
				: "0"(n), "1"(dest.offset((n-1) as isize)), "2"(src.offset((n-1) as isize)) : "memory","cc");
		}
    } else if n > 0 { // copy from beginning
		asm!(
			"cld; rep movsq; movq $4, %rcx; andq $$7, %rcx; rep movsb"
			: "={rcx}"(_i), "={rdx}"(_j), "={rsi}"(_k)
			: "0"(n/8), "r"(n), "1"(dest), "2"(src) : "memory","cc");
    }

    return dest;
}

#[cfg_attr(all(feature = "weak", not(windows), not(target_os = "macos")), linkage = "weak")]
#[cfg(not(all(target_arch="x86_64", feature = "nightly")))]
#[no_mangle]
pub unsafe extern fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *s.offset(i as isize) = c as u8;
        i += 1;
    }
    return s;
}

#[cfg(all(target_arch="x86_64", feature = "nightly"))]
#[no_mangle]
pub unsafe extern fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
	let mut _i: usize;
	let mut _j: usize;

	if c != 0 {
		asm!("cld; rep stosb"
			: "={rcx}"(_i), "={rdi}"(_j)
			: "rax"(c), "1"(s), "0"(n) : "memory","cc" : "volatile");
	} else {
		asm!(
			"cld; rep stosq; movq $5, %rcx; andq $$7, %rcx; rep stosb\n\t"
			: "={rcx}"(_i), "={rdi}"(_j)
			: "rax"(0x00), "1"(s), "0"(n/8), "r"(n): "memory","cc" : "volatile");
	}

    return s;
}

#[cfg_attr(all(feature = "weak", not(windows), not(target_os = "macos")), linkage = "weak")]
#[no_mangle]
pub unsafe extern fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    let mut i = 0;
    while i < n {
        let a = *s1.offset(i as isize);
        let b = *s2.offset(i as isize);
        if a != b {
            return a as i32 - b as i32
        }
        i += 1;
    }
    return 0;
}

#[cfg(test)]
mod test {
    use super::{memcmp, memset, memcpy, memmove};

    #[test]
    fn memcmp_single_byte_pointers() {
        unsafe {
            assert_eq!(memcmp(&0xFAu8, &0xFAu8, 1), 0x00);
            assert!(memcmp(&0xEFu8, &0xFEu8, 1) < 0x00);
        }
    }

    #[test]
    fn memcmp_strings() {
        {
            let (x, z) = ("Hello!", "Good Bye.");
            let l = x.len();
            unsafe {
                assert_eq!(memcmp(x.as_ptr(), x.as_ptr(), l), 0);
                assert!(memcmp(x.as_ptr(), z.as_ptr(), l) > 0);
                assert!(memcmp(z.as_ptr(), x.as_ptr(), l) < 0);
            }
        }
        {
            let (x, z) = ("hey!", "hey.");
            let l = x.len();
            unsafe {
                assert!(memcmp(x.as_ptr(), z.as_ptr(), l) < 0);
            }
        }
    }

    #[test]
    fn memset_single_byte_pointers() {
        let mut x: u8 = 0xFF;
        unsafe {
            memset(&mut x, 0xAA, 1);
            assert_eq!(x, 0xAA);
            memset(&mut x, 0x00, 1);
            assert_eq!(x, 0x00);
            x = 0x01;
            memset(&mut x, 0x12, 0);
            assert_eq!(x, 0x01);
        }
    }

    #[test]
    fn memset_array() {
        let mut buffer = [b'X';  100];
        unsafe {
            memset(buffer.as_mut_ptr(), b'#' as i32, buffer.len());
        }
        for byte in buffer.iter() { assert_eq!(*byte, b'#'); }
    }

    #[test]
    fn memcpy_and_memcmp_arrays() {
        let (src, mut dst) = ([b'X';  100], [b'Y';  100]);
        unsafe {
            assert!(memcmp(src.as_ptr(), dst.as_ptr(), 100) != 0);
            let _ = memcpy(dst.as_mut_ptr(), src.as_ptr(), 100);
            assert_eq!(memcmp(src.as_ptr(), dst.as_ptr(), 100), 0);
        }
    }

    #[test]
    fn memmove_overlapping() {
        {
            let mut buffer = [ b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9' ];
            unsafe {
                memmove(&mut buffer[4], &buffer[0], 6);
                let mut i = 0;
                for byte in b"0123012345".iter() {
                    assert_eq!(buffer[i], *byte);
                    i += 1;
                }
            }
        }
        {
            let mut buffer = [ b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9' ];
            unsafe {
                memmove(&mut buffer[0], &buffer[4], 6);
                let mut i = 0;
                for byte in b"4567896789".iter() {
                    assert_eq!(buffer[i], *byte);
                    i += 1;
                }
            }
        }
    }
}
