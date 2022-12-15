use crate::{console, mem::kalloc::KERNEL_HEAP, process::cpu::cpu_id};

pub unsafe fn rust_main() -> ! {
    if cpu_id() == 0 {
        console::console_init();
        console::println!("KuroOS");
        KERNEL_HEAP.kinit();
    }

    loop {}
}
