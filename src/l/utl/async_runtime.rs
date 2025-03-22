//use anyhow::Result as Rslt;
use futures::future::BoxFuture;
use futures::future::FutureExt;
use futures::task::ArcWake;
use futures::task::waker_ref;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::SyncSender;
use std::sync::mpsc::sync_channel;
use std::task::Context;
use std::task::Poll;

pub struct Night {
	state: StateNight,
}

impl Default for Night {
	fn default() -> Self {
		Self::new()
	}
}

impl Night {
	pub fn new() -> Self {
		Night {
			// initial sate
			state: StateNight::Early,
		}
	}

	fn print_state(&self) {
		let state = self.state_name();
		crate::test_eprintln!("{state}");
	}

	fn state_name(&self) -> String {
		self.state.as_ref().to_owned()
	}
}

impl Future for Night {
	type Output = ();

	fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> std::task::Poll<Self::Output> {
		use StateNight::*;
		match self.state {
			Early => {
				(*self).print_state();
				self.state = Late;
				cx.waker().wake_by_ref();
				Poll::Pending
			},
			Late => {
				(*self).print_state();
				self.state = Mid;
				cx.waker().wake_by_ref();
				Poll::Pending
			},
			Mid => {
				(*self).print_state();
				Poll::Ready(())
			},
		}
	}
}

#[derive(strum_macros::AsRefStr)]
enum StateNight {
	Early,
	Late,
	Mid,
}

// execution unit
pub struct Task {
	// executed coroutine
	future: Mutex<BoxFuture<'static, ()>>,
	// channel for schedule to executor
	sender: SyncSender<Arc<Task>>,
}

/// Scheduling Self
impl ArcWake for Task {
	fn wake_by_ref(arc_self: &Arc<Self>) {
		let self0 = arc_self.clone();
		arc_self.sender.send(self0).unwrap();
	}
}

pub struct Executor {
	// execution queue
	sender: SyncSender<Arc<Task>>,
	receiver: Receiver<Arc<Task>>,
}

impl Default for Executor {
	fn default() -> Self {
		Self::new()
	}
}

impl Executor {
	pub fn new() -> Self {
		// max amount of queued tasks is 1024
		let (sender, receiver) = sync_channel(1024);
		Self { sender: sender.clone(), receiver }
	}

	// get spawner for generate new task
	pub fn get_spawner(&self) -> Spawner {
		Spawner { sender: self.sender.clone() }
	}

	pub fn run(&self) {
		// execute a task send from spawner each time
		while let Ok(task) = self.receiver.recv() {
			// setup context
			let mut future = task.future.lock().unwrap();
			let waker = waker_ref(&task);
			let mut ctx = Context::from_waker(&waker);
			let _ = future.as_mut().poll(&mut ctx);
		}
	}
}

// generate task and send it to queue
pub struct Spawner {
	sender: SyncSender<Arc<Task>>,
}

impl Spawner {
	pub fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
		let future = future.boxed();
		let task = Arc::new(Task { future: Mutex::new(future), sender: self.sender.clone() });

		self.sender.send(task).unwrap();
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	#[ignore = "will not end execution"]
	fn executor_run() {
		let executor = Executor::new();
		executor.get_spawner().spawn(Night::new());
		executor.run();
	}
}
