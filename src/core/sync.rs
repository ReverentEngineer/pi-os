use lock_api::{RawMutex, GuardSend};
use core::sync::atomic::{AtomicBool, Ordering};

pub struct RawSpinlock {
    locked: AtomicBool 
}

impl RawSpinlock {

    pub const fn new() -> Self {
        Self {
            locked: AtomicBool::new(false)
        }
    }

}

unsafe impl RawMutex for RawSpinlock {

    const INIT: Self = RawSpinlock::new();

    type GuardMarker = GuardSend;

    fn lock(&self) {
        while self.try_lock() == false { }
    }

    fn try_lock(&self) -> bool {
        self.locked
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
    }

    unsafe fn unlock(&self) {
        self.locked.store(false, Ordering::Release)
    }
}
