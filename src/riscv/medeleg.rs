use core::arch::asm;

pub fn write_medeleg(medeleg: usize) {
    unsafe {
        asm!("csrw medeleg, {}", in(reg) medeleg);
    }
}
