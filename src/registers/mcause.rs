//! ---------------------------------------------------------------------------
//! File       : mcause.rs
//! Module     : registers::mcause
//! Author     : DiTurr
//! Description:
//! Defines the Mcause CSR register abstraction and accessors.
//! ---------------------------------------------------------------------------

use crate::define_csr;

define_csr!(
    /// Machine trap cause.
    MCAUSE,
    address: 0x342,
    mask: 0xffff_ffff
);
