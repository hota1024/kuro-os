use core::{mem::size_of, ptr};

use crate::{consts::page::LEAF_SIZE, println};

#[inline]
pub fn round_up(n: usize, size: usize) -> usize {
    (((n - 1) / size) + 1) * size
}

#[inline]
pub fn round_down(n: usize, size: usize) -> usize {
    (n / size) * size
}

pub fn log2(mut n: usize) -> usize {
    let mut k = 0;
    while n > 1 {
        k += 1;
        n >>= 1;
    }
    k
}

pub fn block_size(k: usize) -> usize {
    (1 << k) * LEAF_SIZE
}

pub unsafe fn init_slice_empty<T>(cur: &mut usize, len: usize) -> *mut [T] {
    let raw_ptr = *cur as *mut T;
    *cur += size_of::<T>() * len;

    ptr::write_bytes(raw_ptr, 0, len);
    let a = ptr::slice_from_raw_parts_mut(raw_ptr, len);

    a
}
