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
