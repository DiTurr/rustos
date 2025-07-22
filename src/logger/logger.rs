//! ---------------------------------------------------------------------------
//! File       : logger.rs
//! Module     : logger::logger
//! Author     : DiTurr
//! Description:
//! This module provides logging macros (`log_info!`, `log_warn!`, `log_error!`) for use in
//! a `no_std` embedded or OS environment. It avoids heap allocations and uses `core::fmt::Write`
//! to print log messages directly through a UART device.
//! Each log entry includes a timestamp derived from the hardware timer and is prefixed
//! with a log level tag (e.g., `[INF]`, `[WRN]`, `[ERR]`).
//!
//! ## Features
//! - Uses a constant CPU frequency to convert timer ticks into milliseconds.
//! - Relies on a `UART` driver to send output directly to a serial port.
//! - Exported macros can be used anywhere in the crate for structured logging.
//!
//! ## Example
//! ```rust
//! log_info!("Starting kernel...");
//! log_warn!("Battery level is low: {}%", 18);
//! log_error!("Unhandled exception: code={:#x}", code);
//! ```
//!
//! ---------------------------------------------------------------------------

/// The fixed CPU frequency in Hertz used to calculate wall-clock time
/// from timer ticks. This value must match the actual hardware or
/// emulated platform (e.g., QEMU) timer frequency.
pub const CPU_FREQ: u64 = 10_000_000;

/// Logs a message with a given level (e.g., "[INF]", "[WRN]") and timestamp.
///
/// This macro uses `core::fmt::Write` to format the message without heap allocation,
/// and outputs via the UART peripheral. It includes the current time in milliseconds
/// since boot, computed from the hardware timer register.
///
/// # Examples
/// ```rust
/// logln!("[INF]", "System initialized.");
/// logln!("[ERR]", "Failed at step {}", 42);
/// ```
#[macro_export]
macro_rules! logln {
    ($level:expr, $($arg:tt)*) => {{
        use core::fmt::Write;
        // Writer wrapper that implements core::fmt::Write for UART output.
        struct UartWriter;

        impl core::fmt::Write for UartWriter {
            fn write_str(&mut self, s: &str) -> core::fmt::Result {
                // Send each string fragment via the UART driver.
                $crate::peripherals::uart::UART.puts(s);
                Ok(())
            }
        }
        // Read the current timer value (in ticks) from the TIME register.
        let time: usize = $crate::registers::time::TIME::read();
        // Convert to milliseconds using the CPU frequency.
        let time_ms: u64 = (time as u64 * 1_000) / $crate::logger::logger::CPU_FREQ;
        // Print the log level and timestamp prefix.
        let _ = write!(UartWriter, "{} [{}] ", $level, time_ms);
        // Print the formatted message body.
        let _ = writeln!(UartWriter, $($arg)*);
    }};
}

/// Logs an info-level message (abbreviated as "[INF]").
///
/// # Example
/// ```rust
/// log_info!("Kernel started");
/// ```
#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        $crate::logln!("[INF]", $($arg)*);
    };
}

/// Logs a warning-level message (abbreviated as "[WRN]").
///
/// # Example
/// ```rust
/// log_warn!("Battery level low: {}%", level);
/// ```
#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        $crate::logln!("[WRN]", $($arg)*);
    };
}

/// Logs an error-level message (abbreviated as "[ERR]").
///
/// # Example
/// ```rust
/// log_error!("Kernel panic: code {}", panic_code);
/// ```
#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        $crate::logln!("[ERR]", $($arg)*);
    };
}
