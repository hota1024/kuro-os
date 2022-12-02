use core::arch::asm;

pub fn write_mscratch(value: usize) {
    unsafe { asm!("cswr mscratch, {}", in(reg) value) }
}
