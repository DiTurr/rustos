//! ---------------------------------------------------------------------------
//! File       : uart.rs
//! Module     : peripherals::uart
//! Author     : DiTurr
//! Description:
//! This module provides a minimal UART interface for sending characters and strings
//! over a serial interface using memory-mapped I/O (MMIO). It is designed for use
//! in `no_std` environments such as kernels or embedded systems.
//!
//! ## Features
//! - `Uart::putb`: Send a single byte.
//! - `Uart::puts`: Send a string slice byte-by-byte.
//! - `UART`: Global static UART instance.
//! - `uart_println!`: `println!`-like macro that writes to UART with formatting.
//!
//! ## Example
//! ```rust
//! use crate::UART;
//! UART.puts("Booting...\n");
//!
//! uart_println!("CPU ready");
//! uart_println!("Status code: {}", 0x42);
//! ```
//! ---------------------------------------------------------------------------

use core::ptr::{read_volatile, write_volatile};

/// Base address of the UART MMIO register block.
/// This address must match the hardware or QEMU memory map.
const UART_BASE: usize = 0x1000_0000;

/// A minimal UART interface for MMIO-based serial output.
///
/// This struct allows low-level control of a UART device by directly
/// accessing memory-mapped I/O registers. It provides methods for sending
/// bytes and string slices over the serial interface.
pub struct Uart;

impl Uart {
    /// Creates a new `Uart` instance.
    ///
    /// This is a `const fn`, allowing usage in `static` or `const` initializations.
    ///
    /// # Examples
    /// ```
    /// let uart = Uart::new();
    /// ```
    pub const fn new() -> Self {
        Uart
    }

    /// Sends a single byte over UART.
    ///
    /// This function busy-waits until the UART transmitter is ready, then
    /// writes the byte to the transmit register.
    ///
    /// # Arguments
    /// * `byte` - The byte to transmit.
    ///
    /// # Safety
    /// Performs raw pointer access to MMIO registers, and should only
    /// be used when it is safe to access the UART hardware.
    pub fn putb(&self, byte: u8) {
        unsafe {
            // Wait for Transmit Holding Register (THR) to be empty.
            while read_volatile((UART_BASE + 5) as *const u8) & (1 << 5) == 0 {}
            write_volatile((UART_BASE + 0) as *mut u8, byte);
        }
    }

    /// Sends a full string over UART.
    ///
    /// Internally sends each character one byte at a time using [`putb`].
    ///
    /// # Arguments
    /// * `s` - The UTF-8 string slice to send.
    ///
    /// # Examples
    /// ```
    /// UART.puts("Hello, world!\n");
    /// ```
    pub fn puts(&self, s: &str) {
        for b in s.bytes() {
            self.putb(b);
        }
    }
}

/// Global static UART instance.
///
/// This instance can be used throughout the system once memory-mapped
/// peripherals are accessible.
pub static UART: Uart = Uart::new();

/// Macro for printing a formatted line to UART.
///
/// This macro behaves similarly to `println!`, but writes directly to the
/// UART serial interface using the global [`UART`] instance and `core::fmt::Write`.
///
/// It automatically appends a newline (`\n`) at the end of the message.
///
/// # Examples
/// ```
/// uart_println!("Booting kernel...");
/// uart_println!("Value: {}", 1234);
/// ```
#[macro_export]
macro_rules! uart_println {
    ($($arg:tt)*) => {{
        use core::fmt::Write;

        // Adapter implementing core::fmt::Write for the UART
        struct UartWriter;

        impl core::fmt::Write for UartWriter {
            fn write_str(&mut self, s: &str) -> core::fmt::Result {
                $crate::UART.puts(s);
                Ok(())
            }
        }

        // Use `writeln!` to print the formatted message + newline.
        let _ = writeln!(UartWriter, $($arg)*);
    }};
}
