//! ---------------------------------------------------------------------------
//! File       : time.rs
//! Module     : registers::time
//! Author     : DiTurr
//! Description:
//! Defines the time CSR register abstraction and accessors.
//! ---------------------------------------------------------------------------

use crate::define_csr;

define_csr!(
    /// Timer value.
    TIME,
    address: 0xC01,
    mask: 0xffff_ffff
);
