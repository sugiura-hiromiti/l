//#![allow(unused)]

trait Size {
	fn size(&self,) -> usize;
}
impl Size for usize {
	fn size(&self,) -> usize { *self }
}
impl Size for MyUsize {
	fn size(&self,) -> usize { self.0 }
}

#[derive(Clone,)]
struct MyStr(String,);
impl<S: Size,> std::ops::Index<S,> for MyStr {
	type Output = str;

	fn index(&self, index: S,) -> &Self::Output { &self.0[index.size()..=index.size()] }
}

#[derive(PartialOrd, PartialEq, Clone, Copy,)]
struct MyUsize(usize,);

impl std::ops::RangeBounds<usize,> for MyUsize {
	fn start_bound(&self,) -> std::ops::Bound<&usize,> { std::ops::Bound::Included(&self.0,) }

	fn end_bound(&self,) -> std::ops::Bound<&usize,> { std::ops::Bound::Included(&self.0,) }
}

fn stoi(s: &str,) -> u32 {
	match s {
		"1" => 1,
		"2" => 2,
		"3" => 3,
		"4" => 4,
		"5" => 5,
		"6" => 6,
		"7" => 7,
		"8" => 8,
		"9" => 9,
		"0" => 0,
		_ => panic!("`s` should be digit"),
	}
}

// NOTE: if use strategy *add* -------------------------
fn add_string(l: &mut MyStr, r: &MyStr,) {
	if l.0 == "0" {
		*l = r.clone();
	} else {
		let mut moved_up = false;
		let mut li = MyUsize(l.0.len(),);
		let mut ri = Some(MyUsize(r.0.len(),),);
		while li.0 > 0 {
			li.0 -= 1;
			let tmp = (stoi(&l[li.0],)
				+ if let Some(n,) = ri.as_mut() {
					if n.0 == 0 {
						ri = None;
						0
					} else {
						n.0 -= 1;
						stoi(&r[n.0],)
					}
				} else {
					0
				} + if moved_up { 1 } else { 0 })
			.to_string();

			if tmp.len() == 1 {
				l.0.replace_range(li, &tmp,);
				moved_up = false;
			} else {
				l.0.replace_range(li, &tmp[1..=1],);
				moved_up = true;
			}
		}

		if moved_up {
			l.0.insert(0, '1',);
		}
	}
}

// NOTE: if use strategy *mul* -------------------------
fn mul_string(n: &mut String, times: char,) {
	let len = MyUsize(n.len(),);
	let mut cus_n = MyStr(n.clone(),);
	let mut moved_up = 0;
	let mut i = MyUsize(0,);
	while i < len {
		let mul = &mut MyStr(cus_n[i].mul(times,),);
		add_string(mul, &MyStr(moved_up.to_string(),),);
		if mul.0.len() == 1 {
			n.replace_range(i, &mul.0,);
		} else {
			todo!()
		}
	}
}

trait CustomMul {
	fn mul(&self, times: char,) -> String;
}
impl CustomMul for str {
	fn mul(&self, times: char,) -> String {
		let r = times as u8 - '0' as u8;
		match self {
			"0" => 0 * r,
			"1" => 1 * r,
			"2" => 2 * r,
			"3" => 3 * r,
			"4" => 4 * r,
			"5" => 5 * r,
			"6" => 6 * r,
			"7" => 7 * r,
			"8" => 8 * r,
			"9" => 9 * r,
			_ => panic!("incorrect usage of CustomMul"),
		}
		.to_string()
	}
}
