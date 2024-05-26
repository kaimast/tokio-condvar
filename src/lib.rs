use tokio::sync::{MutexGuard, Notify};

#[derive(Default)]
pub struct Condvar {
    inner: Notify,
}

impl Condvar {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn notify_all(&self) {
        self.inner.notify_waiters();
    }

    pub fn notify_one(&self) {
        // Not supported yet
        self.inner.notify_waiters();
    }

    pub async fn wait<'a, T>(&self, guard: MutexGuard<'a, T>) -> MutexGuard<'a, T> {
        let fut = self.inner.notified();
        tokio::pin!(fut);
        fut.as_mut().enable();

        let mutex = MutexGuard::mutex(&guard);
        drop(guard);

        fut.await;
        mutex.lock().await
    }

    #[cfg(feature = "parking_lot")]
    pub async fn parking_lot_wait<'a, T>(
        &self,
        guard: parking_lot::MutexGuard<'a, T>,
    ) -> parking_lot::MutexGuard<'a, T> {
        let fut = self.inner.notified();
        tokio::pin!(fut);
        fut.as_mut().enable();

        let mutex = parking_lot::MutexGuard::mutex(&guard);
        drop(guard);

        fut.await;
        mutex.lock()
    }

    #[cfg(feature = "parking_lot")]
    pub async fn parking_lot_rw_wait<'a, T>(
        &self,
        guard: parking_lot::RwLockReadGuard<'a, T>,
    ) -> parking_lot::RwLockReadGuard<'a, T> {
        let fut = self.inner.notified();
        tokio::pin!(fut);
        fut.as_mut().enable();

        let lock = parking_lot::RwLockReadGuard::rwlock(&guard);
        drop(guard);

        fut.await;
        lock.read()
    }
}
