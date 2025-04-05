//! My custom Library of rust
#![feature(associated_type_defaults)]
#![feature(pattern, never_type, file_buffered, iterator_try_collect)]
#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(impl_trait_in_assoc_type)]
#![feature(str_from_utf16_endian)]
#![allow(unused_doc_comments)]

pub mod l;

#[cfg(test)]
mod tests {
	use super::*;
	use anyhow::Result as Rslt;

	#[test]
	fn macros_sh_cmd() {
		sh_cmd!("cd", "is cur ./".split_whitespace()).unwrap();
		sh_cmd!("ls", ["-a"]).unwrap();
	}

	#[test]
	fn algo_palindrome() {
		assert_eq!(l::utl::algorithm::longest_palindrome("ahy".to_string()), "a".to_string());
		assert_eq!(l::utl::algorithm::longest_palindrome("(0v0)".to_string()), "0v0".to_string());
	}

	#[test]
	fn moku() -> Rslt<(),> {
		let moku = String::from_utf8(vec![
			227, 130, 130, 227, 129, 143, 227, 130, 130, 227, 129, 143, 227, 129, 151, 227, 129,
			190, 227, 129, 153,
		],)?;
		assert_eq!("もくもくします", moku);

		let my_fav_lang = unsafe {
			String::from_utf8_unchecked(vec![
				82, 117, 115, 116, 227, 129, 168, 108, 117, 97, 227, 129, 140, 229, 165, 189, 227,
				129, 141, 227, 129, 167, 227, 130, 136, 227, 129, 143, 232, 167, 166, 227, 129,
				163, 227, 129, 166, 227, 129, 132, 227, 129, 190, 227, 129, 153, 32, 229, 184, 131,
				230, 149, 153, 227, 129, 151, 227, 129, 159, 227, 129, 132,
			],)
		};
		assert_eq!("Rustとluaが好きでよく触っています 布教したい", my_fav_lang);
		Ok((),)
	}

	// #[test]
	// fn sqlite() -> Rslt<(),> {
	// 	let connect = rusqlite::Connection::open_in_memory()?;
	// 	connect.execute_batch(
	// 		"create table if not exists articles (
	// 			id integer primary key,
	// 			date text not null
	// 	);",
	// 	)?;
	//
	// 	let should_one = connect.execute(
	// 		"insert into articles (date) values (?1)",
	// 		&[&chrono::Local::now().timestamp(),],
	// 	)?;
	//
	// 	assert_eq!(1, should_one);
	//
	// 	Ok((),)
	// }

	#[test]
	fn understand_closure() {
		let x = 666;

		/// ---
		/// closure **syntax**
		(0..10).for_each(|i| {
			let cls = |arg| i + arg;

			assert_eq!(i + 666, cls(x));
		},);

		/// ---
		/// reproducing closure which only implements
		/// **FnOnce**
		#[derive(Clone,)]
		struct ClosureFnOnce {
			i: isize,
		}

		impl FnOnce<(isize,),> for ClosureFnOnce {
			type Output = isize;

			extern "rust-call" fn call_once(self, (args,): (isize,),) -> Self::Output {
				self.i + args
			}
		}

		let cls_fn_once_outer = ClosureFnOnce { i: x, };
		(0..10).for_each(|i| {
			assert_eq!(666, cls_fn_once_outer.i,);
			assert_eq!(666 + i, cls_fn_once_outer.clone()(i));
			// this will cause compile error because `cls_fn_once_outer` will be moved in a loop
			// `assert_eq!(1332, cls_fn_once_outer.call_once(x));`

			let cls_fn_once = ClosureFnOnce { i, };
			assert_eq!(i, cls_fn_once.i);
			assert_eq!(666 + i, cls_fn_once.clone()(x));

			// this will cause failing next `call_once` call because `ClosureFnOnce` will be moved
			// on this call. this behavior comes from `FnOnce` enforces move
			// `assert_eq!(666 + i, cls_fn_once.clone()(x));`

			assert_eq!(666 + i, cls_fn_once.clone().call_once((x,)));
			assert_eq!(i, cls_fn_once.i);
			assert_eq!(666 + i, cls_fn_once.call_once((x,)));

			// uncommenting code below cause compile error due to `cls_fn_once` is moved
			// `assert_eq!(i, cls_fn_once.i);`
		},);

		/// ---
		/// reproducing closure which implements
		/// **FnMut (that implies implementation of FnOnce exists)**
		#[derive(Clone,)]
		struct ClosureFnMut {
			i: isize,
		}

		impl FnMut<(isize,),> for ClosureFnMut {
			/// `type Output = ...` is not required because `FnOnce` is super trait of `FnMut`

			extern "rust-call" fn call_mut(&mut self, (args,): (isize,),) -> Self::Output {
				self.i += args;
				self.i
			}
		}

		/// this impl is necessary to `impl FnMut for ClosureFnMut` because FnMut takes FnOnce as a
		/// super trait
		impl FnOnce<(isize,),> for ClosureFnMut {
			type Output = isize;

			extern "rust-call" fn call_once(self, (arg,): (isize,),) -> Self::Output {
				self.i + arg
			}
		}

		impl FnMut<(),> for ClosureFnMut {
			extern "rust-call" fn call_mut(&mut self, _: (),) -> Self::Output {
				self.i *= 2;
				TryInto::<Self::Output,>::try_into(self.i,).unwrap()
			}
		}

		impl FnOnce<(),> for ClosureFnMut {
			type Output = u32;

			extern "rust-call" fn call_once(self, _: (),) -> Self::Output {
				TryInto::<Self::Output,>::try_into(self.i * 10,).unwrap()
			}
		}

		// impl ClosureFnMut {
		// 	fn static_ref<'a,>(i: isize,) -> &'a mut isize {
		// 		macro_rules! exp_as_tt {
		// 			($i:expr) => {
		// 				stringify!($i)
		// 			};
		// 		}
		//
		// 		let stringified = exp_as_tt!(i);
		// 		stringified.parse().expect("failed to parse &str as isize",)
		// 	}
		// }

		let mut store = 0;
		let mut cls_fn_mut_outer = ClosureFnMut { i: store, };
		(0..10).for_each(|i| {
			assert_eq!(store + i, cls_fn_mut_outer(i));
			assert_eq!(store + i * 2, cls_fn_mut_outer.clone().call_once((i,)));
			assert_eq!((store + i) * 2, cls_fn_mut_outer() as isize);
			store = cls_fn_mut_outer.clone().call_once((0,),);

			/// `mut` keyword is required because calling `call_mut()` means `&mut self` is assured
			let mut cls_fn_mut = ClosureFnMut { i, };
			assert_eq!(i + i, cls_fn_mut(i));
			assert_eq!(i * 3, cls_fn_mut.clone().call_once((i,)));
			assert_eq!(i * 3, cls_fn_mut.call_mut((i,)));
			assert_eq!(i * 4, cls_fn_mut.clone()(i));
			assert_eq!(i * 4, cls_fn_mut(i));
			assert_eq!(cls_fn_mut.i + i, cls_fn_mut(i));
			assert_eq!(cls_fn_mut(i), cls_fn_mut.i);

			assert_eq!(i * 6 * 2, cls_fn_mut() as isize);
			assert_eq!(cls_fn_mut.i * 2, cls_fn_mut.clone()() as isize);
			assert_eq!(cls_fn_mut.i * 2, cls_fn_mut.clone().call_mut(()) as isize);
			assert_eq!(i * 6 * 2 * 10, cls_fn_mut.clone().call_once(()) as isize);

			let mut cls_fn_mut2 = cls_fn_mut.clone();
			cls_fn_mut2.i = 666;
			assert_eq!(1332, cls_fn_mut2.call_mut(()));
			assert_eq!(13320, cls_fn_mut2.call_once(()));

			let cls_fn_wont_mut = ClosureFnMut { i: cls_fn_mut(i,), };
			assert_eq!(i * (6 * 2 + 1), cls_fn_wont_mut.i);
			assert_eq!(i * (6 * 2 + 1 + 1), cls_fn_wont_mut.clone().call_once((i,)));
			assert_eq!(i * (6 * 2 + 1), cls_fn_wont_mut.i);
		},);

		/// ---
		/// reproducing closure which implements **Fn**
		let mut s = "closure opens up further possibilities to a program".to_string();
		struct ClosureFn<'a,> {
			i:      isize,
			s:      &'a mut String,
			orig_s: String,
		}

		impl<'a,> Fn<(),> for ClosureFn<'a,> {
			extern "rust-call" fn call(&self, _: (),) -> Self::Output {
				let mut s: Vec<_,> = self.s.split_whitespace().collect();
				s.sort();
				s.join(" ",)
			}
		}

		impl<'a,> FnMut<(),> for ClosureFn<'a,> {
			extern "rust-call" fn call_mut(&mut self, _: (),) -> Self::Output {
				(0..self.i).for_each(|_| {
					self.s.push(' ',);
					self.s.push_str(self.orig_s.clone().as_str(),);
				},);
				self.s.clone()
			}
		}

		impl<'a,> FnOnce<(),> for ClosureFn<'a,> {
			type Output = String;

			extern "rust-call" fn call_once(self, _: (),) -> Self::Output {
				format!("{}{}", self.i, self.s)
			}
		}

		let orig_s = s.clone();
		let repeat = |i| {
			//let mut s_rep = "".to_string();
			(0..i).map(|_| orig_s.clone(),).collect::<Vec<String,>>().join(" ",)
		};

		(0..10).for_each(|i| {
			let mut closure_fn = ClosureFn { i, s: &mut s.clone(), orig_s: orig_s.clone(), };
			assert_eq!("a closure further opens possibilities program to up", closure_fn());
			assert_eq!(repeat(i + 1), closure_fn.call_mut(()));
			assert_eq!(format!("{i}{}", repeat(i + 1)), closure_fn.call_once(()));
		},);
		let mut closure_fn_outer = ClosureFn { i: 2, s: &mut s, orig_s: orig_s.clone(), };
		let mutated_s = closure_fn_outer.call_mut((),);
		let rep_3_s = repeat(3,);
		assert_eq!(rep_3_s, mutated_s);
		assert_eq!(format!("2{rep_3_s}"), closure_fn_outer.call_once(()));

		// we do not need manually drop because previous call of `call_once` moves
		// `closure_fn_outer`
		// drop(closure_fn_outer,);

		// closure can change environment!
		assert_eq!(rep_3_s, s);

		fn normal_function() -> i32 {
			2525
		}
		assert_eq!(2525, normal_function.call(()));
		assert_eq!(2525, normal_function.call_mut(()));
		assert_eq!(2525, normal_function.call_once(()));
	}

	#[test]
	fn ref_mut_as_fn_arg() {
		fn take_ref_mut_arg(x: &mut String,) -> &mut String {
			*x = format!("abc");
			x.push('\n',);
			x
		}

		let mut x = String::new();
		//take_ref_mut_arg(&mut x,);
		assert_eq!(take_ref_mut_arg(&mut x), "abc\n");
		let y = take_ref_mut_arg(&mut x,).clone();
		assert_eq!(x, y);

		// this will cause error
		//let y = take_ref_mut_arg(&mut x,);
		//assert_eq!(x, *y);
	}

	#[test]
	fn test_borrowing_with_different_mutability() {
		let mut a = 0;
		let mut b = &mut a;
		let c = &mut b;

		**c += 1;
		// uncommenting code below cause error
		//assert_eq!(a, 1);
		assert_eq!(*b, 1);

		*b += 1;
		assert_eq!(a, 2);

		a += 1;
		assert_eq!(a, 3);
	}

	#[test]
	fn from_str_binary_radix() -> Rslt<(),> {
		let thirty_eight = u128::from_str_radix("000100110", 2,)?;
		assert_eq!(thirty_eight, 38);

		Ok((),)
	}

	#[test]
	fn bit_and() {
		let bit_field = 0b1_001 & 0b0_001;
		assert_eq!(bit_field, 0b1);
	}

	#[test]
	fn bit_or() {
		let bit_field = 0b1_001 | 0b0_100;
		assert_eq!(bit_field, 0b1_101);
	}

	#[test]
	fn bit_left_shift() {
		let bit_field = 0b1 << 0;
		assert_eq!(bit_field, 0b1);
		let bit_field = 0b1 << 1;
		assert_eq!(bit_field, 0b10);
	}

	#[test]
	fn mod_minus() {
		assert_eq!(-2 % 10, -2);
		assert_eq!(-25 % 10, -5);
	}
}
