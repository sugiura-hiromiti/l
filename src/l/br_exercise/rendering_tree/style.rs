//! This module includes some implementations on node styles.

use super::css::CSSValue;
use super::css::Stylesheet;
use super::dom::Node;
use super::dom::NodeType;
use std::collections::HashMap;

pub type PropertyMap = HashMap<String, CSSValue,>;

#[derive(Debug, PartialEq,)]
pub enum Display {
	Inline,
	Block,
	None,
}

/// `StyledNode` wraps `Node` with related CSS properties.
/// It forms a tree as `Node` does.
#[derive(Debug, PartialEq,)]
pub struct StyledNode<'a,> {
	pub node_type: &'a NodeType,
	pub children:  Vec<StyledNode<'a,>,>,

	pub properties: PropertyMap,
}

pub fn to_styled_node<'a,>(
	_node: &'a Box<Node,>,
	_stylesheet: &Stylesheet,
) -> Option<StyledNode<'a,>,> {
	todo!("you need to implement this")
}

impl StyledNode<'_,> {
	pub fn display(&self,) -> Display {
		match self.properties.get("display",) {
			Some(CSSValue::Keyword(s,),) => match s.as_str() {
				"block" => Display::Block,
				"none" => Display::None,
				_ => Display::Inline,
			},
			_ => Display::Inline,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::super::css::AttributeSelectorOp;
	use super::super::css::Declaration;
	use super::super::css::Rule;
	use super::super::css::SimpleSelector;
	use super::super::dom::Element;

	use super::*;

	#[test]
	#[ignore = "fails"]
	fn test_to_styled_node_single() {
		let e = &Element::new(
			"p".to_string(),
			[("id".to_string(), "test".to_string(),),].iter().cloned().collect(),
			vec![],
		);
		let testcases = vec![
			(
				// * { display: block; }
				Stylesheet::new(vec![Rule {
					selectors:    vec![SimpleSelector::UniversalSelector],
					declarations: vec![Declaration {
						name:  "display".to_string(),
						value: CSSValue::Keyword("block".to_string(),),
					}],
				}],),
				vec![("display".to_string(), CSSValue::Keyword("block".to_string(),),)],
			),
			(
				// div { display: block; }
				Stylesheet::new(vec![Rule {
					selectors:    vec![SimpleSelector::TypeSelector { tag_name: "div".into(), }],
					declarations: vec![Declaration {
						name:  "display".into(),
						value: CSSValue::Keyword("block".to_string(),),
					}],
				}],),
				vec![],
			),
			(
				// * { display: block; }
				// div { display: inline; }
				Stylesheet::new(vec![
					Rule {
						selectors:    vec![SimpleSelector::UniversalSelector],
						declarations: vec![Declaration {
							name:  "display".to_string(),
							value: CSSValue::Keyword("block".into(),),
						}],
					},
					Rule {
						selectors:    vec![SimpleSelector::TypeSelector {
							tag_name: "div".into(),
						}],
						declarations: vec![Declaration {
							name:  "display".into(),
							value: CSSValue::Keyword("inline".into(),),
						}],
					},
				],),
				vec![("display".to_string(), CSSValue::Keyword("block".to_string(),),)],
			),
			(
				// * { display: block; }
				// p { display: inline; testname: testvalue; }
				Stylesheet::new(vec![
					Rule {
						selectors:    vec![SimpleSelector::UniversalSelector],
						declarations: vec![Declaration {
							name:  "display".to_string(),
							value: CSSValue::Keyword("block".into(),),
						}],
					},
					Rule {
						selectors:    vec![SimpleSelector::TypeSelector { tag_name: "p".into(), }],
						declarations: vec![
							Declaration {
								name:  "display".into(),
								value: CSSValue::Keyword("inline".into(),),
							},
							Declaration {
								name:  "testname".into(),
								value: CSSValue::Keyword("testvalue".into(),),
							},
						],
					},
				],),
				vec![
					("display".into(), CSSValue::Keyword("inline".into(),),),
					("testname".into(), CSSValue::Keyword("testvalue".into(),),),
				],
			),
			(
				// * { display: block; }
				// p[id=hello] { testname: testvalue; }
				Stylesheet::new(vec![
					Rule {
						selectors:    vec![SimpleSelector::UniversalSelector],
						declarations: vec![Declaration {
							name:  "display".to_string(),
							value: CSSValue::Keyword("block".into(),),
						}],
					},
					Rule {
						selectors:    vec![SimpleSelector::AttributeSelector {
							tag_name:  "p".into(),
							op:        AttributeSelectorOp::Eq,
							attribute: "id".into(),
							value:     "hello".into(),
						}],
						declarations: vec![Declaration {
							name:  "testname".into(),
							value: CSSValue::Keyword("testvalue".into(),),
						}],
					},
				],),
				vec![("display".into(), CSSValue::Keyword("block".into(),),)],
			),
			(
				// * { display: block; }
				// p[id=hello] { testname: testvalue; }
				Stylesheet::new(vec![
					Rule {
						selectors:    vec![SimpleSelector::UniversalSelector],
						declarations: vec![Declaration {
							name:  "display".to_string(),
							value: CSSValue::Keyword("block".into(),),
						}],
					},
					Rule {
						selectors:    vec![SimpleSelector::AttributeSelector {
							tag_name:  "p".into(),
							op:        AttributeSelectorOp::Eq,
							attribute: "id".into(),
							value:     "test".into(),
						}],
						declarations: vec![Declaration {
							name:  "testname".into(),
							value: CSSValue::Keyword("testvalue".into(),),
						}],
					},
				],),
				vec![
					("display".into(), CSSValue::Keyword("block".into(),),),
					("testname".into(), CSSValue::Keyword("testvalue".into(),),),
				],
			),
		];

		for (stylesheet, properties,) in testcases {
			assert_eq!(
				to_styled_node(e, &stylesheet),
				Some(StyledNode {
					node_type:  &e.node_type,
					properties: properties.iter().cloned().collect(),
					children:   vec![],
				})
			);
		}
	}

	#[test]
	#[ignore = "fails"]
	fn test_to_styled_node_nested() {
		let parent = &Element::new(
			"div".to_string(),
			[("id".to_string(), "test".to_string(),),].iter().cloned().collect(),
			vec![Element::new(
				"p".to_string(),
				[("id".to_string(), "test".to_string(),),].iter().cloned().collect(),
				vec![],
			)],
		);
		let child_node_type = Element::new(
			"p".to_string(),
			[("id".to_string(), "test".to_string(),),].iter().cloned().collect(),
			vec![],
		)
		.node_type;

		{
			// * { display: block; }
			let stylesheet = Stylesheet::new(vec![Rule {
				selectors:    vec![SimpleSelector::UniversalSelector],
				declarations: vec![Declaration {
					name:  "display".to_string(),
					value: CSSValue::Keyword("block".to_string(),),
				}],
			}],);

			assert_eq!(
				to_styled_node(parent, &stylesheet),
				Some(StyledNode {
					node_type:  &parent.node_type,
					properties: [("display".to_string(), CSSValue::Keyword("block".to_string()),)]
						.iter()
						.cloned()
						.collect(),
					children:   vec![StyledNode {
						node_type:  &child_node_type,
						properties: [(
							"display".to_string(),
							CSSValue::Keyword("block".to_string()),
						)]
						.iter()
						.cloned()
						.collect(),
						children:   vec![],
					}],
				})
			);
		}

		{
			// p { display: block; }
			let stylesheet = Stylesheet::new(vec![Rule {
				selectors:    vec![SimpleSelector::TypeSelector { tag_name: "p".into(), }],
				declarations: vec![Declaration {
					name:  "display".to_string(),
					value: CSSValue::Keyword("block".to_string(),),
				}],
			}],);

			assert_eq!(
				to_styled_node(parent, &stylesheet),
				Some(StyledNode {
					node_type:  &parent.node_type,
					properties: [].iter().cloned().collect(),
					children:   vec![StyledNode {
						node_type:  &child_node_type,
						properties: [(
							"display".to_string(),
							CSSValue::Keyword("block".to_string()),
						)]
						.iter()
						.cloned()
						.collect(),
						children:   vec![],
					}],
				})
			);
		}
	}

	#[test]
	#[ignore = "fails"]
	fn test_to_styled_node_nested_single() {
		let parent = &Element::new(
			"div".to_string(),
			[("id".to_string(), "test".to_string(),),].iter().cloned().collect(),
			vec![],
		);

		// p { display: none; }
		let stylesheet = Stylesheet::new(vec![Rule {
			selectors:    vec![SimpleSelector::TypeSelector { tag_name: "div".into(), }],
			declarations: vec![Declaration {
				name:  "display".to_string(),
				value: CSSValue::Keyword("none".to_string(),),
			}],
		}],);

		assert_eq!(to_styled_node(parent, &stylesheet), None);
	}

	#[test]
	#[ignore = "fails"]
	fn test_to_styled_node_nested_none() {
		let parent = &Element::new(
			"div".to_string(),
			[("id".to_string(), "test".to_string(),),].iter().cloned().collect(),
			vec![Element::new(
				"p".to_string(),
				[("id".to_string(), "test".to_string(),),].iter().cloned().collect(),
				vec![],
			)],
		);

		// p { display: none; }
		let stylesheet = Stylesheet::new(vec![Rule {
			selectors:    vec![SimpleSelector::TypeSelector { tag_name: "p".into(), }],
			declarations: vec![Declaration {
				name:  "display".to_string(),
				value: CSSValue::Keyword("none".to_string(),),
			}],
		}],);

		assert_eq!(
			to_styled_node(parent, &stylesheet),
			Some(StyledNode {
				node_type:  &parent.node_type,
				properties: [].iter().cloned().collect(),
				children:   vec![],
			})
		);
	}
}
