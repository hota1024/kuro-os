use super::list::List;
use core::{mem::MaybeUninit, ops::BitOrAssign};

#[repr(C)]
pub struct BuddyInfo {
    // 自由リスト
    pub free: List,
    // アロケートされたブロックを追跡するための配列
    pub alloc: MaybeUninit<*mut [u8]>,
    // スプリットされたブロックを追跡するための配列
    pub split: MaybeUninit<*mut [u8]>,
}

impl BuddyInfo {
    pub unsafe fn get_alloc(&self, index: usize) -> &u8 {
        let alloc_slice_ptr = *self.alloc.as_ptr() as *const [u8];
        alloc_slice_ptr.get_unchecked(index).as_ref().unwrap()
    }

    pub unsafe fn get_alloc_mut(&mut self, index: usize) -> &mut u8 {
        let alloc_slice_ptr = *self.alloc.as_mut_ptr() as *mut [u8];
        alloc_slice_ptr.get_unchecked_mut(index).as_mut().unwrap()
    }

    pub unsafe fn get_split(&self, index: usize) -> &u8 {
        let split_slice_ptr = *self.split.as_ptr() as *const [u8];
        split_slice_ptr.get_unchecked(index).as_ref().unwrap()
    }

    pub unsafe fn get_split_mut(&mut self, index: usize) -> &mut u8 {
        let split_slice_ptr = *self.split.as_mut_ptr() as *mut [u8];
        split_slice_ptr.get_unchecked_mut(index).as_mut().unwrap()
    }

    pub fn alloc_set(&mut self, index: usize, value: bool) {
        let (i1, i2) = self.get_indexes(index);

        let alloc = unsafe { self.get_alloc_mut(i1) };
        let bit = if value { 1 } else { 0 };
        alloc.bitor_assign(bit << i2);
    }

    pub fn split_set(&mut self, index: usize, value: bool) {
        let (i1, i2) = self.get_indexes(index);

        let split = unsafe { self.get_split_mut(i1) };
        let bit = if value { 1 } else { 0 };
        split.bitor_assign(bit << i2);
    }

    pub unsafe fn alloc_get(&self, index: usize) -> bool {
        let (i1, i2) = self.get_indexes(index);

        let alloc = self.get_alloc(i1);
        alloc & (1 << i2) != 0
    }

    pub unsafe fn split_get(&self, index: usize) -> bool {
        let (i1, i2) = self.get_indexes(index);

        let split = self.get_split(i1);
        split & (1 << i2) != 0
    }

    pub fn is_alloc_set(&self, index: usize) -> bool {
        let (i1, i2) = self.get_indexes(index);

        unsafe { self.get_alloc(i1) & (1 << i2) != 0 }
    }

    pub fn is_split_set(&self, index: usize) -> bool {
        let (i1, i2) = self.get_indexes(index);

        unsafe { self.get_split(i1) & (1 << i2) != 0 }
    }

    fn get_indexes(&self, index: usize) -> (usize, usize) {
        let i1 = index / 8;
        let i2 = index % 8;

        (i1, i2)
    }
}
