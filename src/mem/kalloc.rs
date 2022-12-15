use core::alloc::{GlobalAlloc, Layout};

use crate::{
    allocator::buddy_system::BuddySystem, consts::memory_layout::PHYSTOP,
    lock::spin_lock::SpinLock, println,
};

#[global_allocator]
pub static KERNEL_HEAP: KernelHeap = KernelHeap::uninit();

#[alloc_error_handler]
fn on_alloc_error(layout: Layout) -> ! {
    panic!("alloc error: {:?}", layout);
}

pub type KernelHeap = SpinLock<BuddySystem>;

impl KernelHeap {
    const fn uninit() -> Self {
        Self::new(BuddySystem::uninit(), "kernel_heap")
    }

    pub unsafe fn kinit(&self) {
        extern "C" {
            // linker.d で定義済み
            fn end();
        }

        let end = end as usize;
        println!(
            "KernelHeap: avaiable physical memory from {:#x} to {:#x}",
            end, PHYSTOP
        );
        self.init(end, PHYSTOP);
        println!("KernelHeap: memory initialized");
    }

    unsafe fn init(&self, start: usize, end: usize) {
        self.lock().init(start, end);
    }
}

unsafe impl GlobalAlloc for KernelHeap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.lock().alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.lock().dealloc(ptr, layout)
    }
}
