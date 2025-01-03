//! handle html syntax
#![allow(unused, unreachable_code)]

use {
	crate::dom::{AttrMap, Element, Node, Text},
	combine::{
		any, attempt, between, choice, eof,
		error::{ParseError, StreamError, StringStreamError},
		look_ahead, many, many1,
		parser::{
			char::{char, letter, newline, space},
			repeat::repeat_until,
		},
		produce, satisfy, sep_by1,
		stream::StreamErrorFor,
		Parser, Stream,
	},
};

/// `attribute` consumes `name="value"`.
fn attribute<Input,>() -> impl Parser<Input, Output = (String, String,),>
where
	Input: Stream<Token = char,>,
	Input::Error: ParseError<Input::Token, Input::Range, Input::Position,>,
{
	(
		many1::<String, _, _,>(letter(),), //read attribute name
		many::<String, _, _,>(space().or(newline(),),), //skip space & newline
		char('=',),                        //eat =
		many::<String, _, _,>(space().or(newline(),),), //skip space & newline
		between(
			char('"',),
			char('"',),
			many1::<String, _, _,>(satisfy(|c| {
				c != '"'
			},),),
		), /* read value */
	)
		.map(|v| (v.0, v.4,),)
}

/// `attributes` consumes `name1="value1" name2="value2" ... name="value"`
fn attributes<Input,>() -> impl Parser<Input, Output = AttrMap,>
where
	Input: Stream<Token = char,>,
	Input::Error: ParseError<Input::Token, Input::Range, Input::Position,>,
{
	let attrs = (
		attribute(),
		many::<String, _, _,>(space().or(newline(),),),
	)
		.map(|a| a.0,);
	many(attrs,)
}

fn open_tag<Input,>() -> impl Parser<Input, Output = (String, AttrMap,),>
where
	Input: Stream<Token = char,>,
	Input::Error: ParseError<Input::Token, Input::Range, Input::Position,>,
{
	(
		char('<',),
		many1::<String, _, _,>(letter(),),
		between(
			many::<String, _, _,>(space().or(newline(),),),
			char('>',),
			attributes(),
		),
	)
		.map(|t| (t.1, t.2,),)
}

/// close_tag consumes `</tag_name>`.
fn close_tag<Input,>() -> impl Parser<Input, Output = String,>
where
	Input: Stream<Token = char,>,
	Input::Error: ParseError<Input::Token, Input::Range, Input::Position,>,
{
	(
		char('<',),
		between(
			char('/',),
			char('>',),
			many1::<String, _, _,>(satisfy(|c| {
				c != '>'
			},),),
		),
	)
		.map(|t| t.1,)
}

// `nodes_` (and `nodes`) tries to parse input as Element or Text.
fn nodes_<Input,>() -> impl Parser<Input, Output = Vec<Box<Node,>,>,>
where
	Input: Stream<Token = char,>,
	Input::Error: ParseError<Input::Token, Input::Range, Input::Position,>,
{
	attempt(many(choice((
		attempt(element(),),
		attempt(text(),),
	),),),)
}

/// `text` consumes input until `<` comes.
fn text<Input,>() -> impl Parser<Input, Output = Box<Node,>,>
where
	Input: Stream<Token = char,>,
	Input::Error: ParseError<Input::Token, Input::Range, Input::Position,>,
{
	many1(satisfy(|c| {
		c != '<'
	},),)
	.map(|t| Text::new(t,),)
}

/// `element` consumes `<tag_name attr_name="attr_value"
/// ...>(children)</tag_name>`.
fn element<Input,>() -> impl Parser<Input, Output = Box<Node,>,>
where
	Input: Stream<Token = char,>,
	Input::Error: ParseError<Input::Token, Input::Range, Input::Position,>,
{
	(
		open_tag(),
		nodes(),
		close_tag(),
	)
		.and_then(
			|((op_tag, attributes,), children, cls_tag,)| {
				if op_tag == cls_tag {
					Ok(Element::new(
						op_tag, attributes, children,
					),)
				} else {
					Err(
						<Input::Error as combine::error::ParseError<
							char,
							Input::Range,
							Input::Position,
						>>::StreamError::message_static_message(
							"tag name of open tag and close tag mismatched",
						),
					)
				}
			},
		)
}

combine::parser! {
	 fn nodes[Input]()(Input) -> Vec<Box<Node>>
	 where [Input: Stream<Token = char>]
	 {
		  nodes_()
	 }
}

pub fn parse(raw: &str,) -> Box<Node,> {
	let mut nodes = parse_raw(raw,);
	if nodes.len() == 1 {
		nodes.pop().unwrap()
	} else {
		Element::new(
			"html".to_string(),
			AttrMap::new(),
			nodes,
		)
	}
}

pub fn parse_raw(raw: &str,) -> Vec<Box<Node,>,> {
	let (nodes, _,) = nodes_().parse(raw,).unwrap();
	nodes
}

#[cfg(test)]
mod tests {
	use crate::dom::Text;

	use super::*;

	// parsing tests of attributes
	#[test]
	fn test_parse_attribute() {
		assert_eq!(
			attribute().parse("test=\"foobar\""),
			Ok((
				(
					"test".to_string(),
					"foobar".to_string()
				),
				""
			))
		);

		assert_eq!(
			attribute().parse("test = \"foobar\""),
			Ok((
				(
					"test".to_string(),
					"foobar".to_string()
				),
				""
			))
		)
	}

	#[test]
	fn test_parse_attributes() {
		let mut expected_map = AttrMap::new();
		expected_map.insert(
			"test".to_string(),
			"foobar".to_string(),
		);
		expected_map.insert(
			"abc".to_string(),
			"def".to_string(),
		);
		assert_eq!(
			attributes().parse("test=\"foobar\" abc=\"def\""),
			Ok((
				expected_map,
				""
			))
		);

		assert_eq!(
			attributes().parse(""),
			Ok((
				AttrMap::new(),
				""
			))
		)
	}

	#[test]
	fn test_parse_open_tag() {
		{
			assert_eq!(
				open_tag().parse("<p>aaaa"),
				Ok((
					(
						"p".to_string(),
						AttrMap::new()
					),
					"aaaa"
				))
			);
		}
		{
			let mut attributes = AttrMap::new();
			attributes.insert(
				"id".to_string(),
				"test".to_string(),
			);
			assert_eq!(
				open_tag().parse("<p id=\"test\">"),
				Ok((
					(
						"p".to_string(),
						attributes
					),
					""
				))
			)
		}

		{
			let result = open_tag().parse("<p id=\"test\" class=\"sample\">",);
			let mut attributes = AttrMap::new();
			attributes.insert(
				"id".to_string(),
				"test".to_string(),
			);
			attributes.insert(
				"class".to_string(),
				"sample".to_string(),
			);
			assert_eq!(
				result,
				Ok((
					(
						"p".to_string(),
						attributes
					),
					""
				))
			);
		}

		{
			assert!(open_tag()
				.parse("<p id>")
				.is_err());
		}
	}

	// parsing tests of close tags
	#[test]
	fn test_parse_close_tag() {
		let result = close_tag().parse("</p>",);
		assert_eq!(
			result,
			Ok((
				"p".to_string(),
				""
			))
		)
	}

	#[test]
	fn test_parse_element() {
		assert_eq!(
			element().parse("<p></p>"),
			Ok((
				Element::new(
					"p".to_string(),
					AttrMap::new(),
					vec![]
				),
				""
			))
		);

		assert_eq!(
			element().parse("<p>hello world</p>"),
			Ok((
				Element::new(
					"p".to_string(),
					AttrMap::new(),
					vec![Text::new("hello world".to_string())]
				),
				""
			))
		);

		assert_eq!(
			element().parse("<div><p>hello world</p></div>"),
			Ok((
				Element::new(
					"div".to_string(),
					AttrMap::new(),
					vec![Element::new(
						"p".to_string(),
						AttrMap::new(),
						vec![Text::new("hello world".to_string())]
					)],
				),
				""
			))
		);

		assert!(element()
			.parse("<p>hello world</div>")
			.is_err());
	}

	#[test]
	fn test_parse_text() {
		{
			assert_eq!(
				text().parse("Hello World"),
				Ok((
					Text::new("Hello World".to_string()),
					""
				))
			);
		}
		{
			assert_eq!(
				text().parse("Hello World<"),
				Ok((
					Text::new("Hello World".to_string()),
					"<"
				))
			);
		}
	}
}
