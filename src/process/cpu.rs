use crate::riscv::{sstatus, tp};

pub fn cpu_id() -> usize {
    tp::read_tp()
}

pub fn push_off() {
    sstatus::intterupt_off();
}

pub fn pop_off() {
    sstatus::intterupt_on();
}
