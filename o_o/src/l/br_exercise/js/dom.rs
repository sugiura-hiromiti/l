use std::collections::HashMap;

pub type AttrMap = HashMap<String, String>;

#[derive(Debug, PartialEq)]
pub struct Node {
	pub node_type: NodeType,
	pub children: Vec<Box<Node>>,
}

impl Node {
	pub fn inner_text(&self) -> String {
		self.children
			.iter()
			.clone()
			.map(|node| match &node.node_type {
				NodeType::Text(t) => t.data.clone(),
				_ => node.inner_text(),
			})
			.collect::<Vec<_>>()
			.join("")
	}
}

#[derive(Debug, PartialEq)]
pub enum NodeType {
	Element(Element),
	Text(Text),
}

#[derive(Debug, PartialEq)]
pub struct Element {
	pub tag_name: String,
	pub attributes: AttrMap,
}

impl Element {
	pub fn new(name: String, attributes: AttrMap, children: Vec<Box<Node>>) -> Box<Node> {
		Box::new(Node {
			node_type: NodeType::Element(Element { tag_name: name, attributes }),
			children,
		})
	}
}

#[derive(Debug, PartialEq)]
pub struct Text {
	pub data: String,
}

impl Text {
	pub fn new(text: String) -> Box<Node> {
		Box::new(Node { node_type: NodeType::Text(Text { data: text }), children: vec![] })
	}
}
