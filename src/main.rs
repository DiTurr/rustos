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

// Import panic handler info
use core::panic::PanicInfo;

// Declare the uart module and import UART
mod peripherals;
use peripherals::uart::UART;

//
mod registers;
use registers::mepc::MEPC;
// use registers::mcause::MCAUSE;


// The kernel entry point called by the bootloader.
//
// It must never return (`-> !`).
#[unsafe(no_mangle)]
pub extern "C" fn kmain() -> ! {
    //
    let mepc = MEPC::read();
    uart_println!("Kernel loaded at address {:#x}", mepc);
    // Infinite loop to prevent exiting
    loop {}
}

// This function is called on panic.
//
// Since we have no standard I/O, this writes a panic message to UART.
// It must never return (`-> !`).
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    uart_println!("Kernel panic.");
    // Infinite loop to prevent exiting
    loop {}
}
