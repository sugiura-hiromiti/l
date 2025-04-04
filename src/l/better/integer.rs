use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Sub;
use std::ops::SubAssign;

pub trait Integer:
	Add
	+ AddAssign
	+ Sub
	+ SubAssign
	+ Mul
	+ MulAssign
	+ Div
	+ DivAssign
	+ PartialEq
	+ Eq
	+ PartialOrd
	+ Ord
	+ Sized
{
}

macro_rules! impl_integer {
	($ty:tt) => {
		impl Integer for $ty {};
	};
	($ty:tt, $($tys:tt),+) => {
		impl Integer for $ty {}
		impl_integer!($($tys),+);
	};
}

//impl_integer!(u8, u16,);
