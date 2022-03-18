use crate::{CompressedCapability,CcxCap,CcxBoundsBits};
use crate::ffi_num::{FfiU128,FfiI128};

type Length = u128;
type Offset = i128;
type FfiLength = FfiU128;
type FfiOffset = FfiI128;
type Addr = u64;

type Cap = CcxCap<Cc128>;
pub type Cc128Cap = Cap;

/// Import C functions for CC64
#[link(name = "cheri_compressed_cap")]
extern "C" {
    fn cc128_compress_raw(src_cap: *const Cap) -> Addr;
    fn cc128_decompress_raw(pesbt: Addr, cursor: Addr, tag: bool, out_cap: *mut Cap);
    fn cc128_compress_mem(src_cap: *const Cap) -> Addr;
    fn cc128_decompress_mem(pesbt: Addr, cursor: Addr, tag: bool, out_cap: *mut Cap);

    /* Getters */
    fn cc128_get_uperms(cap: *const Cap) -> u32;
    fn cc128_get_perms(cap: *const Cap) -> u32;
    fn cc128_get_otype(cap: *const Cap) -> u32;
    fn cc128_get_reserved(cap: *const Cap) -> u8;
    fn cc128_get_flags(cap: *const Cap) -> u8;

    /* Updaters */
    fn cc128_update_uperms(cap: *mut Cap, value: Addr);
    fn cc128_update_perms(cap: *mut Cap, value: Addr);
    fn cc128_update_otype(cap: *mut Cap, value: Addr);
    fn cc128_update_reserved(cap: *mut Cap, value: Addr);
    fn cc128_update_flags(cap: *mut Cap, value: Addr);

    /* Misc */
    fn cc128_extract_bounds_bits(pesbt: Addr) -> CcxBoundsBits;
    fn cc128_setbounds(cap: *mut Cap, req_base: Addr, req_top: FfiLength) -> bool;
    fn cc128_is_representable_cap_exact(cap: *const Cap) -> bool;
    fn cc128_make_max_perms_cap(base: Addr, cursor: Addr, top: FfiLength) -> Cap;
    fn cc128_get_representable_length(length: Addr) -> Addr;
    fn cc128_get_required_alignment(length: Addr) -> Addr;
    fn cc128_get_alignment_mask(length: Addr) -> Addr;
}

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

    /// ```_CC_N(OTYPE_UNSEALED) = (_CC_N(MAX_REPRESENTABLE_OTYPE) - 0u),
    /// _CC_N(OTYPE_UNSEALED_SIGNED) = (((int64_t)-1) - 0u)```
    /// The OTYPE field is 4 bits (50:47) in CC64
    const MAX_REPRESENTABLE_OTYPE: u32 = 0b1111;
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
    fn set_bounds(cap: &mut Cap, req_base: Self::Addr, req_top: Self::FfiLength) -> bool {
        unsafe { cc128_setbounds(cap, req_base, req_top) }
    }
    fn is_representable_cap_exact(cap: &Cap) -> bool {
        unsafe { cc128_is_representable_cap_exact(cap) }
    }
    fn make_max_perms_cap(base: Self::Addr, cursor: Self::Addr, top: Self::FfiLength) -> Cap {
        unsafe { cc128_make_max_perms_cap(base, cursor, top) }
    }
    fn get_representable_length(length: Self::Addr) -> Self::Addr {
        unsafe { cc128_get_representable_length(length) }
    }
    fn get_required_alignment(length: Self::Addr) -> Self::Addr {
        unsafe { cc128_get_required_alignment(length) }
    }
    fn get_alignment_mask(length: Self::Addr) -> Self::Addr {
        unsafe { cc128_get_alignment_mask(length) }
    }
}