//! this module experiments how arguments of function affects function call performance
//!
//! # ref or copy
//!
//! generally, passing reference is guaranteed in terms of speed
//! for large data type. but is it also true for smaller data type?
//! especially for types which size is smaller than size of reference
//! (i.e. u8, u16, u32, i8 ...)
//! and which size is same to reference (i.e. usize, (u8,u8,u8,u8), )
//!
//! # number of parameters
//!
//! ---

#![feature(test)]

extern crate test;

use o_o::l::better::integer::Integer;

#[cfg(test)]
mod tests {
	use super::*;
	use o_o_proc_macro::bench_for_all_integers;
	use test::Bencher;
	use test::black_box;

	const N: usize = 100;

	struct Size64 {
		f1: u8,
		f2: u8,
		f3: u8,
		f4: u8,
		f5: u8,
		f6: u8,
		f7: u8,
		f8: u8,
	}

	struct Size64_2 {
		f1: u16,
		f2: u16,
		f3: u16,
		f4: u16,
	}

	struct Size32 {
		f1: u8,
		f2: u8,
		f3: u8,
		f4: u8,
	}
	struct Size32_2 {
		f1: u16,
		f2: u16,
	}

	/// # Params
	///
	/// takes reference of `impl Integer`
	///
	/// consider the case that concrete type of p is u8.
	/// size of `u8` is 1
	/// size of `&u8` is 8 (equals to size of `usize`)
	fn take_ref(p: &impl Integer,) {
		let _p = p;
	}

	fn take_copy(p: impl Integer,) {
		let _p = p;
	}

	// TODO: bench &mut
	bench_for_all_integers!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, usize, isize);

	// TODO: bench fn call with more than 8 arguments

	// fn pass_ref_u8(b: &mut Bencher,) {
	// 	b.iter(|| {
	// 		for i in 0..(N as int) {
	// 			take_ref(black_box(&i,),);
	// 		}
	// 	},);
	// }

	// #[bench]
	// fn pass_u8(b: &mut Bencher,) {
	// 	b.iter(|| {
	// 		for i in 0..(N as u8) {
	// 			take_copy(black_box(i,),);
	// 		}
	// 	},);
	// }
}
