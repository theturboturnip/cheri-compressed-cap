/*! u128,i128 are not FFI-safe.
 * This module provides structures compatible with the GCC 128-bit integer extension,
 * which can be used to go between Rust u/i128 and C FFI u/i128.
!*/


use crate::FfiNumType;

#[repr(C)]
#[derive(Default,Copy,Clone,Debug,PartialEq,Eq)]
pub struct FfiU128 {
    bottom: u64,
    top: u64,
}
impl From<FfiU128> for u128 {
    fn from(x: FfiU128) -> u128 { 
        ((x.top as u128) << 64) | (x.bottom as u128)
    }
}
impl From<u128> for FfiU128 {
    fn from(x: u128) -> Self {
        let bottom = x as u64;
        let top = (x >> 64) as u64;
        FfiU128 { bottom, top }
    }
}
impl FfiNumType<u128> for FfiU128 {}


#[repr(C)]
#[derive(Default,Copy,Clone,Debug,PartialEq,Eq)]
pub struct FfiI128{
    bottom: u64,
    top: u64,
}
impl From<FfiI128> for i128 {
    fn from(x: FfiI128) -> i128 { 
        let x_as_u128 = ((x.top as u128) << 64) | (x.bottom as u128);
        x_as_u128 as i128
    }
}
impl From<i128> for FfiI128 {
    fn from(x: i128) -> Self {
        // Convert bits to u128, do all bit-operations in unsigned mode
        let x = x as u128;
        let bottom = x as u64;
        let top = (x >> 64) as u64;
        FfiI128 { bottom, top }
    }
}
impl FfiNumType<i128> for FfiI128 {}
