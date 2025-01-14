use std::future::Future;
use std::sync::Arc;
use std::sync::RwLock;
use std::task::Poll;
use std::task::Waker;

struct SharedState {
	waker: Option<Waker,>,
}

struct IOOperation {
	shared_state: Arc<RwLock<SharedState,>,>,
}

impl Future for IOOperation {
	type Output = String;

	fn poll(
		self: std::pin::Pin<&mut Self,>,
		cx: &mut std::task::Context<'_,>,
	) -> std::task::Poll<Self::Output,> {
		let mut shared_state = self.shared_state.write().unwrap();

		if shared_state.waker.is_none() {
			// store the waker so the other thread can wake
			shared_state.waker = Some(cx.waker().clone(),);
			Poll::Pending
		} else {
			Poll::Ready("I/O Operation completed!".to_string(),)
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use anyhow::Result as Rslt;
	use futures::executor::ThreadPool;
	use std::thread;
	use std::time::Duration;

	fn simulate_io_event_loop(shared_state: Arc<RwLock<SharedState,>,>,) {
		let io_event_loop_thread = thread::spawn(move || {
			thread::sleep(Duration::from_secs(2,),);
			let shared_state = shared_state.read().unwrap();
			if let Some(waker,) = &shared_state.waker {
				waker.wake_by_ref();
			}
		},);

		io_event_loop_thread.join().expect("thread did not complete correctly",);
	}

	#[test]
	fn executor() -> Rslt<(),> {
		let pool = ThreadPool::new()?;
		let future = async { crate::test_println!("good night") };
		pool.spawn_ok(future,);
		Ok((),)
	}

	#[test]
	#[ignore = "blocks test process"]
	fn simulate_future() {
		let pool = ThreadPool::new().expect("failed to create thread pool",);
		let shared_shate = Arc::new(RwLock::new(SharedState { waker: None, },),);
		let io_op = IOOperation { shared_state: shared_shate.clone(), };

		// schedule the future on the thread pool
		pool.spawn_ok(async {
			let rslt = io_op.await;
			crate::test_println!("Rslt: {rslt}");
		},);

		simulate_io_event_loop(shared_shate,);
	}
}
