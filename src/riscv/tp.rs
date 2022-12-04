use core::arch::asm;

pub fn read_tp() -> usize {
    let tp: usize;

    unsafe {
        asm!("mv {}, tp", out(reg) tp);
    }

    tp
}

pub fn write_tp(tp: usize) {
    unsafe {
        asm!("mv tp, {}", in(reg) tp);
    }
}
