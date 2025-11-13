// 
use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
    thread,
    time::Duration,
};
use std::sync::atomic::{AtomicBool, Ordering};

/// Shared state between the future and the setter.
struct WaitingState {
    ready: AtomicBool,             // whether condition became true
    waker: Mutex<Option<Waker>>,   // store waker so setter can wake the task
}

#[derive(Clone)]
struct WaitingFuture {
    state: Arc<WaitingState>,
}

impl WaitingFuture {
    /// Create a new WaitingFuture with initial condition false.
    fn new() -> Self {
        Self {
            state: Arc::new(WaitingState {
                ready: AtomicBool::new(false),
                waker: Mutex::new(None),
            }),
        }
    }

    /// Called by some other thread to mark the condition true and wake the future.
    fn set_ready(&self) {
        // Set the flag
        self.state.ready.store(true, Ordering::SeqCst);

        // Take the waker (if any) and wake the task so it gets polled again
        if let Some(w) = self.state.waker.lock().unwrap().take() {
            w.wake();
        }
    }
}

impl Future for WaitingFuture {
    type Output = ();

    /// The core: when polled, check the flag. If ready -> Poll::Ready(()), else
    /// store the waker and return Poll::Pending.
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Fast path: if condition set, return Ready immediately.
        if self.state.ready.load(Ordering::SeqCst) {
            println!("resolved!");
            return Poll::Ready(());
        }

        // Not ready yet: save the waker so the setter thread can wake us.
        {
            let mut waker_slot = self.state.waker.lock().unwrap();
            // store/replace the waker with the current one:
            *waker_slot = Some(cx.waker().clone());
        }

        // Optional: print that we're waiting (will print each time the executor polls)
        println!("waiting...");

        Poll::Pending
    }
}

fn main() {
    let fut = WaitingFuture::new();
    let fut_clone = fut.clone();

    // spawn a thread to set the condition after a short delay
    let setter = thread::spawn(move || {
        thread::sleep(Duration::from_millis(700));
        println!("setter: setting ready = true");
        fut_clone.set_ready();
    });

    // Drive the future to completion using a simple executor (futures::executor::block_on)
    futures::executor::block_on(async {
        fut.await;
        println!("future has completed in async context");
    });

    setter.join().unwrap();
}
