use crate::consts::memory_layout::UART0;
use core::ptr;

fn dst(addr: usize) -> *mut u8 {
    (UART0 + addr) as *mut u8
}

pub fn uart_init() {
    unsafe {
        // Turn off the FIFO buffer
        ptr::write_volatile(dst(0), 0);

        ptr::write_volatile(dst(3), 0x80); // unlock divisorc
        ptr::write_volatile(dst(0), 12); // set baud rate to 9600(12 = 115200 / 9600)
        ptr::write_volatile(dst(1), 0);
        ptr::write_volatile(dst(3), 0x03); // lock divisor, set word length to 8 bits
        ptr::write_volatile(dst(4), 0);
        ptr::write_volatile(dst(1), 0x01); // enable receive interrupts
    }
}

pub fn uart_putc(c: u8) {
    unsafe {
        while ptr::read_volatile(dst(5)) & 0x20 == 0 {}

        ptr::write_volatile(dst(0), c);
    }
}
