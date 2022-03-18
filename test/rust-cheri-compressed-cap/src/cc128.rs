use crate::{CompressedCapability,CcxCap,CcxBoundsBits};
use crate::ffi_num::{FfiU128,FfiI128};
use crate::c_funcs::*;

pub type Length = u128;
pub type Offset = i128;
pub type FfiLength = FfiU128;
pub type FfiOffset = FfiI128;
pub type Addr = u64;

pub type Cap = CcxCap<Cc128>;
pub type Cc128Cap = Cap;

/// Defines the CC64 capability profile as an implementation of the CompressedCapability trait.
/// 
/// Empty enum, so it cannot be itself constructed. If you want a CC64 capability, instantiate CC64::Cap.
/// 
/// Derives Debug, Copy, Clone so that CcxCap<Cc64> can derive them too.
#[derive(Debug,Copy,Clone)]
pub enum Cc128 {}
impl CompressedCapability for Cc128 {
    type Length = Length;
    type Offset = Offset;
    type Addr = Addr;
    
    type FfiLength = FfiLength;
    type FfiOffset = FfiOffset;

    /// The OTYPE field is 18 bits (108:91) in CC128
    const MAX_REPRESENTABLE_OTYPE: u32 = 0x3_FFFF;
    const OTYPE_UNSEALED:  u32 = Self::MAX_REPRESENTABLE_OTYPE - 0;
    const OTYPE_SENTRY:    u32 = Self::MAX_REPRESENTABLE_OTYPE - 1;
    const OTYPE_RESERVED2: u32 = Self::MAX_REPRESENTABLE_OTYPE - 2;
    const OTYPE_RESERVED3: u32 = Self::MAX_REPRESENTABLE_OTYPE - 3;
    const MAX_UNRESERVED_OTYPE: u32 = Self::MAX_REPRESENTABLE_OTYPE - 4;

    fn compress_raw(cap: &Cap) -> Addr {
        unsafe { cc128_compress_raw(cap) }
    }
    fn decompress_raw(pesbt: Addr, cursor: Addr, tag: bool) -> Cap {
        let mut cap = Default::default();
        unsafe { cc128_decompress_raw(pesbt, cursor, tag, &mut cap) };
        cap
    }
    fn compress_mem(cap: &Cap) -> Addr {
        unsafe { cc128_compress_mem(cap) }
    }
    fn decompress_mem(pesbt: Addr, cursor: Addr, tag: bool) -> Cap {
        let mut cap = Default::default();
        unsafe { cc128_decompress_mem(pesbt, cursor, tag, &mut cap) };
        cap
    }

    /* Getters */
    fn get_uperms(cap: &Cap) -> u32 {
        unsafe { cc128_get_uperms(cap) }
    }
    fn get_perms(cap: &Cap) -> u32 {
        unsafe { cc128_get_perms(cap) }
    }
    fn get_otype(cap: &Cap) -> u32 {
        unsafe { cc128_get_otype(cap) }
    }
    fn get_reserved(cap: &Cap) -> u8 {
        unsafe { cc128_get_reserved(cap) }
    }
    fn get_flags(cap: &Cap) -> u8 {
        unsafe { cc128_get_flags(cap) }
    }

    /* Updaters */
    fn update_uperms(cap: &mut Cap, value: u32) {
        unsafe { cc128_update_uperms(cap, value as Self::Addr) }
    }
    fn update_perms(cap: &mut Cap, value: u32) {
        unsafe { cc128_update_perms(cap, value as Self::Addr) }
    }
    fn update_otype(cap: &mut Cap, value: u32) {
        unsafe { cc128_update_otype(cap, value as Self::Addr) }
    }
    fn update_reserved(cap: &mut Cap, value: u8) {
        unsafe { cc128_update_reserved(cap, value as Self::Addr) }
    }
    fn update_flags(cap: &mut Cap, value: u8) {
        unsafe { cc128_update_flags(cap, value as Self::Addr) }
    }

    /* Misc */
    fn extract_bounds_bits(pesbt: Self::Addr) -> CcxBoundsBits {
        unsafe { cc128_extract_bounds_bits(pesbt) }
    }
    fn set_bounds(cap: &mut Cap, req_base: Self::Addr, req_top: Self::Length) -> bool {
        unsafe { cc128_setbounds(cap, req_base, req_top.into()) }
    }
    fn is_representable_cap_exact(cap: &Cap) -> bool {
        unsafe { cc128_is_representable_cap_exact(cap) }
    }
    fn is_representable_new_addr(sealed: bool, base: Self::Addr, length: Self::Length, cursor: Self::Addr, new_cursor: Self::Addr) -> bool {
        unsafe { cc128_is_representable_new_addr(sealed, base, length.into(), cursor, new_cursor) }
    }
    fn make_max_perms_cap(base: Self::Addr, cursor: Self::Addr, top: Self::Length) -> Cap {
        unsafe { cc128_make_max_perms_cap(base, cursor, top.into()) }
    }
    fn get_representable_length(length: Self::Length) -> Self::Length {
        unsafe { cc128_get_representable_length(length.into()).into() }
    }
    fn get_required_alignment(length: Self::Length) -> Self::Length {
        unsafe { cc128_get_required_alignment(length.into()).into() }
    }
    fn get_alignment_mask(length: Self::Length) -> Self::Length {
        unsafe { cc128_get_alignment_mask(length.into()).into() }
    }
}