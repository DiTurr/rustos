//! ---------------------------------------------------------------------------
//! File       : main.rs
//! Module     : main
//! Author     : DiTurr
//! Description: Kernel entry point and basic panic handler.
//! ---------------------------------------------------------------------------

// We are not linking the Rust standard library.
#![no_std]
// We are not using the standard main entry point.
#![no_main]

// Import panic handler info.
use core::panic::PanicInfo;
// Declare logger module.
mod logger;
// Declare peripherals module.
mod peripherals;
// Declare registers module and import MEPC.
mod registers;
use registers::mepc::MEPC;
// Declare traps module.
mod traps;


// The kernel entry point called by the bootloader.
// It must never return (`-> !`).
#[unsafe(no_mangle)]
pub extern "C" fn kmain() -> ! {
    //
    let mepc = MEPC::read();
    log_info!("Kernel loaded at address {:#x}.", mepc);
    // Force trap
    unsafe { core::arch::asm!("ecall"); }
    // Infinite loop to prevent exiting
    loop {}
}


// This function is called on panic.
// It must never return (`-> !`).
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        log_error!("Kernel panic at line {}, file {}: {}", location.line(), location.file(), info.message());
    } else {
        log_error!("Kernel panic without additional information.");
    }
    // Infinite loop to prevent exiting
    loop {}
}
