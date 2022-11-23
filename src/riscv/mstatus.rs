use core::arch::asm;

pub enum MPP {
    User = 0,
    Supervisor = 1,
    // Reserved = 2,
    Machine = 3,
}

pub fn read_mstatus() -> usize {
    let mstatus: usize;

    unsafe {
        asm!("csrr {}, mstatus", out(reg) mstatus);
    }

    mstatus
}

pub fn write_mstatus(mstatus: usize) {
    unsafe {
        asm!("csrw mstatus, {}", in(reg) mstatus);
    }
}

pub fn set_mpp(mpp: MPP) {
    let mstatus = read_mstatus();
    let mstatus = (mstatus & !(0b11 << 11)) | ((mpp as usize) << 11);
    write_mstatus(mstatus);
}
