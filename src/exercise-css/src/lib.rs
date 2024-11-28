pub mod css;

#[test]
fn tr_custom() {
	println!(
		"{:?}",
		css::rule().parse("test [foo=bar] { }")
	);
	println!(
		"{:?}",
		css::rule().parse("test [foo=bar] { aa:    bb;    cc    :dd;}",)
	);
}
