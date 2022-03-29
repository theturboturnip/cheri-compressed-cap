use std::fmt::Debug;
use num_traits::{Num,WrappingAdd};

/// Trait that the field types defined in CompressedCapability (Length, Offset, Addr) have to implement.
/// This asserts that a) they're numeric, b) they support Default/Copy/Clone/Debug so that CcxCap can derive these.
pub trait NumType: Default + Num + WrappingAdd + Copy + Clone + Debug + PartialOrd + Ord {}
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
impl FfiNumType<u128> for u128 {}
impl FfiNumType<i128> for i128 {}

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
    type FfiLength: FfiNumType<Self::Length>;
    /// ccx_offset_t C-land equivalent - should have a memory layout identical to the C ccx_offset_t.
    /// See [Self::FfiLength] for an explanation.
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
    /// Sets the top/bottom fields of the capability, and the PESBT field, to bounds that encompass (req_base, req_top).
    /// Because a floating-point representation is used for bounds, it may not be able to set (req_base, req_top) exactly.
    /// In this case it will return False.
    fn set_bounds(cap: &mut CcxCap<Self>, req_base: Self::Addr, req_top: Self::Length) -> bool;
    fn is_representable_cap_exact(cap: &CcxCap<Self>) -> bool;
    fn is_representable_new_addr(sealed: bool, base: Self::Addr, length: Self::Length, cursor: Self::Addr, new_cursor: Self::Addr) -> bool;
    fn make_max_perms_cap(base: Self::Addr, cursor: Self::Addr, top: Self::Length) -> CcxCap<Self>;
    fn get_representable_length(length: Self::Length) -> Self::Length;
    fn get_required_alignment(length: Self::Length) -> Self::Length;
    fn get_alignment_mask(length: Self::Length) -> Self::Length;
}

#[repr(C, align(16))]
#[derive(Copy,Clone)]
/// Structure matching the C type `_cc_N(cap)`.
/// Field order and layout is binary-compatible with the C version,
/// assuming the C preprocessor macro `_CC_REVERSE_PESBT_CURSOR_ORDER` is *not* defined.
/// 
/// This is a plain-old-data type. It only supplies getters and setters, and does *not* guarantee any safety/correctness.
/// For example, there are no added assertions or checks if you set the cursor to a value outside the bounds.
/// However, the C FFI functions from [CompressedCapability] may have their own asserts.
/// These are documented where possible.
/// 
/// *For a safe interface, use one of the [crate::wrappers]*
pub struct CcxCap<T: CompressedCapability> {
    /// If [Self::cr_tag] is 1, this is the capability's "cursor" i.e. the address it's actually pointing to.
    /// The bottom half of the capability as stored in memory.
    _cr_cursor: T::Addr,
    /// If [Self::cr_tag] is 1, this is the compressed capability metadata (permissions, otype, bounds, etc.)
    /// The top half of the capability as stored in memory.
    cr_pesbt: T::Addr,

    /// The top of this capability's valid address range.
    /// Derived from [Self::cr_pesbt].
    /// As long as [Self::cr_tag] is 1, the getter/setter will ensure it matches.
    _cr_top: T::FfiLength,
    /// The base of this capability's valid address range.
    /// Derived from [Self::cr_pesbt].
    /// As long as [Self::cr_tag] is 1, the getter/setter will ensure it matches.
    cr_base: T::Addr,
    /// Tag - if 1, this is a valid capability, 0 it's just plain data
    cr_tag: u8,
    /// 0 (false) if the bounds decode step was given an invalid capability.
    /// Should be 1 (true) for all non-Morello capabilities. 
    cr_bounds_valid: u8,
    /// The exponent used for storing the bounds.
    /// Stored from various places, only used in Morello-exclusive function cap_bounds_uses_value().
    cr_exp: u8,
    /// "Additional data stored by the caller."
    /// Seemingly completely unused, essentially padding.
    cr_extra: u8,
}

/// Implements getters and setters similar to the C++-only member functions in the header.
impl<T: CompressedCapability> CcxCap<T> {
    /// Returns a `(tag, [cursor, pesbt])` tuple that represents all data required to 
    /// store a capability in a register.
    /// 
    /// To store capabilities in memory, see [Self::mem_representation]
    pub fn reg_representation(&self) -> (bool, [T::Addr; 2]) {
        // This should be equal to self.cr_pesbt, the compress_raw function just returns that.
        // We use this function in case that behaviour changes in the future, and for consistency with mem_representation.
        let compressed_pesbt = T::compress_raw(self);
        (self.tag(), [self._cr_cursor, compressed_pesbt])
    }

    /// Returns a `(tag, [cursor, pesbt])` tuple that represents all data required to 
    /// store a capability in memory
    /// 
    /// To store capabilities in a register, see [Self::reg_representation]
    pub fn mem_representation(&self) -> (bool, [T::Addr; 2]) {
        // This should be equal to (self.cr_pesbt ^ SOME_XOR_MASK)
        let compressed_pesbt = T::compress_mem(self);
        (self.tag(), [self._cr_cursor, compressed_pesbt])
    }

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
    /// Sets the base and top of this capability using C FFI function [CompressedCapability::set_bounds].
    /// Updates the PEBST field correspondingly.
    /// On non-Morello platforms, will fail with an assertion error if [Self::tag()] is not set.
    pub fn set_bounds_unchecked(&mut self, req_base: T::Addr, req_top: T::Length) -> bool {
        T::set_bounds(self, req_base, req_top)
    }

    pub fn address(&self) -> T::Addr {
        self._cr_cursor
    }
    pub fn set_address_unchecked(&mut self, addr: T::Addr) {
        self._cr_cursor = addr;
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

    /// Helper function for easily calling FFI function [CompressedCapability::is_representable_cap_exact]
    /// on this capability.
    /// Assertions are present in the C code, but should never be triggered.
    pub fn is_exact(&self) -> bool {
        T::is_representable_cap_exact(self)
    }
    /// Helper function for easily calling FFI function [CompressedCapability::is_representable_new_addr]
    /// on this capability.
    /// Assertions are present in the C code, but should never be triggered.
    pub fn is_representable_with_new_addr(&self, new_addr: T::Addr) -> bool {
        T::is_representable_new_addr(self.is_sealed(), self.base(), self.length(), self.address(), new_addr) 
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
impl<T: CompressedCapability> Debug for CcxCap<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CcxCap")
            .field("cr_cursor", &self._cr_cursor)
            .field("cr_base", &self.cr_base)
            .field("cr_top", &self._cr_top)
            .field("cr_tag", &self.cr_tag)
            .field("cr_bounds_valid", &self.cr_bounds_valid)
            .field("cr_exp", &self.cr_exp)
            .field("cr_extra", &self.cr_extra)
            .field("permissions", &self.permissions())
            .field("software_permissions", &self.software_permissions())
            .field("otype", &self.otype())
            .field("reserved_bits", &self.reserved_bits())
            .field("flags", &self.flags())
            .finish()
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

// Include cc64 definitions
mod cc64;
// Export the CC64 instance of CompressedCapability, and the associated CcxCap type
pub use cc64::{Cc64,Cc64Cap};

// Include cc128 definitions
mod cc128;
// Export the CC128 instance of CompressedCapability, and the associated CcxCap type
pub use cc128::{Cc128,Cc128Cap};

pub mod wrappers;

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
        dbg!(cap.is_exact());
    }

    #[test]
    fn test_cc128_u128_conversion() {
        // Generate a capability between 0, 0x1_0000_0000_0000_0000 with the current cursor/address at 0x100
        // When we get the 128-bit top(), it should be the same as the one we specified.
        // If it isn't, the memory representation of FfiU128 and the C u128 may be different

        let base: u64 = 0x1000_0000_0000;
        let top: u128 = 0x2000_0000_0000;
        let cap = crate::Cc128::make_max_perms_cap(base, base, top);
        assert_eq!(cap.top(), top);
        assert_eq!(cap._cr_top, top);
        // cr_base is stored directly after _cr_top, so if the sizes for FfiU128 and C u128 are different it will have been overwritten
        assert_eq!(cap.cr_base, base);
    }
}