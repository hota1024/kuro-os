use core::{
    cell::{Cell, UnsafeCell},
    sync::atomic::{fence, AtomicBool, Ordering},
};

use crate::process::cpu::{cpu_id, pop_off, push_off};

use super::spin_lock_guard::SpinLockGuard;

pub struct SpinLock<T: ?Sized> {
    locked: AtomicBool,
    name: &'static str,
    cpu_id: Cell<isize>,
    data: UnsafeCell<T>,
}

unsafe impl<T: ?Sized + Send> Sync for SpinLock<T> {}

impl<T> SpinLock<T> {
    pub const fn new(data: T, name: &'static str) -> Self {
        Self {
            locked: AtomicBool::new(false),
            name,
            cpu_id: Cell::new(-1),
            data: UnsafeCell::new(data),
        }
    }
}

impl<T: ?Sized> SpinLock<T> {
    pub fn lock(&self) -> SpinLockGuard<'_, T> {
        self.acquire();
        SpinLockGuard {
            lock: &self,
            data: unsafe { &mut *self.data.get() },
        }
    }

    fn acquire(&self) {
        push_off();

        if self.holding() {
            panic!("acquire");
        }

        while self.locked.swap(true, Ordering::Acquire) {
            // spin loop
        }
        fence(Ordering::SeqCst);
    }

    pub fn release(&self) {
        if !self.holding() {
            panic!("release");
        }

        fence(Ordering::SeqCst);
        self.locked.store(false, Ordering::Release);

        pop_off();
    }

    pub fn holding(&self) -> bool {
        let locked = self.locked.load(Ordering::Relaxed);

        locked && self.cpu_id.get() == cpu_id() as isize
    }
}
