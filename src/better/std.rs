pub trait Iter: Iterator {
	fn split(&mut self,) -> impl Iterator;
}
