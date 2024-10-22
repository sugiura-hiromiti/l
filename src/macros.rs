//!My custom Macros

#[macro_export]
///Return stdin
///`cin!() //no args required`
macro_rules! cin {
   ($($stdout:expr)?) => {{
       $(
           let pl = $stdout;
           println!("{pl}");
        )?
      let mut buf = String::new();
      std::io::stdin()
         .read_line(&mut buf,).expect("error from mylibrary::marcos::cin!: failed at read_line",);
     buf.trim().to_string()
   }};
}

#[macro_export]
/// execute `$cmd`.
/// # Return
///
/// this macro returns `anyhow::Result<Option<Output>>` `Output == std::process::Output`
macro_rules! sh_cmd {
	($cmd:expr, $args:expr) => {{
		use anyhow::anyhow;
		if $cmd != "cd" {
			let mut cmd = std::process::Command::new($cmd,);
			cmd.args($args,);
			match cmd.output() {
				Err(e,) => Err(anyhow!("{e}"),),
				Ok(o,) => Ok(Some(o,),),
			}
		} else {
			match std::env::set_current_dir(&$args.last().unwrap(),) {
				Ok((),) => Ok(None,),
				Err(e,) => Err(anyhow!("{e}"),),
			}
		}
	}};

	($cmd:expr) => {{
		use anyhow::anyhow;
		if $cmd == "cd" {
			match std::env::set_current_dir(
				std::env::var("HOME",).expect("|>env_var $HOME not found",),
			) {
				Ok((),) => Ok(None,),
				Err(e,) => Err(anyhow!("{e}"),),
			}
		} else {
			let mut cmd = std::process::Command::new($cmd,);
			match cmd.output() {
				Ok(o,) => Ok(Some(o,),),
				Err(e,) => Err(anyhow!("{e}"),),
			}
		}
	}};
}

#[macro_export]
macro_rules! test_print {
	($exp:expr) => {{
		use std::io::Write;
		writeln!(std::io::stdout().lock(), $exp).unwrap();
	}};
}

#[macro_export]
macro_rules! test_eprint {
	($exp:expr) => {{
		use std::io::Write;
		writeln!(std::io::stderr().lock(), $exp).unwrap();
	}};
}

#[cfg(test)]
mod tests {
	use super::*;

	/*
	   #[test]
	   fn cin_tes() { let a = cin!(); }
	*/

	#[test]
	fn test_with_stdout() {
		test_print!("🫠 from `crate::macros::tests::test_with_stdout`");
	}
	#[test]
	fn test_with_stderr() {
		test_eprint!("🫠 from `crate::macros::tests::test_with_stderr`");
	}
}
