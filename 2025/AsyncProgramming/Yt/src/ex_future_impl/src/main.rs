use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use std::thread;
//use std::time::{Duration, Instant};
use std::time::Duration;

// 1. THE STATE: This represents our "Reactor" state
struct TimerState {
    completed: bool,
    waker: Option<std::task::Waker>,
}

pub struct MyTimerFuture {
    state: Arc<Mutex<TimerState>>,
}

impl MyTimerFuture {
    pub fn new(duration: Duration) -> Self {
        let state = Arc::new(Mutex::new(TimerState {
            completed: false,
            waker: None,
        }));

        let thread_state = state.clone();

        // 2. THE REACTOR MOCK: We spawn an OS thread to simulate
        // hardware (like a network card or clock) waiting for an event.
        thread::spawn(move || {
            thread::sleep(duration);
            let mut guard = thread_state.lock().unwrap();
            guard.completed = true;

            // 3. THE WAKE: If the Executor has registered a Waker,
            // we trigger it to tell the Executor "I'm ready!"
            if let Some(waker) = guard.waker.take() {
                waker.wake();
            }
        });

        MyTimerFuture { state }
    }
}

// 4. THE FUTURE TRAIT: This is what the Executor calls
impl Future for MyTimerFuture {
    type Output = String;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut guard = self.state.lock().unwrap();

        if guard.completed {
            // If the reactor finished, we return Ready
            Poll::Ready("Timer Finished!".to_string())
        } else {
            // 5. THE REGISTRATION: If not finished, we store the 'Waker'
            // from the Context so the Reactor knows who to notify later.
            guard.waker = Some(cx.waker().clone());

            // Return Pending to let the Executor know the thread is free
            Poll::Pending
        }
    }
}

#[tokio::main]
async fn main() {
    let timer = MyTimerFuture::new(Duration::from_secs(5));

    println!("Waiting for the future...");

    // .await calls 'poll' initially, then goes to sleep until 'wake' is called
    let result = timer.await;

    println!("{}", result);
}
