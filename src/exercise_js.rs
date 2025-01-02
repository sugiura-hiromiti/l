//! <https://browserbook.shift-js.info/chapters/parsing-html>
pub mod css;
pub mod dom;
pub mod html;
pub mod javascript;
pub mod layout;
pub mod render;
pub mod renderer;
pub mod style;

const HTML: &str = r#"<body>
    <p>hello</p>
    <p class="inline">world</p>
    <p class="inline">:)</p>
    <div class="none"><p>this should not be shown</p></div>
    <style>
        .none {
            display: none;
        }
        .inline {
            display: inline;
        }
    </style>
</body>"#;

const DEFAULT_STYLESHEET: &str = r#"
script, style {
    display: none;
}
p, div {
    display: block;
}
"#;

pub fn collect_tag_inners(node: &Box<Node,>, tag_name: &str,) -> Vec<String,> {
	if let NodeType::Element(ref element,) = node.node_type {
		if element.tag_name.as_str() == tag_name {
			return vec![node.inner_text()];
		}
	}

	node.children
		.iter()
		.map(|child| collect_tag_inners(child, tag_name,),)
		.collect::<Vec<Vec<String,>,>>()
		.into_iter()
		.flatten()
		.collect()
}

fn js_main() {
	let mut siv = cursive::default();

	let node = html::parse(HTML,);
	let stylesheet = css::parse(&format!(
		"{}\n{}",
		DEFAULT_STYLESHEET,
		collect_tag_inners(&node, "style".into()).join("\n")
	),);

	let container = to_styled_node(&node, &stylesheet,)
		.and_then(|styled_node| Some(to_layout_box(styled_node,),),)
		.and_then(|layout_box| Some(to_element_container(layout_box,),),);
	if let Some(c,) = container {
		siv.add_fullscreen_layer(c,);
	}

	siv.run();
}
