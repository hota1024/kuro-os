use core::arch::asm;

pub fn write_mepc(pc: usize) {
    unsafe {
        asm!("csrw mepc, {}", in(reg) pc);
    }
}
