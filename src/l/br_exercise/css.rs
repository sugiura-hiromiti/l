//! <https://github.com/tiny-browserbook/exercise-css>

pub mod css;

#[test]
fn tr_custom() {
	println!("{:?}", css::rule().parse("test [foo=bar] { }"));
	println!("{:?}", css::rule().parse("test [foo=bar] { aa:    bb;    cc    :dd;}",));
}
