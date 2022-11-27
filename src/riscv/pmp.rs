use core::arch::asm;

pub fn write_pmpcfg0(pmpcfg0: usize) {
    unsafe {
        asm!("csrw pmpcfg0, {}", in(reg) pmpcfg0);
    }
}

pub fn write_pmpaddr0(pmpaddr0: usize) {
    unsafe {
        asm!("csrw pmpaddr0, {}", in(reg) pmpaddr0);
    }
}
