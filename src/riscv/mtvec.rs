use core::arch::asm;

pub fn write_mtvec(value: usize) {
    unsafe { asm!("csrw mtvec, {}", in(reg) value) }
}
