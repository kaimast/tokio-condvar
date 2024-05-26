use std::sync::Arc;

use tokio::sync::Mutex;
use tokio_condvar::Condvar;

#[tokio::test]
async fn wait() {
    let mutex = Arc::new(Mutex::new(false));
    let cond = Arc::new(Condvar::new());

    {
        let mutex = mutex.clone();
        let cond = cond.clone();

        tokio::spawn(async move {
            let mut lock = mutex.lock().await;
            *lock = true;
            cond.notify_all();
        });
    }

    let mut lock = mutex.lock().await;
    while !*lock {
        lock = cond.wait(lock).await;
    }

    // The value must have been set to true at this point
    assert!(*lock);
}

#[cfg(feature="parking_lot")]
#[tokio::test]
async fn parking_lot_notify() {
    let mutex = Arc::new(parking_lot::Mutex::new(false));
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
        lock = cond.parking_lot_wait(lock).await;
    }

    // The value must have been set to true at this point
    assert!(*lock);
}

#[cfg(feature="parking_lot")]
#[tokio::test]
async fn parking_lot_rw_notify() {
    let lock = Arc::new(parking_lot::RwLock::new(false));
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
        guard = cond.parking_lot_rw_wait(guard).await;
    }

    // The value must have been set to true at this point
    assert!(*guard);
}
