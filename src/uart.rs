use core::ptr::{read_volatile, write_volatile};

const UART_BASE: usize = 0x1000_0000;

pub struct Uart;

impl Uart {
    pub fn new() -> Self {
        Uart
    }

    pub fn putb(&self, byte: u8) {
        unsafe {
            // Wait for Transmit Holding Register Empty (bit 5 of LSR)
            while read_volatile((UART_BASE + 5) as *const u8) & (1 << 5) == 0 {}
            write_volatile((UART_BASE + 0) as *mut u8, byte);
        }
    }

    pub fn puts(&self, s: &str) {
        for b in s.bytes() {
            self.putb(b);
        }
    }

    pub fn init(&self) {
        // No init needed for NS16550A in QEMU, it's ready by default
    }
}
