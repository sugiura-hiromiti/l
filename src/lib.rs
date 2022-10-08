//! My custom Library of rust
mod macros;

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn macros_sh_cmd() {
      sh_cmd!("ls", ["-a"].iter());
      sh_cmd!("ls", ["-a", "--color=auto"].iter());
      sh_cmd!("ls", ["-l", "-a", "--color=auto"].iter());
      sh_cmd!("ls",);
   }

   fn by() {
   }
}
