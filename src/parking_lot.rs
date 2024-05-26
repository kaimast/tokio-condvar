// See this clippy bug: https://github.com/rust-lang/rust-clippy/issues/8777
#![allow(clippy::await_holding_lock)]

use parking_lot::{MutexGuard, RwLockReadGuard};
use tokio::sync::Notify;

#[derive(Default)]
pub struct Condvar {
    inner: Notify,
}

impl Condvar {
    pub fn new() -> Self {
        Self::default()
    }

    /// Wake up all tasks currently waiting on this condition variable.
    pub fn notify_all(&self) {
        self.inner.notify_waiters();
    }

    /// Wake up exactly one task that is currently waiting on this condition variable
    /// If no task is waiting, none will be woken up.
    pub fn notify_one(&self) {
        // Not supported yet
        self.inner.notify_waiters();
    }

    /// Wait to be woken up while holding a lock
    /// This function will automatically release the lock before waiting
    /// and reacquires it after waking up
    pub async fn wait<'a, T>(&self, guard: MutexGuard<'a, T>) -> MutexGuard<'a, T> {
        let fut = self.inner.notified();
        tokio::pin!(fut);
        fut.as_mut().enable();

        let mutex = MutexGuard::mutex(&guard);
        drop(guard);

        fut.await;
        mutex.lock()
    }

    /// Same as `Self::wait` but for a parking_lot read-write lock
    pub async fn rw_read_wait<'a, T>(
        &self,
        guard: RwLockReadGuard<'a, T>,
    ) -> RwLockReadGuard<'a, T> {
        let fut = self.inner.notified();
        tokio::pin!(fut);
        fut.as_mut().enable();

        let lock = RwLockReadGuard::rwlock(&guard);
        drop(guard);

        fut.await;
        lock.read()
    }
}
