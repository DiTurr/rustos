//! ---------------------------------------------------------------------------
//! File       : main.rs
//! Module     : main
//! Author     : DiTurr
//! Description:
//! Kernel entry point and panic handler.
//! ---------------------------------------------------------------------------
// We are not linking the Rust standard library (needed for bare-metal systems).
#![no_std]
// We are not using the standard `main` entry point (replaced by `kmain` below).
#![no_main]

// Core panic handler trait (used to define custom panic behavior).
use core::panic::PanicInfo;

// Declare submodules used by the kernel.
mod logger;       // Logging infrastructure
mod peripherals;  // Memory-mapped I/O (UART, etc.)
mod registers;    // Low-level register access (CSRs, etc.)
mod traps;        // Trap (interrupt/exception) handling

// Import CSR abstraction for the machine exception program counter (MEPC).
use registers::mepc::MEPC;

/// Kernel entry point called by the bootloader.
/// This is the first Rust function executed after boot. It must never return,
/// hence the return type `-> !`.
#[no_mangle] // Ensure the symbol name remains exactly `kmain`
pub unsafe extern "C" fn kmain() -> ! {
    // Read the address at which the kernel was loaded (via MEPC CSR).
    let mepc = MEPC::read();
    log_info!("Kernel loaded at address {:#x}.", mepc);
    // Trigger a machine-level trap to test the trap handling system.
    unsafe { core::arch::asm!("ecall"); }
    // Enter an infinite loop to prevent returning from `kmain`.
    loop {}
}

/// Panic handler function for the kernel.
/// This is called whenever a panic occurs. Since we’re in `#![no_std]` mode,
/// we must define it manually. It never returns (`-> !`).
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // If the panic has location information, log it.
    // Otherwise, log a generic panic message.
    if let Some(location) = info.location() {
        log_error!(
            "Kernel panic at line {}, file {}: {}",
            location.line(),
            location.file(),
            info.message()
        );
    } else {
        log_error!("Kernel panic without additional information.");
    }
    // Enter an infinite loop to prevent exiting after panic.
    loop {}
}
