//! ---------------------------------------------------------------------------
//! File       : macros.rs
//! Module     : registers::macros
//! Author     : DiTurr
//! Description: Contains macros for reading and manipulating CSR registers.
//! ---------------------------------------------------------------------------

#[macro_export]
macro_rules! define_csr {
    (
        $(#[$doc:meta])*
        $name:ident,
        address: $addr:literal,
        mask: $mask:expr
    ) => {
        $(#[$doc])*
        pub struct $name;

        impl $name {
            #[inline]
            pub fn read() -> usize {
                let value: usize;
                unsafe {
                    core::arch::asm!("csrr {0}, {1}", out(reg) value, const $addr);
                }
                value & $mask
            }
        }
    };
}
