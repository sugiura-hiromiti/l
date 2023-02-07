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
/// this macro returns `anyhow::Result<Option<Output>>` `Output == std::process::Output`
macro_rules! sh_cmd {
	($cmd:expr, $args:expr) => {{
		if $cmd != "cd" {
			let mut cmd = std::process::Command::new($cmd,);
			cmd.args($args,);
			match cmd.output() {
				Err(e,) => Err(anyhow::anyhow!("{e}"),),
				Ok(o,) => Ok(Some(o,),),
			}
		} else {
			match std::env::set_current_dir(&$args.last().unwrap(),) {
				Ok((),) => Ok(None,),
				Err(e,) => Err(anyhow::anyhow!("{e}"),),
			}
		}
	}};

	($cmd:expr) => {{
		if $cmd == "cd" {
			match std::env::set_current_dir(
				std::env::var("HOME",).expect("|>env_var $HOME not found",),
			) {
				Ok((),) => Ok(None,),
				Err(e,) => Err(anyhow::anyhow!("{e}"),),
			}
		} else {
			let mut cmd = std::process::Command::new($cmd,);
			match cmd.output() {
				Ok(o,) => Ok(Some(o,),),
				Err(e,) => Err(anyhow::anyhow!("{e}"),),
			}
		}
	}};
}
