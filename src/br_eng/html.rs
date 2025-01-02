#![allow(unused)]
use crate::{dom, Parser};

///Parse a HTML document and return the root element.
pub fn parse(src: String,) -> dom::Node {
   let mut nodes = Parser { pos: 0, inp: src, }.parse_nodes();
   //If the document contains a roo element, just return it. Otherwise, crete one
   if nodes.len() == 1 {
      nodes.swap_remove(0,)
   } else {
      dom::elem("html".to_string(), dom::AttrMap::new(), nodes,)
   }
}

impl Parser {
   ///Parse a tag or attribute name.
   fn parse_tag_name(&mut self,) -> String {
      self.cnsm_while(|c| match c {
         'a'..='z' | 'A'..='Z' | '0'..='9' => true,
         _ => false,
      },)
   }

   ///Parse a single node.
   fn parse_node(&mut self,) -> dom::Node {
      match self.next_char() {
         '<' => self.parse_element(),
         _ => self.parse_text(),
      }
   }

   ///parse a text node.
   fn parse_text(&mut self,) -> dom::Node { dom::text(self.cnsm_while(|c| c != '<',),) }

   ///parse a single element, including its open tag, contents and closing tag.
   fn parse_element(&mut self,) -> dom::Node {
      assert_eq!(self.cnsm_chr(), '<'); //Opening tag.
      let tag_name = self.parse_tag_name();
      let attrs = self.parse_attributes();
      assert_eq!(self.cnsm_chr(), '>');
      let children = self.parse_nodes(); //Contents
      assert_eq!(self.cnsm_chr(), '<'); //Closing tag.
      assert_eq!(self.cnsm_chr(), '/');
      assert_eq!(self.parse_tag_name(), tag_name);
      assert_eq!(self.cnsm_chr(), '>');
      dom::elem(tag_name, attrs, children,)
   }

   ///Parse a single name="value" pair.
   fn parse_attr(&mut self,) -> (String, String,) {
      let name = self.parse_tag_name();
      assert_eq!(self.cnsm_chr(), '=');
      let val = self.parse_attr_value();
      (name, val,)
   }

   ///Parse a quoted value.
   fn parse_attr_value(&mut self,) -> String {
      let open_quote = self.cnsm_chr();
      assert!(open_quote == '"' || open_quote == '\'');
      let val = self.cnsm_while(|c| c != open_quote,);
      assert_eq!(self.cnsm_chr(), open_quote);
      val
   }

   ///Parse a list of name="value" pairs, separated by whitespace.
   fn parse_attributes(&mut self,) -> dom::AttrMap {
      let mut attrs = dom::AttrMap::new();
      loop {
         self.cnsm_whitespace();
         if self.next_char() == '>' {
            break;
         }
         let (nam, val,) = self.parse_attr();
         attrs.insert(nam, val,);
      }
      attrs
   }

   ///Parse a sequence of sibling nodes.
   fn parse_nodes(&mut self,) -> Vec<dom::Node,> {
      let mut nodes = vec![];
      loop {
         self.cnsm_whitespace();
         if self.eof() || self.starts_with("</",) {
            break;
         }
         nodes.push(self.parse_node(),);
      }
      nodes
   }
}
