pub trait Itr: Iterator {
	fn split(&mut self) -> impl Iterator;
}
