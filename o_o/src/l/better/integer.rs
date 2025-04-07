use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Sub;
use std::ops::SubAssign;

#[o_o_proc_macro::empty_trait_impl_block(
	u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, usize, isize
)]
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
	+ Copy
	+ Sized
{
}
