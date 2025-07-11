#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod uart;
use uart::Uart;

#[unsafe(no_mangle)]
pub extern "C" fn kmain() -> ! {
    let uart = Uart::new();
    uart.init();
    uart.puts("Hello from kmain!\n");

    loop {
        uart.puts("Hello from kmain!\n");
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    let uart = Uart::new();
    uart.puts("Kernel panic!\n");
    loop {}
}
