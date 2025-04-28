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
	use o_o_proc_macro::bench_fn_call_with_more_than_8_args;
	use o_o_proc_macro::bench_for_all_integers;
	use test::Bencher;
	use test::TestFn::DynBenchFn;
	use test::black_box;

	const N: usize = 100;

	struct Size128 {
		f1:  u8,
		f2:  u8,
		f3:  u8,
		f4:  u8,
		f5:  u8,
		f6:  u8,
		f7:  u8,
		f8:  u8,
		f9:  u8,
		f10: u8,
		f11: u8,
		f12: u8,
		f13: u8,
		f14: u8,
		f15: u8,
		f16: u8,
	}

	struct Size128_2 {
		f1: u16,
		f2: u16,
		f3: u16,
		f4: u16,
		f5: u16,
		f6: u16,
		f7: u16,
		f8: u16,
	}

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

	fn one_param_with_size128(p: Size128,) {}

	#[bench]
	fn bench_one_param_with_size128(b: &mut Bencher,) {
		b.iter(|| {
			for _ in 0..N {
				one_param_with_size128(black_box(Size128 {
					f1:  0,
					f2:  0,
					f3:  0,
					f4:  0,
					f5:  0,
					f6:  0,
					f7:  0,
					f8:  0,
					f9:  0,
					f10: 0,
					f11: 0,
					f12: 0,
					f13: 0,
					f14: 0,
					f15: 0,
					f16: 0,
				},),);
			}
		},);
	}

	fn one_param_with_size128_2(p: Size128_2,) {}

	#[bench]
	fn bench_one_param_with_size128_2(b: &mut Bencher,) {
		b.iter(|| {
			for _ in 0..N {
				one_param_with_size128_2(black_box(Size128_2 {
					f1: 0,
					f2: 0,
					f3: 0,
					f4: 0,
					f5: 0,
					f6: 0,
					f7: 0,
					f8: 0,
				},));
			}
		},);
	}

	fn one_param_with_size64(p: Size64,) {}

	#[bench]
	fn bench_one_param_with_size64(b: &mut Bencher,) {
		b.iter(|| {
			for _ in 0..N {
				one_param_with_size64(black_box(Size64 {
					f1: 0,
					f2: 0,
					f3: 0,
					f4: 0,
					f5: 0,
					f6: 0,
					f7: 0,
					f8: 0,
				},),);
			}
		},);
	}

	fn one_param_with_size64_2(p: Size64_2,) {}

	#[bench]
	fn bench_one_param_with_size64_2(b: &mut Bencher,) {
		b.iter(|| {
			for _ in 0..N {
				one_param_with_size64_2(black_box(Size64_2 { f1: 0, f2: 0, f3: 0, f4: 0, },),);
			}
		},);
	}
	/// # Params
	///
	/// takes reference of `impl Integer`
	///
	/// consider the case that concrete type of p is u8.
	/// size of `u8` is 1
	/// size of `&u8` is 8 (equals to size of `usize`)
	fn take_ref(p: &impl Integer,) {}
	fn take_mut_ref(p: &mut impl Integer,) {}
	fn take_copy(p: impl Integer,) {}
	fn take_mut_copy(mut p: impl Integer,) {}

	// TODO: bench &mut
	bench_for_all_integers!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, usize, isize);

	// TODO: bench fn call with more than 8 arguments
	bench_fn_call_with_more_than_8_args!(u8);
	bench_fn_call_with_more_than_8_args!(u16);
	bench_fn_call_with_more_than_8_args!(u128);

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
