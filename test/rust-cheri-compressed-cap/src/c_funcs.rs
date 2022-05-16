//! Import C functions for CC64,128.
//! 
//! We used to do this separately in cc64.rs and cc128.rs for the 64 and 128 bit versions.
//! This seemed to cause multiple-definition link errors, so now we link all of them in one place.
//! 
//! Normally Rust warns us about using [u128], [i128] in FFI function arguments, because there isn't a well-defined ABI.
//! We initially tried to create a repr(C) struct that matched the ABI, but this seems impossible.
//! Clang (as of LLVM-13) defines [u128],[i128] as built-in types which are passed in two registers.
//! This cannot be replicated exactly by passing a struct - see `clang/lib/CodeGen/TargetInfo.cpp: X86_64ABIInfo::classify()`,
//! and [this godbolt example](https://godbolt.org/z/arn438z77).
//! 
//! Therefore, we instead use Rust native [u128], [i128] under the assumption that
//! 1. Rust is being compiled under LLVM
//! 2. Rust native u128, i128 are represented with LLVM's built-in 128-bit types
//! 3. The C code is also compiled with LLVM, using the built-in 128-bit types.
//! 4. The versions of LLVM used to compile Rust, C are ABI-compatible for 128-bit types.
//! 
//! Assumption 3 may not be necessary - maintaining compatibility with GCC is in Clang's interest, so GCC-compiled C may work just as well.
//! Assumption 4 is difficult to check, we're taking it on faith that LLVM doesn't do anything silly in the future.
//! 
//! Under all these assumptions, we can safely ignore Rust's "improper ctypes" warning.

use crate::CcxBoundsBits;
use crate::cc128;
use crate::cc64;

#[link(name = "cheri_compressed_cap_lib")]
#[allow(improper_ctypes)]
extern "C" {
    pub(crate) fn cc64_compress_raw(src_cap: *const cc64::Cap) -> cc64::Addr;
    pub(crate) fn cc64_decompress_raw(pesbt: cc64::Addr, cursor: cc64::Addr, tag: bool, out_cap: *mut cc64::Cap);
    pub(crate) fn cc64_compress_mem(src_cap: *const cc64::Cap) -> cc64::Addr;
    pub(crate) fn cc64_decompress_mem(pesbt: cc64::Addr, cursor: cc64::Addr, tag: bool, out_cap: *mut cc64::Cap);

    /* Getters */
    pub(crate) fn cc64_get_uperms(cap: *const cc64::Cap) -> u32;
    pub(crate) fn cc64_get_perms(cap: *const cc64::Cap) -> u32;
    pub(crate) fn cc64_get_otype(cap: *const cc64::Cap) -> u32;
    pub(crate) fn cc64_get_reserved(cap: *const cc64::Cap) -> u8;
    pub(crate) fn cc64_get_flags(cap: *const cc64::Cap) -> u8;

    /* Updaters */
    pub(crate) fn cc64_update_uperms(cap: *mut cc64::Cap, value: cc64::Addr);
    pub(crate) fn cc64_update_perms(cap: *mut cc64::Cap, value: cc64::Addr);
    pub(crate) fn cc64_update_otype(cap: *mut cc64::Cap, value: cc64::Addr);
    pub(crate) fn cc64_update_reserved(cap: *mut cc64::Cap, value: cc64::Addr);
    pub(crate) fn cc64_update_flags(cap: *mut cc64::Cap, value: cc64::Addr);

    /* Misc */
    pub(crate) fn cc64_extract_bounds_bits(pesbt: cc64::Addr) -> CcxBoundsBits;
    pub(crate) fn cc64_setbounds(cap: *mut cc64::Cap, req_base: cc64::Addr, req_top: cc64::FfiLength) -> bool;
    pub(crate) fn cc64_is_representable_cap_exact(cap: *const cc64::Cap) -> bool;
    pub(crate) fn cc64_is_representable_new_addr(sealed: bool, base: cc64::Addr, length: cc64::FfiLength, cursor: cc64::Addr, new_cursor: cc64::Addr) -> bool;
    pub(crate) fn cc64_make_max_perms_cap(base: cc64::Addr, cursor: cc64::Addr, top: cc64::FfiLength) -> cc64::Cap;
    pub(crate) fn cc64_get_representable_length(length: cc64::FfiLength) -> cc64::FfiLength;
    pub(crate) fn cc64_get_required_alignment(length: cc64::FfiLength) -> cc64::FfiLength;
    pub(crate) fn cc64_get_alignment_mask(length: cc64::FfiLength) -> cc64::FfiLength;

    // ------------ 128 ----------------


    pub(crate) fn cc128_compress_raw(src_cap: *const cc128::Cap) -> cc128::Addr;
    pub(crate) fn cc128_decompress_raw(pesbt: cc128::Addr, cursor: cc128::Addr, tag: bool, out_cap: *mut cc128::Cap);
    pub(crate) fn cc128_compress_mem(src_cap: *const cc128::Cap) -> cc128::Addr;
    pub(crate) fn cc128_decompress_mem(pesbt: cc128::Addr, cursor: cc128::Addr, tag: bool, out_cap: *mut cc128::Cap);

    /* Getters */
    pub(crate) fn cc128_get_uperms(cap: *const cc128::Cap) -> u32;
    pub(crate) fn cc128_get_perms(cap: *const cc128::Cap) -> u32;
    pub(crate) fn cc128_get_otype(cap: *const cc128::Cap) -> u32;
    pub(crate) fn cc128_get_reserved(cap: *const cc128::Cap) -> u8;
    pub(crate) fn cc128_get_flags(cap: *const cc128::Cap) -> u8;

    /* Updaters */
    pub(crate) fn cc128_update_uperms(cap: *mut cc128::Cap, value: cc128::Addr);
    pub(crate) fn cc128_update_perms(cap: *mut cc128::Cap, value: cc128::Addr);
    pub(crate) fn cc128_update_otype(cap: *mut cc128::Cap, value: cc128::Addr);
    pub(crate) fn cc128_update_reserved(cap: *mut cc128::Cap, value: cc128::Addr);
    pub(crate) fn cc128_update_flags(cap: *mut cc128::Cap, value: cc128::Addr);

    /* Misc */
    pub(crate) fn cc128_extract_bounds_bits(pesbt: cc128::Addr) -> CcxBoundsBits;
    pub(crate) fn cc128_setbounds(cap: *mut cc128::Cap, req_base: cc128::Addr, req_top: cc128::FfiLength) -> bool;
    pub(crate) fn cc128_is_representable_cap_exact(cap: *const cc128::Cap) -> bool;
    pub(crate) fn cc128_is_representable_new_addr(sealed: bool, base: cc128::Addr, length: cc128::FfiLength, cursor: cc128::Addr, new_cursor: cc128::Addr) -> bool;
    pub(crate) fn cc128_make_max_perms_cap(base: cc128::Addr, cursor: cc128::Addr, top: cc128::FfiLength) -> cc128::Cap;
    pub(crate) fn cc128_get_representable_length(length: cc128::FfiLength) -> cc128::FfiLength;
    pub(crate) fn cc128_get_required_alignment(length: cc128::FfiLength) -> cc128::FfiLength;
    pub(crate) fn cc128_get_alignment_mask(length: cc128::FfiLength) -> cc128::FfiLength;
}