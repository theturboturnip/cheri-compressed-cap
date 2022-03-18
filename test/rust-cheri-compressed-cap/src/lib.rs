use std::fmt::Debug;
use num_traits::{Num, One};

/// Trait that the field types defined in CompressedCapability (Length, Offset, Addr) have to implement.
/// This asserts that a) they're numeric, b) they support Default/Copy/Clone/Debug so that CcxCap can derive these.
pub trait NumType: Default + Num + Copy + Clone + Debug + PartialOrd + Ord {}
impl NumType for u32 {}
impl NumType for u64 {}
impl NumType for u128 {}
impl NumType for i32 {}
impl NumType for i64 {}
impl NumType for i128 {}

/// Value which can be converted to T (a NumType).
/// Must also be Default/Copy/Clone/Debug, so CcxCap can derive these.
pub trait FfiNumType<T>: Default + Copy + Clone + Debug + Into<T> + From<T> {}
impl FfiNumType<u64> for u64 {}
impl FfiNumType<i64> for i64 {}

/// Trait defining the public API for a specific capability type.
/// A type X implementing CompressedCapability is equivalent to the API provided by `cheri_compressed_cap_X.h` in C,
/// where `ccx_cap_t` is equivalent to `CcxCap<X>`.
/// 
/// See README.md for description of trait functions
pub trait CompressedCapability: Sized + Copy + Clone {
    /// ccx_length_t Rust-land equivalent - should be a superset of Addr
    type Length: NumType + From<Self::Addr>;
    /// ccx_offset_t Rust-land equivalent - should be a superset of Addr
    type Offset: NumType + From<Self::Addr>;
    /// ccx_addr_t equivalent
    type Addr: NumType + Into<Self::Offset> + Into<Self::Length>;

    /// ccx_length_t C-land equivalent - should have a memory layout identical to the C ccx_length_t.
    /// This is separate from Length because for 128-bit types the Rust and C versions may not look the same.
    /// e.g. Rust u128 is not FFI-safe, the C version uses GCC extension 128-bit values
    type FfiLength: FfiNumType<Self::Length>;
    /// ccx_offset_t C-land equivalent - should have a memory layout identical to the C ccx_offset_t.
    /// See [FfiLength] for an explanation.
    type FfiOffset: FfiNumType<Self::Offset>;

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
    const MAX_UNRESERVED_OTYPE: u32;

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
    fn set_bounds(cap: &mut CcxCap<Self>, req_base: Self::Addr, req_top: Self::Length) -> bool;
    fn is_representable_cap_exact(cap: &CcxCap<Self>) -> bool;
    fn is_representable_new_addr(sealed: bool, base: Self::Addr, length: Self::Length, cursor: Self::Addr, new_cursor: Self::Addr) -> bool;
    fn make_max_perms_cap(base: Self::Addr, cursor: Self::Addr, top: Self::Length) -> CcxCap<Self>;
    fn get_representable_length(length: Self::Length) -> Self::Length;
    fn get_required_alignment(length: Self::Length) -> Self::Length;
    fn get_alignment_mask(length: Self::Length) -> Self::Length;
}

// TODO - Assuming _CC_REVERSE_PESBT_CURSOR_ORDER is *not* set
#[repr(C)]
#[derive(Debug,Copy,Clone)]
pub struct CcxCap<T: CompressedCapability> {
    _cr_cursor: T::Addr,
    cr_pesbt: T::Addr,
    /// _cr_top is stored in memory in a C-compatible way, then converted to the Rust-y version when we manipulate it in Rust
    _cr_top: T::FfiLength,
    cr_base: T::Addr,
    cr_tag: u8,
    cr_bounds_valid: u8,
    cr_exp: u8,
    cr_extra: u8,
}
/// Implements the C++-only member functions
/// 
/// TODO: Decide if this API is opinionated, or just for setting/getting fields.
/// Would be best to have a separate `SafeCap` trait?
impl<T: CompressedCapability> CcxCap<T> {
    pub fn tag(&self) -> bool {
        // cr_tag is interpreted as a boolean with C rules
        self.cr_tag != 0
    }
    pub fn set_tag(&mut self, tag: bool) {
        self.cr_tag = if tag { 1 } else { 0 };
    }

    pub fn base(&self) -> T::Addr {
        self.cr_base
    }
    pub fn top(&self) -> T::Length {
        self._cr_top.into()
    }
    pub fn bounds(&self) -> (T::Addr, T::Length) {
        (self.base(), self.top())
    }
    pub fn set_bounds_unchecked(&mut self, req_base: T::Addr, req_top: T::Length) -> bool {
        T::set_bounds(self, req_base, req_top)
    }
    pub fn set_bounds(&mut self, req_base: T::Addr, req_top: T::Length) -> Result<(),()> {
        if T::set_bounds(self, req_base, req_top) {
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn address(&self) -> T::Addr {
        self._cr_cursor
    }
    pub fn set_address_unchecked(&mut self, addr: T::Addr) {
        self._cr_cursor = addr;
    }
    /// TODO this function has no justification for existing, remove
    pub fn set_address_checked(&mut self, addr: T::Addr) -> Result<(),()> {
        // If addr < base or addr > top + 1, capability would be invalid.
        if addr < self.base() || 
            ((self.top() + T::Length::one()) < addr.into()) {
            return Err(())
        }
        self._cr_cursor = addr;
        Ok(())
    }

    pub fn offset(&self) -> T::Offset {
        let cursor: T::Offset = self._cr_cursor.into();
        let base: T::Offset = self.cr_base.into();
        (cursor - base).into()
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
    pub fn set_software_permissions(&mut self, uperms: u32) {
        T::update_uperms(self, uperms)
    }

    pub fn permissions(&self) -> u32 {
        T::get_perms(self)
    }
    pub fn set_permissions(&mut self, perms: u32) {
        T::update_perms(self, perms)
    }

    pub fn otype(&self) -> u32 {
        T::get_otype(self)
    }
    pub fn is_sealed(&self) -> bool {
        self.otype() != T::OTYPE_UNSEALED
    }
    pub fn set_otype(&mut self, otype: u32) {
        T::update_otype(self, otype)
    }

    pub fn reserved_bits(&self) -> u8 {
        T::get_reserved(self)
    }
    pub fn set_reserved_bits(&mut self, bits: u8) {
        T::update_reserved(self, bits)
    }

    pub fn flags(&self) -> u8 {
        T::get_flags(self)
    }
    pub fn set_flags(&mut self, flags: u8) {
        T::update_flags(self, flags)
    }

    pub fn is_exact(&self) -> bool {
        T::is_representable_cap_exact(self)
    }
    pub fn is_representable_with_new_addr(&self, new_addr: T::Addr) -> bool {
        T::is_representable_new_addr(self.is_sealed(), self.base(), self.length(), self.address(), new_addr) 
    }

    /// Check if an arbitrary object's address range is in this capability's bounds.
    pub fn addr_in_bounds(&self, addr: T::Addr, obj_size: T::Addr) -> bool {
        addr < self.base() || 
            T::Length::from(addr + obj_size - T::Addr::one()) > self.top()
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
impl<T: CompressedCapability> Eq for CcxCap<T> {}
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

mod c_funcs;

mod ffi_num;
pub use ffi_num::{FfiU128,FfiI128};

// Include cc64 definitions
mod cc64;
// Export the CC64 instance of CompressedCapability, and the associated CcxCap type
pub use cc64::{Cc64,Cc64Cap};

// Include cc128 definitions
mod cc128;
// Export the CC128 instance of CompressedCapability, and the associated CcxCap type
pub use cc128::{Cc128,Cc128Cap};

mod wrappers;
// Export 64 and 128-bit instances of CheriRVFuncs
pub use wrappers::CheriRVFuncs;

#[cfg(test)]
mod tests {
    use crate::CompressedCapability;
    // TODO port some tests from the C tests?
    #[test]
    fn test_printing() {
        let cap = crate::Cc64::decompress_raw(0, 0, false);
        println!("{:?}", cap);

        let cap = crate::Cc128::decompress_raw(0, 0, false);
        println!("{:?}", cap);
    }

    #[test]
    fn test_cc128_u128_conversion() {
        // Generate a capability between 0, 0x1_0000_0000_0000_0000 with the current cursor/address at 0x100
        // When we get the 128-bit top(), it should be the same as the one we specified.
        // If it isn't, the memory representation of FfiU128 and the C u128 may be different

        let base: u64 = 0x1000_0000_0000;
        let top: u128 = 0x2000_0000_0000;
        let cap = crate::Cc128::make_max_perms_cap(base, base, top.into());
        assert_eq!(cap.top(), top);
        assert_eq!(cap._cr_top, top.into());
        // cr_base is stored directly after _cr_top, so if the sizes for FfiU128 and C u128 are different it will have been overwritten
        assert_eq!(cap.cr_base, base);
    }
}