//! My custom Library of rust
pub mod algorithm;
pub mod cli;
mod macros;
pub mod sh;
pub mod str;

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn macros_sh_cmd() {
		let rslt = sh_cmd!("exa", ["--color=never"]).unwrap();
		println!("{rslt:?}");
	}

	#[test]
	fn algo_palindrome() {
		assert_eq!(algorithm::longest_palindrome("ahy".to_string()), "a".to_string());
		assert_eq!(algorithm::longest_palindrome("(0v0)".to_string()), "0v0".to_string());
	}
}
