macro_rules! ccap_impl_funcs {
    ($ver:ident) => { paste! {
        fn compress_raw(cap: &Cap) -> Addr {
            unsafe { [<$ver _compress_raw>](cap) }
        }
        fn decompress_raw(pesbt: Addr, cursor: Addr, tag: bool) -> Cap {
            let mut cap = Default::default();
            unsafe { [<$ver _decompress_raw>](pesbt, cursor, tag, &mut cap) };
            cap
        }
        fn compress_mem(cap: &Cap) -> Addr {
            unsafe { [<$ver _compress_mem>](cap) }
        }
        fn decompress_mem(pesbt: Addr, cursor: Addr, tag: bool) -> Cap {
            let mut cap = Default::default();
            unsafe { [<$ver _decompress_mem>](pesbt, cursor, tag, &mut cap) };
            cap
        }

        /* Getters */
        fn get_uperms(cap: &Cap) -> u32 {
            unsafe { [<$ver _get_uperms>](cap) }
        }
        fn get_perms(cap: &Cap) -> u32 {
            unsafe { [<$ver _get_perms>](cap) }
        }
        fn get_otype(cap: &Cap) -> u32 {
            unsafe { [<$ver _get_otype>](cap) }
        }
        fn get_reserved(cap: &Cap) -> u8 {
            unsafe { [<$ver _get_reserved>](cap) }
        }
        fn get_flags(cap: &Cap) -> u8 {
            unsafe { [<$ver _get_flags>](cap) }
        }

        /* Updaters */
        fn update_uperms(cap: &mut Cap, value: u32) {
            unsafe { [<$ver _update_uperms>](cap, value as Self::Addr) }
        }
        fn update_perms(cap: &mut Cap, value: u32) {
            unsafe { [<$ver _update_perms>](cap, value as Self::Addr) }
        }
        fn update_otype(cap: &mut Cap, value: u32) {
            unsafe { [<$ver _update_otype>](cap, value as Self::Addr) }
        }
        fn update_reserved(cap: &mut Cap, value: u8) {
            unsafe { [<$ver _update_reserved>](cap, value as Self::Addr) }
        }
        fn update_flags(cap: &mut Cap, value: u8) {
            unsafe { [<$ver _update_flags>](cap, value as Self::Addr) }
        }

        /* Misc */
        fn extract_bounds_bits(pesbt: Self::Addr) -> CcxBoundsBits {
            unsafe { [<$ver _extract_bounds_bits>](pesbt) }
        }
        fn set_bounds(cap: &mut Cap, req_len: Self::Length) -> bool {
            unsafe { [<$ver _setbounds>](cap, req_len) }
        }
        fn is_representable_cap_exact(cap: &Cap) -> bool {
            unsafe { [<$ver _is_representable_cap_exact>](cap) }
        }
        fn is_representable_new_addr(sealed: bool, base: Self::Addr, length: Self::Length, cursor: Self::Addr, new_cursor: Self::Addr) -> bool {
            unsafe { [<$ver _is_representable_new_addr>](sealed, base, length, cursor, new_cursor) }
        }
        fn make_max_perms_cap(base: Self::Addr, cursor: Self::Addr, top: Self::Length) -> Cap {
            unsafe { [<$ver _make_max_perms_cap>](base, cursor, top) }
        }
        fn get_representable_length(length: Self::Length) -> Self::Length {
            unsafe { [<$ver _get_representable_length>](length) }
        }
        fn get_required_alignment(length: Self::Length) -> Self::Length {
            unsafe { [<$ver _get_required_alignment>](length) }
        }
        fn get_alignment_mask(length: Self::Length) -> Self::Length {
            unsafe { [<$ver _get_alignment_mask>](length) }
        }
    } }
}


pub mod morello {
    use crate::c_funcs::*;
    use crate::{CcxBoundsBits, CcxCap, CompressedCapability};
    use paste::paste;

    pub type Length = u128;
    pub type Offset = i128;
    pub type FfiLength = u128;
    pub type FfiOffset = i128;
    pub type Addr = u64;

    pub type Cap = CcxCap<Cc128>;
    pub type Cc128Cap = Cap;

    /// Implements [CompressedCapability] for Morello 128-bit capabilities.
    ///
    /// Empty enum, so it cannot be itself constructed. If you want a CC128 capability, instantiate Cc128Cap.
    ///
    /// Derives Debug, Copy, Clone so that CcxCap<Cc128> can derive them too.
    #[derive(Debug, Copy, Clone)]
    pub enum Cc128 {}
    impl CompressedCapability for Cc128 {
        type Length = Length;
        type Offset = Offset;
        type Addr = Addr;

        type FfiLength = FfiLength;
        type FfiOffset = FfiOffset;

        const PERM_GLOBAL: u32 = (1 << 0);
        const PERM_EXECUTE: u32 = (1 << 15);
        const PERM_LOAD: u32 = (1 << 17);
        const PERM_STORE: u32 = (1 << 16);
        const PERM_LOAD_CAP: u32 = (1 << 14);
        const PERM_STORE_CAP: u32 = (1 << 13);
        const PERM_STORE_LOCAL: u32 = (1 << 12);
        const PERM_SEAL: u32 = (1 << 11);
        const PERM_CINVOKE: u32 = (1 << 8);
        const PERM_UNSEAL: u32 = (1 << 10);
        const PERM_ACCESS_SYS_REGS: u32 = (1 << 9);
        const PERM_SETCID: u32 = (1 << 7);

        /// _128m.h:171
        /// encoded directly _128m.h:78
        const MAX_REPRESENTABLE_OTYPE: u32 = ((1u32 << 15)) - 1;
        const OTYPE_UNSEALED: u32 = 0;
        const OTYPE_SENTRY: u32 = 1;
        const OTYPE_RESERVED2: u32 = 2;
        const OTYPE_RESERVED3: u32 = 3;
        const MAX_UNRESERVED_OTYPE: u32 = Self::MAX_REPRESENTABLE_OTYPE;

        ccap_impl_funcs!(cc128m);
    }
}

pub mod cheriv9 {
    //! CHERIv9 Implementations

    pub mod cc64 {
        use crate::c_funcs::*;
        use crate::{CcxBoundsBits, CcxCap, CompressedCapability};
        use paste::paste;

        pub type Length = u64;
        pub type Offset = i64;
        pub type FfiLength = u64;
        pub type FfiOffset = i64;
        pub type Addr = u32;

        pub type Cap = CcxCap<Cc64>;
        pub type Cc64Cap = Cap;

        /// Implements [CompressedCapability] for CHERIv9 64-bit capabilities.
        ///
        /// Empty enum, so it cannot be itself constructed. If you want a CC64 capability, instantiate CC64::Cap.
        ///
        /// Derives Debug, Copy, Clone so that CcxCap<Cc64> can derive them too.
        #[derive(Debug, Copy, Clone)]
        pub enum Cc64 {}
        impl CompressedCapability for Cc64 {
            type Length = Length;
            type Offset = Offset;
            type Addr = Addr;

            type FfiLength = FfiLength;
            type FfiOffset = FfiOffset;

            /// _64.h:126
            const MAX_REPRESENTABLE_OTYPE: u32 = 0b1111;
            const OTYPE_UNSEALED: u32 = Self::MAX_REPRESENTABLE_OTYPE - 0;
            const OTYPE_SENTRY: u32 = Self::MAX_REPRESENTABLE_OTYPE - 1;
            const OTYPE_RESERVED2: u32 = Self::MAX_REPRESENTABLE_OTYPE - 2;
            const OTYPE_RESERVED3: u32 = Self::MAX_REPRESENTABLE_OTYPE - 3;
            const MAX_UNRESERVED_OTYPE: u32 = Self::MAX_REPRESENTABLE_OTYPE;

            ccap_impl_funcs! {cc64}
        }
    }

    pub mod cc128 {
        use crate::c_funcs::*;
        use crate::{CcxBoundsBits, CcxCap, CompressedCapability};
        use paste::paste;

        pub type Length = u128;
        pub type Offset = i128;
        pub type FfiLength = u128;
        pub type FfiOffset = i128;
        pub type Addr = u64;

        pub type Cap = CcxCap<Cc128>;
        pub type Cc128Cap = Cap;

        /// Implements [CompressedCapability] for CHERIv9 128-bit capabilities.
        ///
        /// Empty enum, so it cannot be itself constructed. If you want a CC128 capability, instantiate Cc128Cap.
        ///
        /// Derives Debug, Copy, Clone so that CcxCap<Cc128> can derive them too.
        #[derive(Debug, Copy, Clone)]
        pub enum Cc128 {}
        impl CompressedCapability for Cc128 {
            type Length = Length;
            type Offset = Offset;
            type Addr = Addr;

            type FfiLength = FfiLength;
            type FfiOffset = FfiOffset;

            /// The OTYPE field is 18 bits (108:91) in CC128
            /// _128.h:126
            const MAX_REPRESENTABLE_OTYPE: u32 = 0x3_FFFF;
            const OTYPE_UNSEALED: u32 = Self::MAX_REPRESENTABLE_OTYPE - 0;
            const OTYPE_SENTRY: u32 = Self::MAX_REPRESENTABLE_OTYPE - 1;
            const OTYPE_RESERVED2: u32 = Self::MAX_REPRESENTABLE_OTYPE - 2;
            const OTYPE_RESERVED3: u32 = Self::MAX_REPRESENTABLE_OTYPE - 3;
            const MAX_UNRESERVED_OTYPE: u32 = Self::MAX_REPRESENTABLE_OTYPE - 4;

            ccap_impl_funcs!(cc128);
        }
    }
}

pub mod rvy {
    //! RISC-V Y extension implementations

    pub mod cc64 {
        use crate::c_funcs::*;
        use crate::{CcxBoundsBits, CcxCap, CompressedCapability};
        use paste::paste;

        pub type Length = u64;
        pub type Offset = i64;
        pub type FfiLength = u64;
        pub type FfiOffset = i64;
        pub type Addr = u32;

        pub type Cap = CcxCap<Cc64>;
        pub type Cc64Cap = Cap;

        /// Implements [CompressedCapability] for RISC-V Y 64-bit capabilities.
        ///
        /// Empty enum, so it cannot be itself constructed. If you want a CC64 capability, instantiate CC64::Cap.
        ///
        /// Derives Debug, Copy, Clone so that CcxCap<Cc64> can derive them too.
        #[derive(Debug, Copy, Clone)]
        pub enum Cc64 {}
        impl CompressedCapability for Cc64 {
            type Length = Length;
            type Offset = Offset;
            type Addr = Addr;

            type FfiLength = FfiLength;
            type FfiOffset = FfiOffset;

            // Lots of permissions are missing
            const PERM_GLOBAL: u32 = 0;
            const PERM_EXECUTE: u32 = (1 << 17);
            const PERM_LOAD: u32 = (1 << 18);
            const PERM_STORE: u32 = (1 << 0);
            const PERM_LOAD_CAP: u32 = 0; // encoded with combo of PERM_CAPABILITY and _LOAD
            const PERM_STORE_CAP: u32 = 0; // encoded with combo of PERM_CAPABILITY and _STORE
            const PERM_STORE_LOCAL: u32 = 0;
            const PERM_SEAL: u32 = 0;
            const PERM_CINVOKE: u32 = 0;
            const PERM_UNSEAL: u32 = 0;
            const PERM_ACCESS_SYS_REGS: u32 = 0;
            const PERM_SETCID: u32 = 0;

            /// This is a single bit in RV32Y but CHERIoT does something different not modelled here
            const MAX_REPRESENTABLE_OTYPE: u32 = 0b1;
            const OTYPE_UNSEALED: u32 = 0;
            const OTYPE_SENTRY: u32 = 1;
            const OTYPE_RESERVED2: u32 = u32::MAX;
            const OTYPE_RESERVED3: u32 = u32::MAX;
            const MAX_UNRESERVED_OTYPE: u32 = u32::MAX;

            ccap_impl_funcs! {cc64r}
        }
    }

    pub mod cc128 {
        use crate::c_funcs::*;
        use crate::{CcxBoundsBits, CcxCap, CompressedCapability};
        use paste::paste;

        pub type Length = u128;
        pub type Offset = i128;
        pub type FfiLength = u128;
        pub type FfiOffset = i128;
        pub type Addr = u64;

        pub type Cap = CcxCap<Cc128>;
        pub type Cc128Cap = Cap;

        /// Implements [CompressedCapability] for RISC-V Y 128-bit capabilities.
        ///
        /// Empty enum, so it cannot be itself constructed. If you want a CC128 capability, instantiate Cc128Cap.
        ///
        /// Derives Debug, Copy, Clone so that CcxCap<Cc128> can derive them too.
        #[derive(Debug, Copy, Clone)]
        pub enum Cc128 {}
        impl CompressedCapability for Cc128 {
            type Length = Length;
            type Offset = Offset;
            type Addr = Addr;

            type FfiLength = FfiLength;
            type FfiOffset = FfiOffset;

            // Lots of permissions are missing
            const PERM_GLOBAL: u32 = 0;
            const PERM_EXECUTE: u32 = (1 << 17);
            const PERM_LOAD: u32 = (1 << 18);
            const PERM_STORE: u32 = (1 << 0);
            const PERM_LOAD_CAP: u32 = 0; // encoded with combo of PERM_CAPABILITY and _LOAD
            const PERM_STORE_CAP: u32 = 0; // encoded with combo of PERM_CAPABILITY and _STORE
            const PERM_STORE_LOCAL: u32 = 0;
            const PERM_SEAL: u32 = 0;
            const PERM_CINVOKE: u32 = 0;
            const PERM_UNSEAL: u32 = 0;
            const PERM_ACCESS_SYS_REGS: u32 = 0;
            const PERM_SETCID: u32 = 0;

            /// This is a single bit in RV64Y but there are reserved bits off the top
            const MAX_REPRESENTABLE_OTYPE: u32 = 0b1;
            const OTYPE_UNSEALED: u32 = 0;
            const OTYPE_SENTRY: u32 = 1;
            const OTYPE_RESERVED2: u32 = u32::MAX;
            const OTYPE_RESERVED3: u32 = u32::MAX;
            const MAX_UNRESERVED_OTYPE: u32 = u32::MAX;

            ccap_impl_funcs!(cc128r);
        }
    }
}
