use crate::CcxBoundsBits;
use crate::cc128;
use crate::cc64;

/// Import C functions for CC128.
/// 
/// We used to do this separately in cc64.rs and cc128.rs for just the 64 and 128 bit versions respectively.
/// This seemed to cause multiple-definition link errors, so now we link all of them in one place.
#[link(name = "cheri_compressed_cap_lib")]
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