use std::env::set_current_dir;
use std::io;

pub fn cd<P,>(path: P,) -> io::Result<(),>
where P: AsRef<std::path::Path,> + Clone + std::fmt::Display {
	set_current_dir(path.clone(),)?;
	println!("\n|cd: moved to {path}");
	Ok((),)
}
