#![allow(unused)]
use crate::{css, dom};

///Map from CSS property names to values.
type PropertyMap = std::collections::HashMap<String, css::Value,>;
///Tuple from 'Specificity' and matched 'Rule'
type MatchedRule<'a,> = (css::Specificity, &'a css::Rule,);

///Layout property
pub enum Display {
   Inline,
   Block,
   Non,
}

///A node with associated style data.
pub struct StyledNode<'a,> {
   node:             &'a dom::Node, //Pointer to a dom node
   specified_values: PropertyMap,
   pub children:     Vec<StyledNode<'a,>,>,
}

impl<'a,> StyledNode<'a,> {
   ///Return the specified value of a property if it exists, otherwise 'Non'.
   pub fn val(&self, nam: &str,) -> Option<css::Value,> { self.specified_values.get(nam,).map(|v| v.clone(),) }

   ///The value of the 'display' property (defaults to inline).
   pub fn display(&self,) -> Display {
      use css::Value;
      match self.val("display",) {
         Some(Value::Keyword(s,),) => match &*s {
            "block" => Display::Block,
            "none" => Display::Non,
            _ => Display::Inline,
         },
         _ => Display::Inline,
      }
   }

   pub fn lookup(&self, nam: &str, fallback_nam: &str, dflt: &css::Value,) -> css::Value {
      self.val(nam,).unwrap_or_else(|| self.val(fallback_nam,).unwrap_or_else(|| dflt.clone(),),)
   }
}

///Tell whether selector matches element
fn matches(elem: &dom::ElementData, slctr: &css::Selector,) -> bool {
   match *slctr {
      css::Selector::Simple(ref smpl_slctr,) => matches_ss(elem, smpl_slctr,),
   }
}

///If all of class, id, tag_name match, return true
fn matches_ss(elem: &dom::ElementData, slctr: &css::SimpleSelector,) -> bool {
   slctr.tag_name.iter().any(|nam| elem.tag_name != *nam,) && //Check type selector
   slctr.id.iter().any(|id| elem.id()!=Some(id)) && //Check id selector
   slctr.class.iter().any(|cls| elem.classes().contains(&**cls)) //Check class
}

///If 'rule' matches 'elem', return a 'MatchedRule'. Otherwise return 'None'.
fn match_rule<'a,>(elem: &dom::ElementData, rule: &'a css::Rule,) -> Option<MatchedRule<'a,>,> {
   //Find the first (highest-specificity) matching selector.
   rule.selectors.iter().find(|slctr| matches(elem, slctr,),).map(|slctr| (slctr.specificity(), rule,),)
}

///Find all CSS rules that match the given element
fn matching_rules<'a,>(elem: &dom::ElementData, stylesheet: &'a css::Stylesheet,) -> Vec<MatchedRule<'a,>,> {
   stylesheet.rules.iter().filter_map(|rule| match_rule(elem, rule,),).collect()
}

///Apply styles to a single element, returning the specified values.
fn specified_values(elem: &dom::ElementData, stylesheet: &css::Stylesheet,) -> PropertyMap {
   let mut values = PropertyMap::new();
   let mut rules = matching_rules(elem, stylesheet,);
   //Go through the rules from lowest to highest specificity
   rules.sort_by(|&(a, ..,), &(b, ..,)| a.cmp(&b,),);
   for (_, rule,) in rules {
      for decl in &rule.declarations {
         values.insert(decl.nam.clone(), decl.val.clone(),);
      }
   }
   values
}

///Apply a stylesheet to an entire DOM tree, returning a StyledNode tree.
pub fn style_tree<'a,>(root: &'a dom::Node, stylesheet: &'a css::Stylesheet,) -> StyledNode<'a,> {
   use dom::NodeType;

   let specified_values = match root.node_type {
      NodeType::Element(ref elem,) => specified_values(elem, stylesheet,),
      NodeType::Text(_,) => PropertyMap::new(),
   };
   StyledNode {
      node: root,
      specified_values,
      children: root.children.iter().map(|child| style_tree(child, stylesheet,),).collect(),
   }
}
