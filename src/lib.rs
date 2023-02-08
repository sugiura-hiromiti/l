//! My custom Library of rust
pub mod algorithm;
pub mod cli;
mod macros;
pub mod sh;
pub mod str;

#[allow(unused)]
fn tmp() -> anyhow::Result<i32,> {
	Ok::<i32, anyhow::Error,>(1,)?;
	Err(anyhow::anyhow!("hello"),)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn macros_sh_cmd() {
		sh_cmd!("cd", "is cur ./".split_whitespace()).unwrap();
		sh_cmd!("ls", ["-a"]).unwrap();
	}

	#[test]
	fn algo_palindrome() {
		assert_eq!(algorithm::longest_palindrome("ahy".to_string()), "a".to_string());
		assert_eq!(algorithm::longest_palindrome("(0v0)".to_string()), "0v0".to_string());
	}
}
