//! this module experiments speed of function call with argument
//! generally, passing reference is guaranteed in terms of speed
//! for large data type. but is it also true for smaller data type?
//! especially for types which size is smaller than size of reference
//! (i.e. u8, u16, u32, i8 ...)
//! and which size is same to reference (i.e. usize, (u8,u8,u8,u8), )

#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
	use super::*;
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
	/// takes reference of `u8`
	/// size of `u8` is 1
	/// size of `&u8` is 8 (equals to size of `usize`)
	fn take_ref_u8(p: &u8,) {
		let _p_pow2 = p * p;
	}

	fn take_u8(p: u8,) {
		let _p_pow2 = p * p;
	}

	#[bench]
	fn pass_ref_u8(b: &mut Bencher,) {
		b.iter(|| {
			for i in 0..(N as u8) {
				take_ref_u8(black_box(&i,),);
			}
		},);
	}

	#[bench]
	fn pass_u8(b: &mut Bencher,) {
		b.iter(|| {
			for i in 0..(N as u8) {
				take_u8(black_box(i,),);
			}
		},);
	}
}
