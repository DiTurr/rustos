//! ---------------------------------------------------------------------------
//! File       : mepc.rs
//! Module     : registers::mepc
//! Author     : DiTurr
//! Description: Defines the mepc CSR register abstraction and accessors.
//! ---------------------------------------------------------------------------

use crate::define_csr;

define_csr!(
    /// Machine Exception Program Counter.
    MEPC,
    address: 0x341,
    mask: 0xffff_ffff
);
