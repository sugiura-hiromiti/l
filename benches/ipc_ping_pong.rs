//! TODO:
//! [source](https://pranitha.rs/posts/rust-ipc-ping-pong/)

fn main() {
	divan::main();
}

#[divan::bench]
fn stdio(_b: divan::Bencher,) {
	let _n = 1000;
}
