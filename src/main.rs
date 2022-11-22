#![no_std]
#![no_main]

#[allow(unused_imports)]
use kuro_os;

use core::panic::PanicInfo;
#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
