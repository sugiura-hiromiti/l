//!My custom Macros

#[macro_export]
///Return stdin
macro_rules! cin {
   ($($inp:literal),+) => {{
      "export stdin"
   }};
}

#[macro_export]
///Macro for shell command io
macro_rules! sh_cmd {
   ($cmd:literal, $($arg:expr),*) => {
      let cmd_name = $cmd;
      let mut cmd = std::process::Command::new($cmd,);
      $(
          cmd.arg($arg);
       )*

      let output=cmd.output().unwrap();
      println!("\n |{cmd_name}: {}\n", output.status,);
      {
          use std::io::Write;
                std::io::stdout().write(&output.stdout,).unwrap();
                std::io::stderr().write(&output.stderr,).unwrap();

      };
   };
}
