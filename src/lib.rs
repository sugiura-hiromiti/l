//! My custom Library of rust
pub mod cli;
mod macros;
pub mod sh;

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn macros_sh_cmd() {
      sh_cmd!("ls", ["-a"]);
      sh_cmd!("ls", ["-a", "--color=auto"]);
      sh_cmd!("ls", ["-l", "-a", "--color=auto"]);
      //this should cause error sh_cmd!("ls", );
   }
}
