//#![allow(unused)]

pub trait Size {
	fn size(&self) -> usize;
}
impl Size for usize {
	fn size(&self) -> usize {
		*self
	}
}
impl Size for MyUsize {
	fn size(&self) -> usize {
		self.0
	}
}

#[derive(Clone)]
pub struct MyStr(String);
impl<S: Size> std::ops::Index<S> for MyStr {
	type Output = str;

	fn index(&self, index: S) -> &Self::Output {
		&self.0[index.size()..=index.size()]
	}
}

#[derive(PartialOrd, PartialEq, Clone, Copy)]
pub struct MyUsize(usize);

impl std::ops::RangeBounds<usize> for MyUsize {
	fn start_bound(&self) -> std::ops::Bound<&usize> {
		std::ops::Bound::Included(&self.0)
	}

	fn end_bound(&self) -> std::ops::Bound<&usize> {
		std::ops::Bound::Included(&self.0)
	}
}

pub trait CustomMul {
	fn mul(&self, times: char) -> String;
}
impl CustomMul for str {
	fn mul(&self, times: char) -> String {
		let r = times as u8 - b'0';
		match self {
			"0" => 0,
			"1" => r,
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
