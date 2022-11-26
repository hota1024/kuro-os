use core::arch::asm;

pub enum SIE {
    External = 1 << 9,
    Timer = 1 << 5,
    Software = 1 << 1,
}

pub fn write_sie(sie: usize) {
    unsafe {
        asm!("csrw sie, {}", in(reg) sie);
    }
}

pub fn read_sie() -> usize {
    let sie: usize;

    unsafe {
        asm!("csrr {}, sie", out(reg) sie);
    }

    sie
}
