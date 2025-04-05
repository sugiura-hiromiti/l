//! [gas project url](https://script.google.com/home/projects/1rf8bIOJE2_QU15ktQH5UuA_tR4_zne_SVPGIEGdEu5vYwdOSYH6FFJLo/edit)

#[no_mangle]
pub fn add(left: i32, right: i32,) -> i32 {
	left + right
}

#[no_mangle]
pub fn say<'a,>() -> &'a str {
	"🫠 from wasm 0w0"
}

#[no_mangle]
pub fn return_true() -> bool {
	true
}
