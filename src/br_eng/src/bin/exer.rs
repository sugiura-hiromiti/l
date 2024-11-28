#[cfg(feature = "exer")]
mod xr {

   pub fn prettyprint(nod: Node,) { println!("{:#?}", nod) }

   #[test]
   fn xr_pp() {
      let src = "<html>
    <body>
        <h1>Title</h1>
        <div id=\"main\" class=\"test\">
            <p>Hello <em>world</em>!</p>
        </div>
    </body>
</html>"
         .to_string();
      dom::prettyprint(html::parse(src,),)
   }
}
fn main() {}
