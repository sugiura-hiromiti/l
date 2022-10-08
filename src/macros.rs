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
   ($cmd:expr,$arg:literal)=>{
      let cmd_name = $cmd.as_bytes();
      if cmd_name==b"cd"{
         let _cd_rslt=      std::env::set_current_dir($arg).expect("**cd failed**");
      }else{
         let mut cmd = std::process::Command::new($cmd,);
         cmd.arg($arg);
         //show execution result
         let output=cmd.output().unwrap();
         println!("\n |{cmd_name:?}: {}\n", output.status,);
      {
         use std::io::Write;
         std::io::stdout().write(&output.stdout,).unwrap();
         std::io::stderr().write(&output.stderr,).unwrap();
      };
   }
};

   ($cmd:literal)=>{
       let cmd_name=$cmd.as_bytes();
       if cmd_name==b"cd"{
          let _cd_rslt=std::env::set_current_dir("~");
       }else{
           let o=std::process::Command::new(cmd_name).output().unwrap();
           {
               use std::io::{self,Write};
               stdout().write(&output.stdout).unwrap();
               stderr().write(&output.stderr).unwrap();
           };
       }
   };

   ($cmd:literal, $($args:expr)?) => {
      let cmd_name = $cmd.as_bytes();
      let mut cmd = std::process::Command::new($cmd,);
      $(
          cmd.args($args);
       )?
      //show execution result
      let output=cmd.output().unwrap();
      println!("\n |{cmd_name:?}: {}\n", output.status,);
      {
          use std::io::Write;
                std::io::stdout().write(&output.stdout,).unwrap();
                std::io::stderr().write(&output.stderr,).unwrap();

      };
   };


}
