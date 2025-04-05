#![allow(dead_code)]

struct Solution;
// NOTE: ideas
// 1. emulate scramble & find out we actually get s2(bit tree)
// 2. determine scrambling possibility by indices of chars of s1 & s2
// 3. insert splitter(represent as `|` here) and emulate scramble with splitter rule
// 4. 漸化式作れないか？
type CharMap = std::collections::HashMap<char, Vec<usize,>,>;
impl Solution {
	/// # Parameter
	/// assume that length of s1 & s2 is the same, length of s1 is longer than 0
	/// & shorter than 31, s1 & s2 only contains lowercase English letters
	pub fn is_scramble(s1: String, s2: String,) -> bool {
		let mut map_one = Self::create_char_map(&s1,);
		let mut map_two = Self::create_char_map(&s2,);

		// let mut scenarios:Vec<(usize, u8/*this is bit flag*/)>=vec![];
		// determine is valid s2 indices mapping or not
		//		s2.char_indices()

		let (mut l1, mut l2, len,) = (0, 0, s1.len(),);
		todo!()
	}

	fn create_char_map(s: &String,) -> CharMap {
		let mut map = std::collections::HashMap::<char, Vec<usize,>,>::new();
		s.char_indices().for_each(|(u, c,)| {
			map.entry(c,).and_modify(|v| v.push(u,),).or_insert(vec![u],);
		},);
		map
	}
}

// abde (a bde - a e bd) aebd
// bcde cedb
//
// 12345 45312

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn code_test() {
		let s1 = "aabbcc".to_string();
		let map_one = std::collections::HashMap::<char, usize,>::from_iter(
			s1.char_indices().map(|(i, c,)| (c, i,),),
		);
		println!("{map_one:?}");
		map_one.keys().for_each(|k| {
			let value = match k {
				'a' | 'b' | 'c' => *map_one.get(k,).unwrap(),
				_ => 0,
			};
			assert_eq!(value, 2);
		},);
	}

	#[test]
	fn hashmap_eq() {
		let map_one = Solution::create_char_map(&"11ii3ab".to_string(),);
		let map_two = Solution::create_char_map(&"11ii3ab".to_string(),);
		let map_three = Solution::create_char_map(&"ba3i1".to_string(),);
		assert_eq!(map_one, map_two);
		assert_ne!(map_one, map_three);
	}

	#[test]
	fn test_1() {
		let ans = true;
		let sol = Solution::is_scramble("great".to_owned(), "rgeat".to_owned(),);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_2() {
		let ans = false;
		let sol = Solution::is_scramble("abcde".to_string(), "caebd".to_string(),);
		assert_eq!(ans, sol);
	}

	#[test]
	fn test_3() {
		let ans = true;
		let sol = Solution::is_scramble("a".to_string(), "a".to_string(),);
		assert_eq!(ans, sol);
	}
}
