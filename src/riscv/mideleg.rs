use core::arch::asm;

pub fn write_mideleg(mideleg: usize) {
    unsafe {
        asm!("csrw mideleg, {}", in(reg) mideleg);
    }
}
