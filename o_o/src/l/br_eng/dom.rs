use std::collections::HashMap;
use std::collections::HashSet;

pub type AttrMap = HashMap<String, String>;

//#[derive(Debug,)]
pub struct Node {
	//data common to all nodes
	pub children: Vec<Node>,
	//data specific to each node type
	pub node_type: NodeType,
}

//#[derive(Debug,)]
pub enum NodeType {
	Text(String),
	Element(ElementData),
}

//#[derive(Debug,)]
pub struct ElementData {
	pub tag_name: String,
	attributes: AttrMap,
}

impl ElementData {
	///Get attribute's id
	pub fn id(&self) -> Option<&String> {
		self.attributes.get("id")
	}

	///Get class list
	pub fn classes(&self) -> HashSet<&str> {
		match self.attributes.get("class") {
			Some(classlist) => classlist.split(' ').collect(),
			None => HashSet::new(),
		}
	}
}

pub fn text(data: String) -> Node {
	Node { children: Vec::new(), node_type: NodeType::Text(data) }
}
pub fn elem(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
	Node {
		children,
		node_type: NodeType::Element(ElementData { tag_name: name, attributes: attrs }),
	}
}
