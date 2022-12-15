use crate::{console, mem::kalloc::KERNEL_HEAP, println, process::cpu::cpu_id};

pub unsafe fn rust_main() -> ! {
    if cpu_id() == 0 {
        console::console_init();
        KERNEL_HEAP.kinit();
        println!("KuroOS");
    }

    loop {}
}
