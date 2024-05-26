#[cfg(feature = "parking_lot")]
use std::sync::Arc;

#[cfg(feature = "parking_lot")]
use tokio_condvar::parking_lot::Condvar;

#[cfg(feature = "parking_lot")]
use parking_lot::{Mutex, RwLock};

#[cfg(feature = "parking_lot")]
#[tokio::test]
async fn notify() {
    let mutex = Arc::new(Mutex::new(false));
    let cond = Arc::new(Condvar::new());

    {
        let mutex = mutex.clone();
        let cond = cond.clone();

        tokio::spawn(async move {
            let mut lock = mutex.lock();
            *lock = true;
            cond.notify_one();
        });
    }

    let mut lock = mutex.lock();
    while !*lock {
        lock = cond.wait(lock).await;
    }

    // The value must have been set to true at this point
    assert!(*lock);
}

#[cfg(feature = "parking_lot")]
#[tokio::test]
async fn rw_notify() {
    let lock = Arc::new(RwLock::new(false));
    let cond = Arc::new(Condvar::new());

    {
        let lock = lock.clone();
        let cond = cond.clone();

        tokio::spawn(async move {
            let mut lock = lock.write();
            *lock = true;
            cond.notify_one();
        });
    }

    let mut guard = lock.read();
    while !*guard {
        guard = cond.rw_read_wait(guard).await;
    }

    // The value must have been set to true at this point
    assert!(*guard);
}
