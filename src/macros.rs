//!My custom Macros

#[macro_export]
///Return stdin
macro_rules! cin {
   ($($inp:literal),+) => {{
      let mut buf = String::new();
      std::io::stdin()
         .read_line(&mut buf,)
         .expect("error from mylibrary::marcos::cin!: failed at read_line",);
      buf
   }};
}

#[macro_export]
///```rust
/// //return ()
/// sh_cmd!($cmd:literal, $($arg:expr),* $($args:block),*)
/// ````
///>Execute shell command. Then show result.
///>This macro doesn't work with
macro_rules! sh_cmd {
   ($cmd:literal, $($arg:expr),* $($args:block),*) => {
      let cmd_name = $cmd;
      let mut cmd = std::process::Command::new($cmd,);
      $(
          cmd.arg($arg);
       )*
      $(
          cmd.args($args);
       )*
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
