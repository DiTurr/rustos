//! ---------------------------------------------------------------------------
//! File       : uart.rs
//! Module     : peripherals::uart
//! Author     : DiTurr
//! Description: UART peripheral interface and register definitions.
//!              Minimal UART driver for memory-mapped I/O on a fixed address.
//!              This driver supports writing single bytes or full strings
//!              to the UART, and includes a `uart_println!`.
//! ---------------------------------------------------------------------------

use core::ptr::{read_volatile, write_volatile};

/// Base address of the UART MMIO register block.
const UART_BASE: usize = 0x1000_0000;

/// A minimal UART interface for MMIO-based serial output.
pub struct Uart;

impl Uart {
    /// Creates a new `Uart` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// let uart = Uart::new();
    /// ```
    ///
    /// This is a `const fn` and can be used in `static` initializations.
    pub const fn new() -> Self {
        Uart
    }

    /// Sends a single byte over UART.
    ///
    /// Blocks until the UART is ready to transmit (THR empty).
    ///
    /// # Arguments
    ///
    /// * `byte` - The byte to send.
    ///
    /// # Safety
    ///
    /// Accesses hardware registers via raw pointers.
    pub fn putb(&self, byte: u8) {
        unsafe {
            // Wait for Transmit Holding Register Empty (bit 5 of LSR)
            while read_volatile((UART_BASE + 5) as *const u8) & (1 << 5) == 0 {}
            write_volatile((UART_BASE + 0) as *mut u8, byte);
        }
    }

    /// Sends a string over UART.
    ///
    /// Internally sends one byte at a time using [`putb`].
    ///
    /// # Arguments
    ///
    /// * `s` - The string slice to send.
    pub fn puts(&self, s: &str) {
        for b in s.bytes() {
            self.putb(b);
        }
    }
}

/// Global static UART instance.
///
/// This can be used anywhere to write to UART after initialization.
pub static UART: Uart = Uart::new();

/// Macro for writing a formatted line to the UART (with newline).
///
/// This behaves like `println!`, but writes directly to the UART using
/// the [`UART`] instance. It uses `core::fmt` under the hood.
///
/// # Examples
///
/// ```
/// uart_println!("Hello, UART!");
/// uart_println!("Value: {}", 42);
/// ```
#[macro_export]
macro_rules! uart_println {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        struct UartWriter;
        impl core::fmt::Write for UartWriter {
            fn write_str(&mut self, s: &str) -> core::fmt::Result {
                $crate::UART.puts(s);
                Ok(())
            }
        }
        let _ = writeln!(UartWriter, $($arg)*);
    }};
}
