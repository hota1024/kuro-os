use crate::{console, process::cpu::cpu_id};

pub fn rust_main() -> ! {
    if cpu_id() == 0 {
        console::console_init();
        console::println!("KuroOS");
    }

    loop {}
}
