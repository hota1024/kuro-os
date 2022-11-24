use core::arch::asm;

pub fn write_satp(satp: usize) {
    unsafe {
        asm!("csrw satp, {}", in(reg) satp);
    }
}
