mod css;
mod dom;
mod html;
mod layout;
mod painting;
mod style;

use std::fs;

use image;

struct Parser {
   pos: usize,
   inp: String,
}

impl Parser {
   ///Read the current character without consuming it.
   pub fn next_char(&self,) -> char { self.inp[self.pos..].chars().next().unwrap() }

   ///Do the next characters start with the given string?
   pub fn starts_with(&self, s: &str,) -> bool { self.inp[self.pos..].starts_with(s,) }

   ///Return true if all input is consumed.
   pub fn eof(&self,) -> bool { self.pos >= self.inp.len() }

   ///Return the current character, and advance self.pos to the next character.
   pub fn cnsm_chr(&mut self,) -> char {
      let mut itr = self.inp[self.pos..].char_indices();
      let (_, cur_chr,) = itr.next().unwrap();
      let (next_pos, _,) = itr.next().unwrap_or((1, ' ',),);
      self.pos += next_pos;
      cur_chr
   }

   ///Consume characters until 'test' returns false.
   pub fn cnsm_while(&mut self, test: impl Fn(char,) -> bool,) -> String {
      let mut rslt = String::new();
      while !self.eof() && test(self.next_char(),) {
         rslt.push(self.cnsm_chr(),);
      }
      rslt
   }

   ///Consume and discard zero or more whitespace characters.
   pub fn cnsm_whitespace(&mut self,) { self.cnsm_while(char::is_whitespace,); }
}

fn main() {
   //read input files
   let html_file = fs::read_to_string("examples/test.html",).unwrap();
   let css_file = fs::read_to_string("examples/test.css",).unwrap();
   //Since we don't have an actual window, hardcode the 'viewport' size
   let mut viewport: layout::Dimensions = Default::default();
   viewport.content.width = 800.0;
   viewport.content.height = 600.0;
   //Parse and rendering
   let root_node = html::parse(html_file,);
   let stylesheet = css::parse(css_file,);
   let style_root = style::style_tree(&root_node, &stylesheet,);
   let layout_root = layout::layout_tree(&style_root, viewport.clone(),);
   //Create output file
   let board = painting::paint(&layout_root, viewport.content,);
   let (w, h,) = (board.width as u32, board.height as u32,);
   let img = image::ImageBuffer::from_fn(w, h, |x, y| {
      let clr = &board.pixels[(y * w + x) as usize];
      image::Rgba([clr.r, clr.g, clr.b, clr.a,],)
   },);
   img.save("output.png",).unwrap();
}
