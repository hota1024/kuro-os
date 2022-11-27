use core::arch::asm;

pub fn mret() {
    unsafe {
        asm!("mret");
    }
}
