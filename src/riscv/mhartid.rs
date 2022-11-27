use core::arch::asm;

pub fn read_mhartid() -> usize {
    let mhartid: usize;

    unsafe {
        asm!("csrr {}, mhartid", out(reg) mhartid);
    }

    mhartid
}
