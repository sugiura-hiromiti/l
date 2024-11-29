// Write code here.
//
// To see what the code looks like after macro expansion:
//     $ cargo expand
//
// To run the code:
//     $ cargo run

#[derive(derive_builder::Builder,)]
struct A {
	a: Option<i32,>,
	b: i32,
}

fn main() {}
