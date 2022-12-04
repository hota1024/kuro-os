use core::arch::asm;

pub fn read_mie() -> usize {
    let mie: usize;

    unsafe {
        asm!("csrr {}, mie", out(reg) mie);
    }

    mie
}

pub fn write_mie(mie: usize) {
    unsafe {
        asm!("csrw mie, {}", in(reg) mie);
    }
}

pub fn set_mtie() {
    let mut mie = read_mie();

    // mie.MTIE = 1 に設定(タイマ割り込みを挿入)
    mie |= 1 << 7;

    write_mie(mie);
}
