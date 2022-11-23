use crate::riscv::mstatus;

pub fn rust_main() -> ! {
    crate::console::console_init();
    crate::console::println!("KuroOS");
    mstatus::set_mpp(mstatus::MPP::Supervisor);

    loop {}
}
