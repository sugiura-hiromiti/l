use std::env::set_current_dir;
use std::io;

pub fn cd(path: String,) -> io::Result<(),> {
   set_current_dir(path.clone(),)?;
   println!("\n|cd: moved to {path}");
   Ok((),)
}
