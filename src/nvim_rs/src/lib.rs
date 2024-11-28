#![allow(unused_variables, unused_imports, dead_code, unreachable_code)]
use nvim_oxi::api::get_option_value;
use nvim_oxi::api::opts::OptionOpts;
use nvim_oxi::api::set_option_value;
use nvim_oxi::api::StringOrListOfStrings;
use nvim_oxi::Object;
use nvim_oxi::ObjectKind;

#[nvim_oxi::plugin]
fn o_o() -> Result<(), nvim_oxi::api::Error,> {
	let opts = OptionOpts::builder();
	let light = "light".to_object();
	panic!("🫠-----------------------🫠");
	set_option_value("background", light, &opts.build(),)
}

#[cfg(test)]
mod tests {
	use nvim_oxi::api::StringOrListOfStrings;

	use super::*;

	#[nvim_oxi::test(cmd = "lua print'🫠entered nvim instance'")]
	fn empty_oxi_test() {}

	//	#[nvim_oxitest]
	fn o_o_test() -> Result<(), nvim_oxi::api::Error,> {
		o_o()?;
		assert_eq!(
			get_option_value::<Object,>(
				"background",
				&OptionOpts::builder().build()
			)
			.expect("Failed to get option value"),
			"light".to_object()
		);

		let bg_dflt = nvim_oxi::api::get_option_info2(
			"background",
			&OptionOpts::builder().build(),
		)?
		.default;
		assert_eq!(bg_dflt, "dark".to_object());
		Ok((),)
	}
}
