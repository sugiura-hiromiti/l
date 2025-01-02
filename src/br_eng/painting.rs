#![allow(unused)]
use crate::{css, layout};

type DisplayList = Vec<DisplayCommand,>;

///Commands to display layout
enum DisplayCommand {
   SolidColor(css::Color, layout::Rct,),
   //insert more commands here
}

///Store pixels which is converted from DisplayCommand
pub struct Canvas {
   pub pixels: Vec<css::Color,>,
   pub width:  usize,
   pub height: usize,
}

impl Canvas {
   ///Constructor. Create a blank Canvas
   fn new(width: usize, height: usize,) -> Canvas {
      let white = css::Color { r: 255, g: 255, b: 255, a: 255, };
      Canvas { pixels: vec![white; height * width], width, height, }
   }

   ///Here, we just paint a rectangler
   fn paint_item(&mut self, item: &DisplayCommand,) {
      match &item {
         &DisplayCommand::SolidColor(color, rct,) => {
            let x0 = rct.x.clamp(0.0, self.width as f64,) as usize;
            let y0 = rct.y.clamp(0.0, self.height as f64,) as usize;
            let x1 = (rct.x + rct.width).clamp(0.0, self.width as f64,) as usize;
            let y1 = (rct.y + rct.height).clamp(0.0, self.height as f64,) as usize;
            for y in y0..y1 {
               for x in x0..x1 {
                  self.pixels[x + y * self.width] = color.clone();
               }
            }
         }
         _ => {}
      }
   }
}

///Constructor of DisplayList. With init, draw empty layout box
fn build_display_list(layout_root: &layout::LayoutBox,) -> DisplayList {
   let mut list = Vec::new();
   render_layout_box(&mut list, layout_root,);
   list
}

///Render background, borders, texts, etc
fn render_layout_box(list: &mut DisplayList, layout_box: &layout::LayoutBox,) {
   render_bg(list, layout_box,);
   render_borders(list, layout_box,);
   //text rendering is still unsupported
   for child in &layout_box.children {
      render_layout_box(list, child,)
   }
}

///Render background. If bg color isn't specified, transparent
fn render_bg(list: &mut DisplayList, layout_box: &layout::LayoutBox,) {
   get_color(layout_box, "background",)
      .map(|clr| list.push(DisplayCommand::SolidColor(clr, layout_box.dimensions.border_box(),),),);
}

///If AnonymousBlock or not specified color, return None. Else, return
/// specified color
fn get_color(layout_box: &layout::LayoutBox, nam: &str,) -> Option<css::Color,> {
   match layout_box.box_type {
      layout::BoxType::BlockNode(style,) | layout::BoxType::InlineNode(style,) => match style.val(nam,) {
         Some(css::Value::ColorValue(clr,),) => Some(clr,),
         _ => None,
      },
      layout::BoxType::AnonymousBlock => None,
   }
}

///Render borders
fn render_borders(list: &mut DisplayList, layout_box: &layout::LayoutBox,) {
   let clr = match get_color(layout_box, "border-color",) {
      Some(clr,) => clr,
      _ => return, //bail out if no border-color is specified
   };
   let d = &layout_box.dimensions;
   let border_box = d.border_box();

   //Left border
   list.push(DisplayCommand::SolidColor(clr.clone(), layout::Rct {
      x:      border_box.x,
      y:      border_box.y,
      width:  d.border.left,
      height: border_box.height,
   },),);

   //Right border
   list.push(DisplayCommand::SolidColor(clr.clone(), layout::Rct {
      x:      border_box.x + border_box.width - d.border.right,
      y:      border_box.y,
      width:  d.border.right,
      height: border_box.height,
   },),);

   //Top border
   list.push(DisplayCommand::SolidColor(clr.clone(), layout::Rct {
      x:      border_box.x,
      y:      border_box.y,
      width:  border_box.width,
      height: d.border.top,
   },),);

   //Bottom border
   list.push(DisplayCommand::SolidColor(clr.clone(), layout::Rct {
      x:      border_box.x,
      y:      border_box.y + border_box.height - d.border.bottom,
      width:  border_box.width,
      height: d.border.bottom,
   },),);
}

///Paint a tree of Layout Boxes to an array of pixels.
pub fn paint(layout_root: &layout::LayoutBox, bounds: layout::Rct,) -> Canvas {
   let disp_lst = build_display_list(layout_root,);
   let mut canvas = Canvas::new(bounds.width as usize, bounds.height as usize,);
   for item in disp_lst {
      canvas.paint_item(&item,);
   }
   canvas
}
