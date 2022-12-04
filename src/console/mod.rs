pub mod uart;

use core::fmt;

pub fn console_init() {
    uart::uart_init();
}

pub fn console_putc(c: u8) {
    uart::uart_putc(c);
}

struct Print;

impl fmt::Write for Print {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.bytes() {
            console_putc(c);
        }
        Ok(())
    }
}

pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    Print.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::console::_print(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n")
    };
    ($fmt:expr) => {
        $crate::print!(concat!($fmt, "\n"))
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::print!(concat!($fmt, "\n"), $($arg)*)
    };
}

pub(crate) use println;
