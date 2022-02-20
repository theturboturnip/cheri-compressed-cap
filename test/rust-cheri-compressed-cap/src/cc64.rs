use crate::{CompressedCapability,CcxCap,CcxBoundsBits};

type Length = u64;
type Offset = i64;
type Addr = u32;

type Cap = CcxCap<Cc64>;
pub type Cc64Cap = Cap;

/// Import C functions for CC64
#[link(name = "cheri_compressed_cap", kind = "static")]
extern "C" {
    fn cc64_compress_raw(src_cap: *const Cap) -> Addr;
    fn cc64_decompress_raw(pesbt: Addr, cursor: Addr, tag: bool, out_cap: *mut Cap);
    fn cc64_compress_mem(src_cap: *const Cap) -> Addr;
    fn cc64_decompress_mem(pesbt: Addr, cursor: Addr, tag: bool, out_cap: *mut Cap);

    /* Getters */
    fn cc64_get_uperms(cap: *const Cap) -> u32;
    fn cc64_get_perms(cap: *const Cap) -> u32;
    fn cc64_get_otype(cap: *const Cap) -> u32;
    fn cc64_get_reserved(cap: *const Cap) -> u8;
    fn cc64_get_flags(cap: *const Cap) -> u8;

    /* Updaters */
    fn cc64_update_uperms(cap: *mut Cap, value: Addr);
    fn cc64_update_perms(cap: *mut Cap, value: Addr);
    fn cc64_update_otype(cap: *mut Cap, value: Addr);
    fn cc64_update_reserved(cap: *mut Cap, value: Addr);
    fn cc64_update_flags(cap: *mut Cap, value: Addr);

    /* Misc */
    fn cc64_extract_bounds_bits(pesbt: Addr) -> CcxBoundsBits;
    fn cc64_setbounds(cap: *mut Cap, req_base: Addr, req_top: Addr) -> bool;
    fn cc64_is_representable_cap_exact(cap: *const Cap) -> bool;
    fn cc64_make_max_perms_cap(base: Addr, cursor: Addr, top: Addr) -> Cap;
    fn cc64_get_representable_length(length: Addr) -> Addr;
    fn cc64_get_required_alignment(length: Addr) -> Addr;
    fn cc64_get_alignment_mask(length: Addr) -> Addr;
}

/// Defines the CC64 capability profile as an implementation of the CompressedCapability trait.
/// 
/// Empty enum, so it cannot be itself constructed. If you want a CC64 capability, instantiate CC64::Cap.
pub enum Cc64 {}
impl CompressedCapability for Cc64 {
    type Length = Length;
    type Offset = Offset;
    type Addr = Addr;
    
    /// ```_CC_N(OTYPE_UNSEALED) = (_CC_N(MAX_REPRESENTABLE_OTYPE) - 0u),
    /// _CC_N(OTYPE_UNSEALED_SIGNED) = (((int64_t)-1) - 0u)```
    /// The OTYPE field is 4 bits (50:47) in CC64
    const MAX_REPRESENTABLE_OTYPE: u32 = 0b1111;
    const OTYPE_UNSEALED:  u32 = Self::MAX_REPRESENTABLE_OTYPE - 0;
    const OTYPE_SENTRY:    u32 = Self::MAX_REPRESENTABLE_OTYPE - 1;
    const OTYPE_RESERVED2: u32 = Self::MAX_REPRESENTABLE_OTYPE - 2;
    const OTYPE_RESERVED3: u32 = Self::MAX_REPRESENTABLE_OTYPE - 3;

    fn compress_raw(cap: &Cap) -> Addr {
        unsafe { cc64_compress_raw(cap) }
    }
    fn decompress_raw(pesbt: Addr, cursor: Addr, tag: bool) -> Cap {
        let mut cap = Default::default();
        unsafe { cc64_decompress_raw(pesbt, cursor, tag, &mut cap) };
        cap
    }
    fn compress_mem(cap: &Cap) -> Addr {
        unsafe { cc64_compress_mem(cap) }
    }
    fn decompress_mem(pesbt: Addr, cursor: Addr, tag: bool) -> Cap {
        let mut cap = Default::default();
        unsafe { cc64_decompress_mem(pesbt, cursor, tag, &mut cap) };
        cap
    }

    /* Getters */
    fn get_uperms(cap: &Cap) -> u32 {
        unsafe { cc64_get_uperms(cap) }
    }
    fn get_perms(cap: &Cap) -> u32 {
        unsafe { cc64_get_perms(cap) }
    }
    fn get_otype(cap: &Cap) -> u32 {
        unsafe { cc64_get_otype(cap) }
    }
    fn get_reserved(cap: &Cap) -> u8 {
        unsafe { cc64_get_reserved(cap) }
    }
    fn get_flags(cap: &Cap) -> u8 {
        unsafe { cc64_get_flags(cap) }
    }

    /* Updaters */
    fn update_uperms(cap: &mut Cap, value: u32) {
        unsafe { cc64_update_uperms(cap, value as Self::Addr) }
    }
    fn update_perms(cap: &mut Cap, value: u32) {
        unsafe { cc64_update_perms(cap, value as Self::Addr) }
    }
    fn update_otype(cap: &mut Cap, value: u32) {
        unsafe { cc64_update_otype(cap, value as Self::Addr) }
    }
    fn update_reserved(cap: &mut Cap, value: u8) {
        unsafe { cc64_update_reserved(cap, value as Self::Addr) }
    }
    fn update_flags(cap: &mut Cap, value: u8) {
        unsafe { cc64_update_flags(cap, value as Self::Addr) }
    }

    /* Misc */
    fn extract_bounds_bits(pesbt: Self::Addr) -> CcxBoundsBits {
        unsafe { cc64_extract_bounds_bits(pesbt) }
    }
    fn set_bounds(cap: &mut Cap, req_base: Self::Addr, req_top: Self::Addr) -> bool {
        unsafe { cc64_setbounds(cap, req_base, req_top) }
    }
    fn is_representable_cap_exact(cap: &Cap) -> bool {
        unsafe { cc64_is_representable_cap_exact(cap) }
    }
    fn make_max_perms_cap(base: Self::Addr, cursor: Self::Addr, top: Self::Addr) -> Cap {
        unsafe { cc64_make_max_perms_cap(base, cursor, top) }
    }
    fn get_representable_length(length: Self::Addr) -> Self::Addr {
        unsafe { cc64_get_representable_length(length) }
    }
    fn get_required_alignment(length: Self::Addr) -> Self::Addr {
        unsafe { cc64_get_required_alignment(length) }
    }
    fn get_alignment_mask(length: Self::Addr) -> Self::Addr {
        unsafe { cc64_get_alignment_mask(length) }
    }
}