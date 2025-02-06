//! utility for cli project
use anyhow::Result as Rslt;
use anyhow::anyhow;
use std::env::Args;
use std::io::Write;

pub trait CliParser {
	fn to_string(self,) -> String;
}

impl CliParser for Args {
	///On default, rust's `std::env::args()` returns literally **command line input**.
	///This means if you type `cn --lib tntn`, args() is equal to
	///
	///`"cn --lib tntn".to_string().split_whitespace()`
	///Calling `to_string()` method, this return value is equal to
	///
	///`"--lib tntn".to_string()`
	fn to_string(mut self,) -> String {
		self.next();
		let arg_string: String = self.collect();
		arg_string
	}
}

pub fn experiment_exit_code(min: i32, max: i32,) -> Rslt<Vec<i32,>,> {
	let path = "/tmp/experiment_exit_code.c";
	let path_executable = "/tmp/experiment_exit_code";

	let mut exitcodes = vec![];
	for i in min..max {
		// clear contents on open
		let mut c_source_file_hndlr =
			std::fs::OpenOptions::new().write(true,).create(true,).truncate(true,).open(path,)?;

		let c_code = format!(
			r#"
			int main() {{
				return {i};
			}}
			"#
		);

		c_source_file_hndlr.write_all(c_code.as_bytes(),)?;

		// compile generated c code
		let compile_c_code = std::process::Command::new("clang",)
			.arg(path,)
			.arg("-o",)
			.arg(path_executable,)
			.status()?;

		let exitcode = if compile_c_code.success() {
			// run executable
			std::process::Command::new(path_executable,)
				.status()
				.unwrap_or_else(|_| panic!("execution of `{path_executable}` has failed"),)
				.code()
				.expect("process terminated by signal",)
		} else {
			//  TODO: should have proper error handling
			return Err(anyhow!("compilation has failed with index: {i}"),);
		};

		exitcodes.push(exitcode,);
	}

	Ok(exitcodes,)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	#[ignore = "takes too long time"]
	fn test_experiment_exit_code() -> Rslt<(),> {
		let min = -128;
		let max = 127;
		let answer = ((256 + min)..256).chain(0..=max,);
		let rslt = experiment_exit_code(min, max,)?;

		answer
			.zip(rslt,)
			.enumerate()
			.for_each(|(i, (a, r,),)| assert_eq!(a, r, "with index: {i}"),);
		Ok((),)
	}
}
