use core::arch::asm;

pub fn write_mscratch(value: usize) {
    unsafe { asm!("csrw mscratch, {}", in(reg) value) }
}
