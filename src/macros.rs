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
         .read_line(&mut buf,)
         .expect("error from mylibrary::marcos::cin!: failed at read_line",);
      buf
   }};
}

#[macro_export]
/// execute `$cmd`.
/// # Return
///
/// this macro returns `std::io::Result<()>`
macro_rules! sh_cmd {
	($cmd:expr, $args:expr) => {{
		if $cmd != "cd" {
			let mut cmd = std::process::Command::new($cmd,);
			cmd.args($args,);
			cmd.output()
		} else {
			match std::env::set_current_dir(&$args.last().unwrap(),) {
				Ok((),) => Err(std::io::Error::new(std::io::ErrorKind::Other, "cd succeed",),),
				Err(e,) => panic!("{e}"),
			}
		}
	}};

	($cmd:expr) => {{
		if $cmd == "cd" {
			match std::env::set_current_dir(
				std::env::var("HOME",).expect("|>env_var $HOME not found",),
			) {
				Ok((),) => Err(std::io::Error::new(std::io::ErrorKind::Other, "cd succeed",),),
				Err(e,) => panic!("{e}"),
			}
		} else {
			let mut cmd = std::process::Command::new($cmd,);
			cmd.output()
		}
	}};
}
