#![allow(unused)]

use crate::Parser;

pub type Specificity = (usize, usize, usize,);

pub struct Stylesheet {
   pub rules: Vec<Rule,>,
}

pub struct Rule {
   pub selectors:    Vec<Selector,>,
   pub declarations: Vec<Declaration,>,
}

pub enum Selector {
   Simple(SimpleSelector,),
}

impl Selector {
   ///Ranking when confilct occurs
   pub fn specificity(&self,) -> Specificity {
      let Selector::Simple(ref simple,) = *self;
      let a = simple.id.iter().count();
      let b = simple.class.len();
      let c = simple.tag_name.iter().count();
      (a, b, c,)
   }
}

pub struct SimpleSelector {
   pub tag_name: Option<String,>,
   pub id:       Option<String,>,
   pub class:    Vec<String,>,
}

pub struct Declaration {
   pub nam: String,
   pub val: Value,
}

#[derive(Clone, PartialEq,)]
pub enum Value {
   Keyword(String,),
   Length(f64, Unit,),
   ColorValue(Color,),
}

impl Value {
   pub fn to_px(&self,) -> f64 {
      match self {
         Value::Length(f, Unit::Px,) => *f,
         _ => 0.0,
      }
   }
}

#[derive(PartialEq, Clone,)]
pub enum Unit {
   Px,
}

#[derive(Clone, PartialEq,)]
pub struct Color {
   pub r: u8,
   pub g: u8,
   pub b: u8,
   pub a: u8,
}

impl Parser {
   ///Parse a list of rule sets, separated by optional whitespace.
   fn parse_rules(&mut self,) -> Vec<Rule,> {
      let mut rules = vec![];
      loop {
         self.cnsm_whitespace();
         if self.eof() {
            break;
         }
         rules.push(self.parse_rule(),);
      }
      rules
   }

   ///Parse a rule set: '<selectors>{<declarations>}'.
   fn parse_rule(&mut self,) -> Rule { Rule { selectors: self.parse_selectors(), declarations: self.parse_declarations(), } }

   ///Parse a comma-separated list of selectors.
   fn parse_selectors(&mut self,) -> Vec<Selector,> {
      let mut selectors = vec![];
      loop {
         selectors.push(Selector::Simple(self.parse_simple_selector(),),);
         self.cnsm_whitespace();
         match self.next_char() {
            '.' => {
               self.cnsm_chr();
               self.cnsm_whitespace();
            }
            '{' => break,
            c => panic!("Unexpected cahracter {c} in selector list"),
         }
      }
      //Return selectors with highest specificity first, for use in matching.
      selectors.sort_by(|st, nd| nd.specificity().cmp(&st.specificity(),),);
      selectors
   }

   ///Parse one simple seelctor, e.g.: 'type#id.class1.class2.class3'
   fn parse_simple_selector(&mut self,) -> SimpleSelector {
      let mut selector = SimpleSelector { tag_name: None, id: None, class: vec![], };
      while !self.eof() {
         match self.next_char() {
            '#' => {
               self.cnsm_chr();
               selector.id = Some(self.parse_idf(),);
            }
            '.' => {
               self.cnsm_chr();
               selector.class.push(self.parse_idf(),);
            }
            '*' => {
               //universal selector
               self.cnsm_chr();
            }
            c if valid_idf_chr(c,) => {
               selector.tag_name = Some(self.parse_idf(),);
            }
            _ => break,
         }
      }
      selector
   }

   ///Parse a list of declarations enclosed in '{...}'.
   fn parse_declarations(&mut self,) -> Vec<Declaration,> {
      assert_eq!(self.cnsm_chr(), '{');
      let mut declarations = vec![];
      loop {
         self.cnsm_whitespace();
         if self.next_char() == '}' {
            self.cnsm_chr();
            break;
         }
         declarations.push(self.parse_declaration(),);
      }
      declarations
   }

   ///Parse one '<property>: <value>;' declaration.
   fn parse_declaration(&mut self,) -> Declaration {
      let property_name = self.parse_idf();
      self.cnsm_whitespace();
      assert_eq!(self.cnsm_chr(), ':');
      self.cnsm_whitespace();
      let val = self.parse_val();
      self.cnsm_whitespace();
      assert_eq!(self.cnsm_chr(), ';');

      Declaration { nam: property_name, val, }
   }

   //Methods for parsing Value. ------------------------

   fn parse_val(&mut self,) -> Value {
      match self.next_char() {
         '0'..='9' => self.parse_length(),
         '#' => self.parse_color(),
         _ => Value::Keyword(self.parse_idf(),),
      }
   }

   fn parse_length(&mut self,) -> Value { Value::Length(self.parse_float(), self.parse_unit(),) }

   fn parse_float(&mut self,) -> f64 {
      let s = self.cnsm_while(|c| match c {
         '0'..='9' | '.' => true,
         _ => false,
      },);
      s.parse().unwrap()
   }

   fn parse_unit(&mut self,) -> Unit {
      match &*self.parse_idf().to_ascii_lowercase() {
         "px" => Unit::Px,
         _ => panic!("unrecognized unit"),
      }
   }

   fn parse_color(&mut self,) -> Value {
      assert_eq!(self.cnsm_chr(), '#');
      Value::ColorValue(Color { r: self.parse_hex_pair(), g: self.parse_hex_pair(), b: self.parse_hex_pair(), a: 255, },)
   }

   ///Parse two hexadecimal digits.
   fn parse_hex_pair(&mut self,) -> u8 {
      let s = &self.inp[self.pos..self.pos + 2];
      self.pos += 2;
      u8::from_str_radix(s, 16,).unwrap()
   }

   ///Parse a property name or Keyword.
   fn parse_idf(&mut self,) -> String { self.cnsm_while((valid_idf_chr),) }
}

fn valid_idf_chr(c: char,) -> bool {
   match c {
      'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' => true,
      _ => false,
   }
}

pub fn parse(src: String,) -> Stylesheet {
   let mut parser = Parser { pos: 0, inp: src, };
   Stylesheet { rules: parser.parse_rules(), }
}
