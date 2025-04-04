/*!
# result of bench
```sh
cargo bench
test tests::all_in_once_with_byte ... bench:         295.06 ns/iter (+/- 3.14)
test tests::all_in_once           ... bench:         369.95 ns/iter (+/- 3.95)
test tests::buffering             ... bench:         751.14 ns/iter (+/- 6.97)
test tests::normal_o              ... bench:       1,288.46 ns/iter (+/- 55.03)
test tests::lockless              ... bench:      27,148.77 ns/iter (+/- 213.89)
test tests::no_ln                 ... bench:      27,165.60 ns/iter (+/- 216.85)
```
---
*/

#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
	use super::*;
	use std::io::BufWriter;
	use std::io::Write;
	use std::io::stderr;
	use test::Bencher;

	const N: usize = 100;

	#[bench]
	fn normal_o(b: &mut Bencher,) {
		b.iter(|| {
			(0..N).for_each(|_| {
				eprintln!("yes");
			},);
		},);
	}

	#[bench]
	fn lockless(b: &mut Bencher,) {
		b.iter(|| {
			let mut err = stderr().lock();
			(0..N).for_each(|_| {
				writeln!(err, "yes").unwrap();
			},);
		},);
	}

	#[bench]
	fn no_ln(b: &mut Bencher,) {
		b.iter(|| {
			let mut err = stderr().lock();
			(0..N).for_each(|_| {
				write!(err, "yes").unwrap();
			},);
		},);
	}

	#[bench]
	fn buffering(b: &mut Bencher,) {
		b.iter(|| {
			let mut err = BufWriter::new(stderr().lock(),);
			(0..N).for_each(|_| {
				write!(err, "yes").unwrap();
			},);
		},);
	}

	#[bench]
	fn all_in_once(b: &mut Bencher,) {
		b.iter(|| {
			let mut yes = String::with_capacity(N * 3,);
			(0..N).for_each(|_| {
				yes += "yes";
			},);

			assert_eq!(yes.len(), yes.capacity());
			BufWriter::new(stderr().lock(),).write_all(yes.as_bytes(),).unwrap();
		},);
	}

	#[bench]
	fn all_in_once_with_byte(b: &mut Bencher,) {
		b.iter(|| {
			let mut yes = [0; 300];
			(0..N).for_each(|i| {
				yes[i * 3] = b'y';
				yes[i * 3 + 1] = b'e';
				yes[i * 3 + 2] = b's';
			},);

			BufWriter::new(stderr().lock(),).write_all(&yes,).unwrap();
		},);
	}
}
