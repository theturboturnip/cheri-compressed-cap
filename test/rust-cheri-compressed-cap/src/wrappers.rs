
use std::convert::TryInto;
use crate::CcxCap;
use crate::CompressedCapability;

/// Trait exposing the utility functions used to specify CHERI-RISC-V behaviour in Tech Report 951
/// Behaviour is derived from [the Sail specification](https://github.com/CTSRD-CHERI/sail-cheri-riscv)
pub trait CheriRVFuncs<T: CompressedCapability> {
    type Cap;

    type CapAddrInt;
    type CapAddrBits;
    type CapLen;

    type OType;
    type Perms;
    type Flags;

    fn getCapBounds(c: &Self::Cap) -> (Self::CapAddrInt, Self::CapLen);
    fn getCapBaseBits(c: &Self::Cap) -> Self::CapAddrBits;
    fn getCapTop(c: &Self::Cap) -> Self::CapLen;
    fn getCapLength(c: &Self::Cap) -> Self::CapLen;
    fn inCapBounds(c: &Self::Cap, addr: Self::CapAddrBits, size: Self::CapLen) -> bool;
    fn getCapCursor(c: &Self::Cap) -> Self::CapAddrInt;
    fn getCapOffsetBits(c: &Self::Cap) -> Self::CapAddrBits;

    // These can return (false, cap); `cap` may not preserve bounds (???)
    fn setCapBounds(c: &Self::Cap, base: Self::CapAddrBits, top: Self::CapLen) -> (bool, Self::Cap);
    fn setCapAddr(c: &Self::Cap, addr: Self::CapAddrBits) -> (bool, Self::Cap);
    fn setCapOffset(c: &Self::Cap, offset: Self::CapAddrBits) -> (bool, Self::Cap);
    fn incCapOffset(c: &Self::Cap, offset_inc: Self::CapAddrBits) -> (bool, Self::Cap);

    fn getRepresentableAlignmentMask(val: Self::CapLen) -> Self::CapLen;
    fn getRepresentableLength(val: Self::CapLen) -> Self::CapLen;

    fn sealCap(c: &Self::Cap, otype: Self::OType) -> Self::Cap;
    fn unsealCap(c: &Self::Cap) -> Self::Cap;
    fn isCapSealed(c: &Self::Cap) -> bool;
    fn hasReservedOType(c: &Self::Cap) -> bool;
    fn invalidateCap(c: &Self::Cap) -> Self::Cap;

    fn getCapPerms(c: &Self::Cap) -> Self::Perms;
    fn setCapPerms(c: &Self::Cap, perms: Self::Perms) -> Self::Cap;
    fn getCapFlags(c: &Self::Cap) -> Self::Flags;
    fn setCapFlags(c: &Self::Cap, flags: Self::Flags) -> Self::Cap;
}
impl<T: CompressedCapability> CheriRVFuncs<T> for T where T::Offset: TryInto<T::Addr> {
    type Cap = CcxCap<T>;

    type CapAddrInt = T::Addr;
    type CapAddrBits = T::Addr;
    type CapLen = T::Length;

    type OType = u32;
    type Perms = u32;
    type Flags = u8;

    fn getCapBounds(c: &Self::Cap) -> (Self::CapAddrInt, Self::CapLen) {
        (c.base(), c.top())
    }
    fn getCapBaseBits(c: &Self::Cap) -> Self::CapAddrBits {
        c.base()
    }
    fn getCapTop(c: &Self::Cap) -> Self::CapLen {
        c.top()
    }
    fn getCapLength(c: &Self::Cap) -> Self::CapLen {
        c.length()
    }
    fn inCapBounds(c: &Self::Cap, addr: Self::CapAddrBits, size: Self::CapLen) -> bool {
        addr >= c.base() && (size + addr.into()) <= c.top()
    }
    fn getCapCursor(c: &Self::Cap) -> Self::CapAddrInt {
        c.address()
    }
    fn getCapOffsetBits(c: &Self::Cap) -> Self::CapAddrBits {
        match c.offset().try_into() {
            Ok(val) => val,
            Err(_) => panic!("getCapOffsetBits can't convert to address")
        }
    }

    // These can return (false, cap); `cap` may not preserve bounds (???)
    fn setCapBounds(c: &Self::Cap, base: Self::CapAddrBits, top: Self::CapLen) -> (bool, Self::Cap) {
        let c = *c;
        c.set_bounds(base, top);
        (c.is_exact(), c)
    }
    fn setCapAddr(c_old: &Self::Cap, addr: Self::CapAddrBits) -> (bool, Self::Cap) {
        let c = *c_old;
        c.set_address_unchecked(addr);
        todo!("Sail computes the bounds on each check, relative to the address(?). 
        Thus, checking if the bounds have changed = checking if the capability is representable
        how should we check this in Rust?")
        (c.bounds() == c_old.bounds(), c)
    }
    fn setCapOffset(c: &Self::Cap, offset: Self::CapAddrBits) -> (bool, Self::Cap) {
        todo!("Sail uses a 'fastRepCheck' function to see if this is representable - can we use .is_exact() for this?");
        Self::setCapAddr(c, c.base() + offset)
    }
    fn incCapOffset(c: &Self::Cap, offset_inc: Self::CapAddrBits) -> (bool, Self::Cap) {
        todo!("Sail uses a 'fastRepCheck' function to see if this is representable - can we use .is_exact() for this?");
        Self::setCapAddr(c, c.address() + offset_inc)
    }

    fn getRepresentableAlignmentMask(val: Self::CapLen) -> Self::CapLen {
        todo!("mismatch between Sail and C library - in Sail getRepresentableAlignmentMask takes and returns Length, not Address");
        // T::get_alignment_mask(val)

        
    }
    fn getRepresentableLength(val: Self::CapLen) -> Self::CapLen {
        todo!("mismatch between Sail and C library - in Sail getRepresentableLength takes and returns Length, not Address");
        // T::get_representable_length(val)
    }

    fn sealCap(c: &Self::Cap, otype: Self::OType) -> Self::Cap {
        assert!(otype != T::OTYPE_UNSEALED);
        // Set otype to whatever we asked for
        let c = *c;
        c.set_otype(otype);
        c
    }
    fn unsealCap(c: &Self::Cap) -> Self::Cap {
        // Just set otype = UNSEALED
        let c = *c;
        c.set_otype(T::OTYPE_UNSEALED);
        c
    }
    fn isCapSealed(c: &Self::Cap) -> bool {
        c.is_sealed()
    }
    fn hasReservedOType(c: &Self::Cap) -> bool {
        c.otype() > T::MAX_UNRESERVED_OTYPE
    }
    fn invalidateCap(c: &Self::Cap) -> Self::Cap {
        let c = *c;
        c.set_tag(false);
        c
    }

    fn getCapPerms(c: &Self::Cap) -> Self::Perms {
        c.permissions()
    }
    fn setCapPerms(c: &Self::Cap, perms: Self::Perms) -> Self::Cap {
        // Deref (i.e. make a copy)
        let c = *c;
        c.set_permissions(perms);
        c
    }
    fn getCapFlags(c: &Self::Cap) -> Self::Flags {
        c.flags()
    }
    fn setCapFlags(c: &Self::Cap, flags: Self::Flags) -> Self::Cap {
        // Deref (i.e. make a copy)
        let c = *c;
        c.set_flags(flags);
        c
    }
}