pub trait Unwrap {
	fn unwrap(self,) -> Self;
}

pub trait Wrap<T,> {
	type Wrapped = T;
	fn wrap(value: Self::Wrapped,) -> Self;
}

pub trait Container<T,>: Wrap<T,> + Unwrap {}
