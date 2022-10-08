//! utility for cli project
use std::env::Args;

pub trait CliParser {
   fn to_string(self,) -> String;
}

impl CliParser for Args {
   ///On default, rust's `std::env::args()` returns literally **command line input**.  
   ///This means if you type `cn --lib tntn`, args() is equal to
   ///
   ///`"cn --lib tntn".to_string().split_whitespace()`
   ///Calling `to_string()` method, this return value is equal to
   ///
   ///`"--lib tntn".to_string()`
   fn to_string(mut self,) -> String {
      self.next();
      let arg_string: String = self.collect();
      arg_string
   }
}
