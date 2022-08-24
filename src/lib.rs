//! My custom Library of rust
pub mod macros;

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn macros_sh_cmd() {
      sh_cmd!("ls", "-a");
      sh_cmd!("ls", "-a", "--color=auto");
      sh_cmd!("ls", "-l", "-a", "--color=auto");
      sh_cmd!("ls", "-a");
   }
}
