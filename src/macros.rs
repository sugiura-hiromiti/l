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
/// `sh_cmd!($cmd:literal, $($args:expr)?)` return value is ()
///>Execute shell command. Then show result.
///>This macro doesn't work with `cd` command
macro_rules! sh_cmd {
   ($cmd:literal, $($args:expr)?) => {
      let cmd_name = $cmd;
      let mut cmd = std::process::Command::new($cmd,);
      $(
          cmd.args($args);
       )?
      //show execution result
      let output=cmd.output().unwrap();
      println!("\n |{cmd_name}: {}\n", output.status,);
      {
          use std::io::Write;
                std::io::stdout().write(&output.stdout,).unwrap();
                std::io::stderr().write(&output.stderr,).unwrap();

      };
   };
}
