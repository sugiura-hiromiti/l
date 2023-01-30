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
/// # Params
/// `sh_cmd!($cmd:literal, $args:expr)`:
/// - $cmd: command itself
/// - $args: iterator of arguments to be passed to `$cmd`
/// # Return
/// `sh_cmd` returns io::Result<Output>
/// # Explanation
/// Execute shell command. Then show result.
macro_rules! sh_cmd {
	($cmd:expr, $args:expr) => {
		let mut cmd = std::process::Command::new($cmd,);
		cmd.args($args,);

		//return execution result
		cmd.output()
	};
}
