//! <https://github.com/tiny-browserbook/exercise-css>

use combine::ParseError;
use combine::Parser;
use combine::Stream;
use combine::between;
use combine::choice;
use combine::many;
use combine::many1;
use combine::optional;
use combine::parser::char::char;
use combine::parser::char::letter;
use combine::parser::char::newline;
use combine::parser::char::space;
use combine::parser::char::{self};
use combine::sep_by;
use combine::sep_end_by;

/// `Stylesheet` represents a single stylesheet.
/// It consists of multiple rules, which are called "rule-list" in the standard (https://www.w3.org/TR/css-syntax-3/).
#[derive(Debug, PartialEq)]
pub struct Stylesheet {
	pub rules: Vec<Rule>,
}

impl Stylesheet {
	pub fn new(rules: Vec<Rule>) -> Self {
		Stylesheet { rules }
	}
}

/// `Rule` represents a single CSS rule.
#[derive(Debug, PartialEq)]
pub struct Rule {
	pub selectors: Vec<Selector>,
	pub declarations: Vec<Declaration>,
}

/// NOTE: This is not compliant to the standard for simplicity.
/// In the standard, *a selector* is *a chain* of one or more sequences of simple selectors
/// separated by combinators, where a sequence of simple selectors is a chain of simple selectors
/// that are not separated by a combinator. Hence `Selector` is in fact something like
/// `Vec<Vec<SimpleSelector>>`.
pub type Selector = SimpleSelector;

/// `SimpleSelector` represents a simple selector defined in the following standard:
/// https://www.w3.org/TR/selectors-3/#selector-syntax
#[derive(Debug, PartialEq)]
pub enum SimpleSelector {
	UniversalSelector,
	TypeSelector {
		tag_name: String,
	},
	AttributeSelector {
		tag_name: String,
		op: AttributeSelectorOp,
		attribute: String,
		value: String,
	},
	ClassSelector {
		class_name: String,
	},
	// TODO (enhancement): support multiple attribute selectors like `a[href=bar][ping=foo]`
	// TODO (enhancement): support more attribute selectors
}

/// `AttributeSelectorOp` is an operator which is allowed to use.
/// See https://www.w3.org/TR/selectors-3/#attribute-selectors to check the full list of available operators.
#[derive(Debug, PartialEq)]
pub enum AttributeSelectorOp {
	Eq,      // =
	Contain, // ~=
}

/// `Declaration` represents a CSS declaration defined at [CSS Syntax Module Level 3](https://www.w3.org/TR/css-syntax-3/#declaration)
/// Declarations are further categorized into the followings:
/// - descriptors, which are mostly used in "at-rules" like `@foo (bar: piyo)` https://www.w3.org/Style/CSS/all-descriptors.en.html
/// - properties, which are mostly used in "qualified rules" like `.foo {bar: piyo}` https://www.w3.org/Style/CSS/all-descriptors.en.html
/// For simplicity, we handle two types of declarations together.
#[derive(Debug, PartialEq)]
pub struct Declaration {
	pub name: String,
	pub value: CSSValue,
	// TODO (enhancement): add a field for `!important`
}

/// `CSSValue` represents some of *component value types* defined at [CSS Values and Units Module Level 3](https://www.w3.org/TR/css-values-3/#component-types).
#[derive(Debug, PartialEq, Clone)]
pub enum CSSValue {
	Keyword(String),
}

pub fn parse(raw: &str) -> Stylesheet {
	rules().parse(raw).map(|(rules, _)| Stylesheet::new(rules)).unwrap()
}

fn rules<Input>() -> impl Parser<Input, Output = Vec<Rule>>
where
	Input: Stream<Token = char>,
	Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
	many1((rule(), many1::<String, _, _>(space().or(newline()))).map(|t| t.0))
}

fn rule<Input>() -> impl Parser<Input, Output = Rule>
where
	Input: Stream<Token = char>,
	Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
	(
		selectors(),
		between(
			char('{'),
			char('}'),
			(many::<String, _, _>(space().or(newline())), declarations()),
		),
	)
		.map(|(selectors, (_, declarations))| Rule { selectors, declarations })
}

fn selectors<Input>() -> impl Parser<Input, Output = Vec<Selector>>
where
	Input: Stream<Token = char>,
	Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
	let ss = (
		many::<String, _, _>(space::<Input>().or(newline())),
		simple_selector(),
		many::<String, _, _>(space::<Input>().or(newline())),
	)
		.map(|t| t.1);
	sep_by(ss, char(','))
}

fn simple_selector<Input>() -> impl Parser<Input, Output = SimpleSelector>
where
	Input: Stream<Token = char>,
	Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
	let cls =
		(char('.'), many1(letter())).map(|t| SimpleSelector::ClassSelector { class_name: t.1 });
	let attr = (
		many1(letter::<Input>()),
		many::<String, _, _>(space()),
		optional(between(
			char('['),
			char(']'),
			(many1(letter()), optional(char('~')), char('='), many1(letter())),
		)),
	)
		.map(|(n, _, o)| match o {
			Some(p) => {
				let op = match p.1 {
					Some(_) => AttributeSelectorOp::Contain,
					None => AttributeSelectorOp::Eq,
				};
				SimpleSelector::AttributeSelector { tag_name: n, op, attribute: p.0, value: p.3 }
			},
			None => SimpleSelector::TypeSelector { tag_name: n },
		});
	choice((char('*').map(|_| SimpleSelector::UniversalSelector), cls, attr))
}

fn declarations<Input>() -> impl Parser<Input, Output = Vec<Declaration>>
where
	Input: Stream<Token = char>,
	Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
	let dcl = (declaration::<Input>(), many::<String, _, _>(space::<Input>().or(newline())))
		.map(|(d, _)| d);
	sep_end_by(dcl, (char(';'), many::<String, _, _>(space().or(newline()))))
}

fn declaration<Input>() -> impl Parser<Input, Output = Declaration>
where
	Input: Stream<Token = char>,
	Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
	(
		many1(letter()),
		many::<String, _, _>(space().or(newline())),
		char(':'),
		many::<String, _, _>(space().or(newline())),
		css_value(),
	)
		.map(|t| Declaration { name: t.0, value: t.4 })
}

fn css_value<Input>() -> impl Parser<Input, Output = CSSValue>
where
	Input: Stream<Token = char>,
	Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
	many1(letter()).map(CSSValue::Keyword)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[ignore = "fails"]
	#[test]
	fn test_stylesheet() {
		assert_eq!(
			rules().parse("test [foo=bar] { aa: bb; cc: dd } rule { ee: dd;  }"),
			Ok((
				vec![
					Rule {
						selectors: vec![SimpleSelector::AttributeSelector {
							tag_name: "test".to_string(),
							attribute: "foo".to_string(),
							op: AttributeSelectorOp::Eq,
							value: "bar".to_string(),
						}],
						declarations: vec![
							Declaration {
								name: "aa".to_string(),
								value: CSSValue::Keyword("bb".to_string()),
							},
							Declaration {
								name: "cc".to_string(),
								value: CSSValue::Keyword("dd".to_string()),
							}
						],
					},
					Rule {
						selectors: vec![SimpleSelector::TypeSelector {
							tag_name: "rule".to_string(),
						}],
						declarations: vec![Declaration {
							name: "ee".to_string(),
							value: CSSValue::Keyword("dd".to_string()),
						}],
					},
				],
				""
			))
		);
	}

	#[test]
	fn test_rule() {
		assert_eq!(
			rule().parse("test [foo=bar] {}"),
			Ok((
				Rule {
					selectors: vec![SimpleSelector::AttributeSelector {
						tag_name: "test".to_string(),
						attribute: "foo".to_string(),
						op: AttributeSelectorOp::Eq,
						value: "bar".to_string(),
					}],
					declarations: vec![],
				},
				""
			))
		);

		assert_eq!(
			rule().parse("test [foo=bar], testtest[piyo~=guoo] {}"),
			Ok((
				Rule {
					selectors: vec![
						SimpleSelector::AttributeSelector {
							tag_name: "test".to_string(),
							attribute: "foo".to_string(),
							op: AttributeSelectorOp::Eq,
							value: "bar".to_string(),
						},
						SimpleSelector::AttributeSelector {
							tag_name: "testtest".to_string(),
							attribute: "piyo".to_string(),
							op: AttributeSelectorOp::Contain,
							value: "guoo".to_string(),
						}
					],
					declarations: vec![],
				},
				""
			))
		);

		assert_eq!(
			rule().parse("test [foo=bar] { aa: bb; cc: dd; }"),
			Ok((
				Rule {
					selectors: vec![SimpleSelector::AttributeSelector {
						tag_name: "test".to_string(),
						attribute: "foo".to_string(),
						op: AttributeSelectorOp::Eq,
						value: "bar".to_string(),
					}],
					declarations: vec![
						Declaration {
							name: "aa".to_string(),
							value: CSSValue::Keyword("bb".to_string()),
						},
						Declaration {
							name: "cc".to_string(),
							value: CSSValue::Keyword("dd".to_string()),
						}
					],
				},
				""
			))
		);
	}

	#[test]
	fn test_declarations() {
		assert_eq!(
			declarations().parse("foo: bar; piyo: piyopiyo;"),
			Ok((
				vec![
					Declaration {
						name: "foo".to_string(),
						value: CSSValue::Keyword("bar".to_string()),
					},
					Declaration {
						name: "piyo".to_string(),
						value: CSSValue::Keyword("piyopiyo".to_string()),
					}
				],
				""
			))
		);
	}

	#[test]
	fn test_selectors() {
		assert_eq!(
			selectors().parse("test [foo=bar], a"),
			Ok((
				vec![
					SimpleSelector::AttributeSelector {
						tag_name: "test".to_string(),
						attribute: "foo".to_string(),
						op: AttributeSelectorOp::Eq,
						value: "bar".to_string(),
					},
					SimpleSelector::TypeSelector { tag_name: "a".to_string() }
				],
				""
			))
		);
	}

	#[test]
	fn test_simple_selector() {
		assert_eq!(simple_selector().parse("*"), Ok((SimpleSelector::UniversalSelector, "")));

		assert_eq!(
			simple_selector().parse("test"),
			Ok((SimpleSelector::TypeSelector { tag_name: "test".to_string() }, ""))
		);

		assert_eq!(
			simple_selector().parse("test [foo=bar]"),
			Ok((
				SimpleSelector::AttributeSelector {
					tag_name: "test".to_string(),
					attribute: "foo".to_string(),
					op: AttributeSelectorOp::Eq,
					value: "bar".to_string(),
				},
				""
			))
		);

		assert_eq!(
			simple_selector().parse(".test"),
			Ok((SimpleSelector::ClassSelector { class_name: "test".to_string() }, ""))
		);
	}

	#[test]
	fn test_declaration() {
		assert_eq!(
			declaration().parse("keykey:piyo"),
			Ok((
				Declaration {
					name: "keykey".to_string(),
					value: CSSValue::Keyword("piyo".to_string()),
				},
				""
			))
		);

		assert_eq!(
			declaration().parse("keyabc : piyo "),
			Ok((
				Declaration {
					name: "keyabc".to_string(),
					value: CSSValue::Keyword("piyo".to_string()),
				},
				" "
			))
		);

		assert_eq!(
			declaration().parse("keyhello : piyo "),
			Ok((
				Declaration {
					name: "keyhello".to_string(),
					value: CSSValue::Keyword("piyo".to_string()),
				},
				" "
			))
		);

		assert!(declaration().parse("aaaaa").is_err())
	}
}
