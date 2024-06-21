use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};

use parking_lot::{Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};

use tokio::sync::futures::Notified;
use tokio::sync::Notify;

#[derive(Default)]
pub struct Condvar {
    inner: Notify,
}

pub struct MutexFuture<'a, T> {
    mutex: &'a Mutex<T>,
    inner: Pin<Box<Notified<'a>>>,
}

impl<'a, T> Future for MutexFuture<'a, T> {
    type Output = MutexGuard<'a, T>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match Notified::poll(self.inner.as_mut(), cx) {
            Poll::Ready(_) => Poll::Ready(self.mutex.lock()),
            Poll::Pending => Poll::Pending,
        }
    }
}

pub struct RwReadFuture<'a, T> {
    lock: &'a RwLock<T>,
    inner: Pin<Box<Notified<'a>>>,
}

impl<'a, T> Future for RwReadFuture<'a, T> {
    type Output = RwLockReadGuard<'a, T>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match Notified::poll(self.inner.as_mut(), cx) {
            Poll::Ready(_) => Poll::Ready(self.lock.read()),
            Poll::Pending => Poll::Pending,
        }
    }
}

pub struct RwWriteFuture<'a, T> {
    lock: &'a RwLock<T>,
    inner: Pin<Box<Notified<'a>>>,
}

impl<'a, T> Future for RwWriteFuture<'a, T> {
    type Output = RwLockWriteGuard<'a, T>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match Notified::poll(self.inner.as_mut(), cx) {
            Poll::Ready(_) => Poll::Ready(self.lock.write()),
            Poll::Pending => Poll::Pending,
        }
    }
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
    ///
    /// Note: If no task is currently waiting, this might lead to spurious wakeups in the future.
    pub fn notify_one(&self) {
        self.inner.notify_one();
    }

    /// Wait to be woken up while holding a lock
    /// This function will automatically release the lock before waiting
    /// and reacquires it after waking up
    pub fn wait<'a, T>(&'a self, guard: MutexGuard<'a, T>) -> MutexFuture<'a, T> {
        let mut notify_fut = Box::pin(self.inner.notified());
        notify_fut.as_mut().enable();

        let mutex = MutexGuard::mutex(&guard);
        drop(guard);

        MutexFuture {
            mutex,
            inner: notify_fut,
        }
    }

    /// Same as `Self::wait` but for a parking_lot read-write lock
    pub fn rw_read_wait<'a, T>(&'a self, guard: RwLockReadGuard<'a, T>) -> RwReadFuture<'a, T> {
        let mut notify_fut = Box::pin(self.inner.notified());
        notify_fut.as_mut().enable();

        let lock = RwLockReadGuard::rwlock(&guard);

        RwReadFuture {
            lock,
            inner: notify_fut,
        }
    }

    /// Same as Self::wait but for a RwLockWriteGuard
    pub fn rw_write_wait<'a, T>(&'a self, guard: RwLockWriteGuard<'a, T>) -> RwWriteFuture<'a, T> {
        let mut notify_fut = Box::pin(self.inner.notified());
        notify_fut.as_mut().enable();

        let lock = RwLockWriteGuard::rwlock(&guard);

        RwWriteFuture {
            lock,
            inner: notify_fut,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use parking_lot::RwLock;

    fn assert_send<T: Send>(_: T) {}

    #[tokio::test]
    async fn rw_read_is_send() {
        let lock = RwLock::new(false);
        let cond = Condvar::default();
        let guard = lock.read();

        assert_send(cond.rw_read_wait(guard));
    }

    #[tokio::test]
    async fn rw_write_is_send() {
        let lock = RwLock::new(false);
        let cond = Condvar::default();
        let guard = lock.write();

        assert_send(cond.rw_write_wait(guard));
    }
}
