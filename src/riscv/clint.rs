use core::ptr;

use crate::consts::memory_layout::{CLINT_MTIME, CLINT_MTIMECMP};

pub fn read_mtime() -> usize {
    unsafe { ptr::read_volatile(CLINT_MTIME as *const usize) }
}

fn get_mtimecmp_addr(hartid: usize) -> usize {
    CLINT_MTIMECMP + 8 * hartid
}

pub fn read_mtimecmp(mhartid: usize) -> usize {
    unsafe { ptr::read_volatile(get_mtimecmp_addr(mhartid) as *const usize) }
}

pub fn write_mtimecmp(mhartid: usize, value: usize) {
    unsafe {
        ptr::write_volatile(get_mtimecmp_addr(mhartid) as *mut usize, value);
    }
}

pub fn add_mtimecmp(mhartid: usize, value: usize) {
    let mtime = read_mtime();
    write_mtimecmp(mhartid, mtime + value);
}
