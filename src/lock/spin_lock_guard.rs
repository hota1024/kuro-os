use core::ops::{Deref, DerefMut};

use super::spin_lock::SpinLock;

pub struct SpinLockGuard<'a, T: ?Sized> {
    pub lock: &'a SpinLock<T>,
    pub data: &'a mut T,
}

/* スマートポインタ系の実装 */

// デリファレンス演算子のオーバーロード(`*`)
impl<'a, T: ?Sized> Deref for SpinLockGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        // data を返す
        &*self.data
    }
}

// mut なデリファレンス演算子のオーバーロード(`*mut`)
impl<'a, T: ?Sized> DerefMut for SpinLockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        // data を返す
        &mut *self.data
    }
}

// スコープを抜けたときの処理
impl<'a, T: ?Sized> Drop for SpinLockGuard<'a, T> {
    fn drop(&mut self) {
        self.lock.release();
    }
}

impl<'a, T> SpinLockGuard<'a, T> {
    pub unsafe fn hodling(&self) -> bool {
        self.lock.holding()
    }
}
