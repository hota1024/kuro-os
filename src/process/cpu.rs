use crate::riscv::tp;

pub fn cpu_id() -> usize {
    tp::read_tp()
}
