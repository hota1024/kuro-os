use core::{alloc::Layout, cmp, mem::MaybeUninit, ptr};

use crate::{
    consts::page::{LEAF_SIZE, PAGE_SIZE},
    println,
};

use super::{
    buddy_info::BuddyInfo,
    helpers::{block_size, init_slice_empty, log2, round_down, round_up},
    list::List,
};

// BuddySystem はメモリ割り当てのアルゴリズム。

pub struct BuddySystem {
    initialized: bool,
    base: usize,
    actual_end: usize,
    nsizes: usize,
    infos: MaybeUninit<*mut [BuddyInfo]>,
}

unsafe impl Send for BuddySystem {}

impl BuddySystem {
    pub const fn uninit() -> Self {
        Self {
            initialized: false,
            base: 0,
            actual_end: 0,
            nsizes: 0,
            infos: MaybeUninit::uninit(),
        }
    }

    pub unsafe fn init(&mut self, start: usize, end: usize) {
        if self.initialized {
            panic!("buddy system already initialized");
        }

        let size = cmp::max(LEAF_SIZE, PAGE_SIZE);
        let mut cur: usize = round_up(start, size);
        self.base = cur;
        self.actual_end = round_down(end, size);

        self.nsizes = log2((self.actual_end - cur) / LEAF_SIZE) + 1;
        if self.actual_end - cur > block_size(self.max_size()) {
            self.nsizes += 1;
        }

        println!(
            "buddy system: useful memory is {:#x} bytes",
            self.actual_end - self.base
        );
        println!("buddy system: leaf size is {} bytes", LEAF_SIZE);
        println!(
            "buddy system: free lists have {:#x} different sizes",
            self.nsizes
        );

        let info_slice_ptr = init_slice_empty(&mut cur, self.nsizes);
        self.infos.as_mut_ptr().write(info_slice_ptr);

        for i in 0..self.nsizes {
            let block_num = self.count_blocks(i);
            let info = self.get_info_mut(i);

            info.free.init();

            let alloc_size = round_up(block_num, 8) / 8;
            let alloc_slice_ptr = init_slice_empty(&mut cur, alloc_size);
            info.alloc.as_mut_ptr().write(alloc_slice_ptr);
        }

        for i in 1..self.nsizes {
            let block_num = self.count_blocks(i);
            let info = self.get_info_mut(i);

            let split_size = round_up(block_num, 8) / 8;
            let split_slice_ptr = init_slice_empty(&mut cur, split_size);
            info.split.as_mut_ptr().write(split_slice_ptr);
        }

        cur = round_up(cur, LEAF_SIZE);

        let meta = self.mark_meta(cur);

        let unavailable = self.mark_unavailable();

        let free = self.init_free(cur);

        if free != block_size(self.max_size()) - meta - unavailable {
            panic!("buddy system: init failed");
        }
        println!("here");

        self.initialized = true;
    }

    pub fn alloc(&mut self, layout: Layout) -> *mut u8 {
        if layout.size() == 0 {
            return ptr::null_mut();
        }

        if layout.align() > PAGE_SIZE {
            panic!(
                "buddy system: alignment({}) is larger than PAGE_SIZE({})",
                layout.align(),
                PAGE_SIZE
            );
        }

        let smalli = if layout.size() <= LEAF_SIZE {
            0
        } else {
            (layout.size().next_power_of_two() / LEAF_SIZE).trailing_zeros() as usize
        };

        let mut sizei = smalli;

        while sizei < self.nsizes {
            let info = unsafe { self.get_info_mut(sizei) };

            if !info.free.is_empty() {
                break;
            }

            sizei += 1;
        }

        if sizei >= self.nsizes {
            return ptr::null_mut();
        }

        let info = unsafe { self.get_info_mut(sizei) };
        let raw_addr = unsafe { info.free.pop() };
        let bi = self.block_index(sizei, raw_addr);
        unsafe {
            self.get_info_mut(sizei).alloc_set(bi, true);
        }

        while sizei > smalli {
            let bi = self.block_index(sizei, raw_addr);
            let info = unsafe { self.get_info_mut(sizei) };
            info.split_set(bi, true);

            let bi1 = self.block_index(sizei - 1, raw_addr);
            let info1 = unsafe { self.get_info_mut(sizei - 1) };
            info1.alloc_set(bi1, true);

            let buddy_addr = raw_addr + block_size(sizei - 1);
            unsafe { info1.free.push(buddy_addr) };

            sizei -= 1;
        }

        raw_addr as *mut u8
    }

    pub fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
        let mut raw_addr = ptr as usize;
        if raw_addr < self.base || raw_addr >= self.actual_end {
            panic!("buddy system: invalid address");
        }

        let mut sizei = self.nsizes;
        for i in 0..self.max_size() {
            let bi = self.block_index(i + 1, raw_addr);
            let info = unsafe { self.get_info_mut(i + 1) };

            if info.is_split_set(bi) {
                sizei = i;
                break;
            }
        }

        if sizei == self.nsizes {
            panic!("buddy system: dealloc cannot recycle ptr");
        }

        if layout.size() > block_size(sizei) {
            panic!(
                "  buddy system: dealloc layout size({:?}) is larger than block size({})",
                layout,
                block_size(sizei)
            );
        }

        while sizei < self.max_size() {
            let bi = self.block_index(sizei, raw_addr);
            let buddyi = if bi % 2 == 0 { bi + 1 } else { bi - 1 };
            let info = unsafe { self.get_info_mut(sizei) };

            info.alloc_set(bi, false);

            if info.is_alloc_set(buddyi) {
                break;
            }

            let buddy_addr = self.block_addr(sizei, buddyi);
            unsafe { (buddy_addr as *mut List).as_mut().unwrap().remove() }

            if buddyi % 2 == 0 {
                raw_addr = buddy_addr;
            }

            sizei += 1;
            let spliti = self.block_index(sizei, raw_addr);
            let info = unsafe { self.get_info_mut(sizei) };
            info.split_set(spliti, false);
        }

        let info = unsafe { self.get_info_mut(sizei) };
        unsafe { info.free.push(raw_addr) };
    }

    fn mark_meta(&mut self, cur: usize) -> usize {
        let meta = cur - self.base;
        println!("buddy system: alloc {:#x} bytes meta data", meta);
        self.mark(self.base, cur);

        meta
    }

    fn mark_unavailable(&mut self) -> usize {
        let unavailable = block_size(self.max_size()) - (self.actual_end - self.base);
        self.mark(self.actual_end, self.actual_end + unavailable);

        unavailable
    }

    fn mark(&mut self, left: usize, right: usize) {
        assert_eq!(left % LEAF_SIZE, 0);
        assert_eq!(right % LEAF_SIZE, 0);

        for i in 0..self.nsizes {
            let mut bi = self.block_index(i, left);
            let bj = self.block_index_next(i, right);

            while bi < bj {
                let info = unsafe { self.get_info_mut(i) };

                info.alloc_set(bi, true);

                if i > 0 {
                    info.split_set(bi, true);
                }

                bi += 1;
            }
        }
    }

    fn init_free(&mut self, left: usize) -> usize {
        let right = self.actual_end;
        let mut free = 0;

        for i in 0..self.max_size() {
            let left_bi = self.block_index_next(i, left);
            let right_bi = self.block_index(i, right);
            free += self.init_free_pair(i, left_bi);

            if left < right {
                free += self.init_free_pair(i, right_bi);
            }
        }

        free
    }

    fn init_free_pair(&mut self, sizei: usize, bi: usize) -> usize {
        let buddyi = if bi % 2 == 0 { bi + 1 } else { bi - 1 };
        let block_addr_bi = self.block_addr(sizei, bi);
        let block_addr_buddyi = self.block_addr(sizei, buddyi);

        let info = unsafe { self.get_info_mut(sizei) };

        if info.is_alloc_set(bi) != info.is_alloc_set(buddyi) {
            unsafe {
                if info.is_alloc_set(bi) {
                    info.free.push(block_addr_buddyi);
                } else {
                    info.free.push(block_addr_bi);
                }
            }

            block_size(sizei)
        } else {
            0
        }
    }

    unsafe fn get_info_mut(&mut self, index: usize) -> &mut BuddyInfo {
        let info_slice_ptr = *self.infos.as_ptr();

        info_slice_ptr.get_unchecked_mut(index).as_mut().unwrap()
    }

    fn max_size(&self) -> usize {
        self.nsizes - 1
    }

    fn count_blocks(&self, k: usize) -> usize {
        1 << (self.nsizes - k - 1)
    }

    fn block_index(&self, k: usize, addr: usize) -> usize {
        (addr - self.base) / block_size(k)
    }

    fn block_index_next(&self, k: usize, addr: usize) -> usize {
        let mut i = (addr - self.base) / block_size(k);

        if (addr - self.base) % block_size(k) != 0 {
            i += 1;
        }

        i
    }

    fn block_addr(&self, k: usize, index: usize) -> usize {
        self.base + block_size(k) * index
    }
}
