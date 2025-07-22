//! ---------------------------------------------------------------------------
//! File       : macros.rs
//! Module     : registers::macros
//! Author     : DiTurr
//! Description:
//! Contains macros for reading and manipulating CSR registers. This macro simplifies the
//! creation of types for reading Control and Status Registers (CSRs) in RISC-V architectures.
//! It defines a public struct with a static `read()` method, which emits inline assembly to
//! safely access the CSR and apply an optional bitmask.
//!
//! ## Example
//! ```rust
//! define_csr!(
//!     /// Machine Timer Register
//!     MTime,
//!     address: 0xC01,
//!     mask: 0xFFFF_FFFF_FFFF_FFFF
//! );
//!
//! let ticks = MTime::read();
//! ```
//! ---------------------------------------------------------------------------

#[macro_export]
macro_rules! define_csr {
    (
        // Optional outer documentation for the struct
        $(#[$doc:meta])*
        $name:ident,
        address: $addr:literal,
        mask: $mask:expr
    ) => {
        // Apply the outer documentation, if provided
        $(#[$doc])*
        /// Auto-generated CSR accessor struct.
        pub struct $name;
        impl $name {
            /// Reads the value of the CSR at the given address, masked with the provided bitmask.
            ///
            /// # Safety
            /// Uses inline assembly (`csrr`) to read the CSR. Safe to use if the address
            /// and access mode are correct for the target platform.
            ///
            /// # Returns
            /// A masked `usize` value of the CSR.
            #[inline]
            pub fn read() -> usize {
                let value: usize;
                unsafe {
                    // Emit a `csrr` instruction to read from a constant CSR address.
                    core::arch::asm!("csrr {0}, {1}", out(reg) value, const $addr);
                }
                // Apply the mask to filter relevant bits
                value & $mask
            }
        }
    };
}
