use std::ops::{Add,Sub};

/// Trait defining the public API for a specific capability type.
/// A type X implementing CompressedCapability is equivalent to the API provided by `cheri_compressed_cap_X.h` in C,
/// where `ccx_cap_t` is equivalent to `CcxCap<X>`.
/// 
/// See README.md for description of trait functions
pub trait CompressedCapability: Sized {
    /// ccx_length_t equivalent
    type Length: Default + Copy + Add + Sub + PartialEq;
    /// ccx_offset_t equivalent
    type Offset: Default + Copy + Add + Sub + PartialEq;
    /// ccx_addr_t equivalent
    type Addr: Default + Copy + Into<Self::Offset> + Into<Self::Length> + Add + Sub + PartialEq;

    /// CCX_PERM_GLOBAL equivalent
    /// These are the same for 64 and 128bit, but should be overridden for Morello-128
    const PERM_GLOBAL: u32 = (1 << 0);
    const PERM_EXECUTE: u32 = (1 << 1);
    const PERM_LOAD: u32 = (1 << 2);
    const PERM_STORE: u32 = (1 << 3);
    const PERM_LOAD_CAP: u32 = (1 << 4);
    const PERM_STORE_CAP: u32 = (1 << 5);
    const PERM_STORE_LOCAL: u32 = (1 << 6);
    const PERM_SEAL: u32 = (1 << 7);
    const PERM_CINVOKE: u32 = (1 << 8);
    const PERM_UNSEAL: u32 = (1 << 9);
    const PERM_ACCESS_SYS_REGS: u32 = (1 << 10);
    const PERM_SETCID: u32 = (1 << 11);

    const MAX_REPRESENTABLE_OTYPE: u32;
    /// CCX_OTYPE_UNSEALED equivalent
    const OTYPE_UNSEALED: u32;
    const OTYPE_SENTRY: u32;
    const OTYPE_RESERVED2: u32;
    const OTYPE_RESERVED3: u32;

    // Adapted, Rust-safe version of the C API
    // Should be defined by building a wrapper around a linked C function
    fn compress_raw(src_cap: &CcxCap<Self>) -> Self::Addr;
    fn decompress_raw(pesbt: Self::Addr, cursor: Self::Addr, tag: bool) -> CcxCap<Self>;
    fn compress_mem(src_cap: &CcxCap<Self>) -> Self::Addr;
    fn decompress_mem(pesbt: Self::Addr, cursor: Self::Addr, tag: bool) -> CcxCap<Self>;

    /* Getters */
    fn get_uperms(cap: &CcxCap<Self>) -> u32;
    fn get_perms(cap: &CcxCap<Self>) -> u32;
    fn get_otype(cap: &CcxCap<Self>) -> u32;
    fn get_reserved(cap: &CcxCap<Self>) -> u8;
    fn get_flags(cap: &CcxCap<Self>) -> u8;

    /*
    Updaters
    
    The C API updaters all use Self::Addr for the type of `value`. 
    I've changed these to use the types from corresponding `get` functions.
    */
    fn update_uperms(cap: &mut CcxCap<Self>, value: u32);
    fn update_perms(cap: &mut CcxCap<Self>, value: u32);
    fn update_otype(cap: &mut CcxCap<Self>, value: u32);
    fn update_reserved(cap: &mut CcxCap<Self>, value: u8);
    fn update_flags(cap: &mut CcxCap<Self>, value: u8);

    /* Misc */
    fn extract_bounds_bits(pesbt: Self::Addr) -> CcxBoundsBits;
    fn set_bounds(cap: &mut CcxCap<Self>, req_base: Self::Addr, req_top: Self::Addr) -> bool;
    fn is_representable_cap_exact(cap: &CcxCap<Self>) -> bool;
    fn make_max_perms_cap(base: Self::Addr, cursor: Self::Addr, top: Self::Addr) -> CcxCap<Self>;
    fn get_representable_length(length: Self::Addr) -> Self::Addr;
    fn get_required_alignment(length: Self::Addr) -> Self::Addr;
    fn get_alignment_mask(length: Self::Addr) -> Self::Addr;
}

// TODO - Assuming _CC_REVERSE_PESBT_CURSOR_ORDER is *not* set
#[repr(C)]
#[derive(Debug,Copy,Clone)]
pub struct CcxCap<T: CompressedCapability> {
    _cr_cursor: T::Addr,
    cr_pesbt: T::Addr,
    _cr_top: T::Length,
    cr_base: T::Addr,
    cr_tag: u8,
    cr_bounds_valid: u8,
    cr_exp: u8,
    cr_extra: u8,
}
/// Implements the C++-only member functions
impl<T: CompressedCapability> CcxCap<T> 
    // This is an annoyingly long where-clause which states (T::Offset - T::Offset) is of type T::Offset,
    // and the same for T::Length.
    // These are required because our member functions need to evaluate these subtractions,
    // but Rust doesn't know what Offset or Length are ahead of time.
    where <T::Offset as Sub>::Output: Into<T::Offset>, 
            <T::Length as Sub>::Output: Into<T::Length> {

    pub fn base(&self) -> T::Addr {
        self.cr_base
    }
    pub fn address(&self) -> T::Addr {
        self._cr_cursor
    }
    pub fn offset(&self) -> T::Offset {
        let cursor: T::Offset = self._cr_cursor.into();
        let base: T::Offset = self.cr_base.into();
        (cursor - base).into()
    }
    pub fn top(&self) -> T::Length {
        self._cr_top
    }
    // TODO top64
    pub fn length(&self) -> T::Length {
        let top: T::Length = self._cr_top.into();
        let base: T::Length = self.cr_base.into();
        (top - base).into()
    }
    // TODO length64
    pub fn software_permissions(&self) -> u32 {
        T::get_uperms(self)
    }
    pub fn permissions(&self) -> u32 {
        T::get_perms(self)
    }
    pub fn otype(&self) -> u32 {
        T::get_otype(self)
    }
    pub fn is_sealed(&self) -> bool {
        self.otype() != T::OTYPE_UNSEALED
    }
    pub fn reserved_bits(&self) -> u8 {
        T::get_reserved(self)
    }
    pub fn flags(&self) -> u8 {
        T::get_flags(self)
    }
}
/// Implements the `operator==` from cheri_compressed_cap_common.h
impl<T: CompressedCapability> PartialEq for CcxCap<T> {
    fn eq(&self, other: &Self) -> bool {
        self.cr_tag == other.cr_tag && 
        self._cr_cursor == other._cr_cursor && 
        self.cr_pesbt == other.cr_pesbt
    }
}
/// Equivalent to initialization pattern used in tests
/// ```ccx_cap_t value;
/// memset(&value, 0, sizeof(value));```
/// 
/// cc64.rs didn't pick it up when it was automatically #[derive]-d, so it's manually implemented here
impl<T: CompressedCapability> Default for CcxCap<T> {
    fn default() -> Self {
        CcxCap {
            // Use Default::default for the associated types (Addr, Length, Offset)
            // Rust doesn't have enough information to know they are numbers
            _cr_cursor: Default::default(),
            cr_pesbt: Default::default(),
            _cr_top: Default::default(),
            cr_base: Default::default(),
            cr_tag: 0,
            cr_bounds_valid: 0,
            cr_exp: 0,
            cr_extra: 0,
        }
    }
}

#[repr(C)]
pub struct CcxBoundsBits {
    b: u16,
    t: u16,
    e: u8,
    ie: bool,
}

// Include cc64 definitions
mod cc64;
// Export the CC64 instance of CompressedCapability, and the associated CcxCap type
pub use cc64::{Cc64,Cc64Cap};

#[cfg(test)]
mod tests {
    // TODO port some tests from the C tests?
}