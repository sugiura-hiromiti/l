//!My custom Macros

macro_rules! cin{
    ($($inp: literal),+)=>{}
}

///Macro for shell command io
macro_rules! sh_cmd {
   ($cmd:literal, $args:expr, $expect:literal) => {
      let output = std::process::Command::new($cmd,).args($args,).output().expect($expect,);
      let cmd_name = $cmd;
      println!("\n |{cmd_name}: {}\n", output.status,);
      std::io::stdout().write(&output.stdout,).unwrap();
      std::io::stderr().write(&output.stderr,).unwrap();
   };
}

