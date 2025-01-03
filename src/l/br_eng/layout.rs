#![allow(unused)]
use crate::{css, style};

///CSS box model. All sizes are in px.
#[derive(Default, Clone,)]
pub struct Dimensions {
   //Position of the content area  relative to the document origin:
   pub content: Rct,
   //Surrounding edges:
   padding:     EdgeSizes,
   pub border:  EdgeSizes,
   margin:      EdgeSizes,
}

impl Dimensions {
   ///The area covered by the content area plus its padding.
   fn padding_box(&self,) -> Rct { self.content.expanded_by(&self.padding,) }

   ///The area covered by the content area plus padding and borders.
   pub fn border_box(&self,) -> Rct { self.padding_box().expanded_by(&self.border,) }

   ///The area covered by the content area plus padding, borders, and margin.
   fn margin_box(&self,) -> Rct { self.border_box().expanded_by(&self.margin,) }
}

///Rectangular module
#[derive(Default, Clone,)]
pub struct Rct {
   pub x:      f64,
   pub y:      f64,
   pub width:  f64,
   pub height: f64,
}

impl Rct {
   fn expanded_by(&self, edge: &EdgeSizes,) -> Rct {
      Rct {
         x:      self.x - edge.left,
         y:      self.y - edge.top,
         width:  self.width + edge.left + edge.right,
         height: self.height + edge.top + edge.bottom,
      }
   }
}

///Positions of 4 corners
#[derive(Default, Clone,)]
pub struct EdgeSizes {
   pub left:   f64,
   pub right:  f64,
   pub top:    f64,
   pub bottom: f64,
}

///The layout tree is a collection of layoutboxes. It contains boxes as child
#[derive(Clone,)]
pub struct LayoutBox<'a,> {
   pub box_type:   BoxType<'a,>,
   pub dimensions: Dimensions,
   pub children:   Vec<LayoutBox<'a,>,>,
}

///A box can be a block node, an inline node, OR an anonymous block box
#[derive(Clone,)]
pub enum BoxType<'a,> {
   BlockNode(&'a style::StyledNode<'a,>,),
   InlineNode(&'a style::StyledNode<'a,>,),
   AnonymousBlock,
}

impl<'a,> LayoutBox<'a,> {
   ///Constructor
   fn new(box_type: BoxType,) -> LayoutBox { LayoutBox { box_type, dimensions: Default::default(), children: vec![], } }

   ///getter of style_node which is contained in box_type
   fn get_style_node(&self,) -> &'a style::StyledNode<'a,> {
      match &self.box_type {
         BoxType::BlockNode(nod,) | BoxType::InlineNode(nod,) => nod,
         BoxType::AnonymousBlock => panic!("AnonymousBlock has no style node",),
      }
   }

   ///Where a new inline child should go.
   fn get_inline_container(self,) -> LayoutBox<'a,> {
      match self.box_type {
         BoxType::BlockNode(_,) => {
            //If we've just generated an anonymous block box, keep using it.
            //Otherwise, create a new one.
            match self.children.last() {
               Some(&LayoutBox { box_type: BoxType::AnonymousBlock, .. },) => self.children.last().unwrap().clone(),
               _ => {
                  let mut cl = self;
                  cl.children.push(LayoutBox::new(BoxType::AnonymousBlock,),);
                  cl.children.last().unwrap().clone()
               }
            }
         }
         _ => self,
      }
   }

   ///Layout a box and its descendants.
   fn layout(&mut self, cntin_blck: &Dimensions,) {
      match self.box_type {
         BoxType::BlockNode(_,) => self.layout_block(cntin_blck,),
         BoxType::InlineNode(_,) => {}
         BoxType::AnonymousBlock => {}
      }
   }

   ///Block's width depends on its parent, height depends on its children
   fn layout_block(&mut self, cntin_blck: &Dimensions,) {
      //Calculate parent's width at first
      self.calc_width(&cntin_blck,);
      self.calc_position(cntin_blck,);
      self.layout_children();
      //Calculate parent's height at last
      self.calc_height();
   }

   ///Calculate width of block
   fn calc_width(&mut self, cntin_blck: &Dimensions,) {
      use css::{
         Unit,
         Value::{Keyword, Length},
      };
      let style = self.get_style_node();
      //'width' has initial value 'auto'
      let auto = Keyword("auto".to_string(),);
      let mut width = style.val("width",).unwrap_or(auto.clone(),);

      //margin, border, padding have init value 0.
      let zero = Length(0.0, Unit::Px,);

      let mut margin_left = style.lookup("margin-left", "margin", &zero,);
      let mut margin_right = style.lookup("margin-right", "margin", &zero,);
      let border_left = style.lookup("border-left-width", "border-width", &zero,);
      let border_right = style.lookup("border-right-width", "border-width", &zero,);
      let padding_left = style.lookup("padding-left", "padding", &zero,);
      let padding_right = style.lookup("padding-right", "padding", &zero,);

      let total = [&margin_left, &margin_right, &border_left, &border_right, &padding_left, &padding_right, &width,]
         .iter()
         .map(|v| v.to_px(),)
         .sum::<f64>();
      //if width!=auto & total is wider than container, treat auto margins as 0.
      if width != auto && total > cntin_blck.content.width {
         if margin_left == auto {
            margin_left = Length(0.0, Unit::Px,);
         }
         if margin_right == auto {
            margin_right = Length(0.0, Unit::Px,);
         }
      }
      //if 'flow' is +, it's underflow. 'flow' is -, it's overflow.
      let flow = cntin_blck.content.width - total;
      match (width == auto, margin_left == auto, margin_right == auto,) {
         //If the values are overconstrained, calculate margin_right.
         (false, false, false,) => margin_right = Length(margin_right.to_px() + flow, Unit::Px,),
         //If exactly one size is auto, its used value follows from the equality.
         (false, false, true,) => margin_right = Length(flow, Unit::Px,),
         (false, true, false,) => margin_left = Length(flow, Unit::Px,),
         //If width is set to auto, any other auto values become 0.
         (true, ..,) => {
            if margin_left == auto {
               margin_left = Length(0.0, Unit::Px,);
            }
            if margin_right == auto {
               margin_right = Length(0.0, Unit::Px,);
            }
            if flow >= 0.0 {
               width = Length(flow, Unit::Px,);
            } else {
               width = Length(0.0, Unit::Px,);
               margin_right = Length(margin_right.to_px() + flow, Unit::Px,);
            }
         }
         //If margin_left and margin_right are both auto, their used values are equal.
         (false, true, true,) => {
            margin_left = Length(flow / 2.0, Unit::Px,);
            margin_right = Length(flow / 2.0, Unit::Px,);
         }
      }
   }

   fn calc_position(&mut self, cntin_blck: &Dimensions,) {
      let style = self.get_style_node();
      let d = &mut self.dimensions;

      //margin, border, and padding have init value 0.
      let zero = css::Value::Length(0.0, css::Unit::Px,);

      //If margin_top or margin_bottom is 'auto', the used value is zero.
      d.margin.top = style.lookup("margin-top", "margin", &zero,).to_px();
      d.margin.bottom = style.lookup("margin-bottom", "margin", &zero,).to_px();
      d.border.top = style.lookup("border-top-width", "border-width", &zero,).to_px();
      d.border.bottom = style.lookup("border-bottom-width", "border-width", &zero,).to_px();
      d.padding.top = style.lookup("padding-top", "padding", &zero,).to_px();
      d.padding.bottom = style.lookup("padding-bottom", "padding", &zero,).to_px();

      d.content.x = cntin_blck.content.x + d.margin.left + d.border.left + d.padding.left;
      d.content.y = cntin_blck.content.height + cntin_blck.content.y + d.margin.top + d.border.top + d.padding.top;
   }

   fn layout_children(&mut self,) {
      let d = &mut self.dimensions;
      for child in &mut self.children {
         child.layout(d,);
         //Track the height so each child is laid out below the previous content.
         d.content.height = d.content.height + child.dimensions.margin_box().height;
      }
   }

   ///If the height is set to an explicit lenght, use that exact lenght.
   /// Otherwise, just keep the value set by 'layout_block_children'.
   fn calc_height(&mut self,) {
      if let Some(css::Value::Length(h, css::Unit::Px,),) = self.get_style_node().val("height",) {
         self.dimensions.content.height = h;
      }
   }
}

///Build the tree of LayoutBoxes, but don't perform any layout calculations
/// yet.
fn build_layout_tree<'a,>(style_node: &'a style::StyledNode<'a,>,) -> LayoutBox<'a,> {
   use {style::Display::*, BoxType::*};
   //Create the root box.
   let mut root = LayoutBox::new(match style_node.display() {
      Block => BlockNode(style_node,),
      Inline => InlineNode(style_node,),
      Non => panic!("Root node has display: none."),
   },);
   //Create the descendant boxes.
   for child in &style_node.children {
      root = match child.display() {
         Block | Inline => {
            let mut cl = root.clone();
            let mut ret = cl.get_inline_container();
            ret.children.push(build_layout_tree(&child,),);
            ret
         }
         _ => root,
      };
   }
   root
}

///Transform a style tree into a layout tree
pub fn layout_tree<'a,>(node: &'a style::StyledNode<'a,>, mut cntin_blck: Dimensions,) -> LayoutBox<'a,> {
   //The layout algorithm expects the container height to start at 0
   cntin_blck.content.height = 0.0;

   let mut root_box = build_layout_tree(node,);
   root_box.layout(&cntin_blck,);
   root_box
}
