use core::arch::asm;

pub enum SStatus {
    SPP = 1 << 8,
    SPIE = 1 << 5,
    UPIE = 1 << 4,
    SIE = 1 << 1,
    UIE = 1 << 0,
}

pub fn read_sstatus() -> usize {
    let sstatus: usize;

    unsafe {
        asm!("csrr {}, sstatus", out(reg) sstatus);
    }

    sstatus
}

pub fn write_sstatus(sstatus: usize) {
    unsafe {
        asm!("csrw sstatus, {}", in(reg) sstatus);
    }
}

pub fn intterupt_on() {
    write_sstatus(read_sstatus() | SStatus::SIE as usize);
}

pub fn intterupt_off() {
    write_sstatus(read_sstatus() & !(SStatus::SIE as usize));
}
