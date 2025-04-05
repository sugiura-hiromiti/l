//! <https://github.com/tiny-browserbook/exercise-html>
pub mod dom;
pub mod html;

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn attrmap_from() {
		let from_attr = dom::AttrMap::from([("attribute".to_string(), "name".to_string())]);
		let mut ins_attr = dom::AttrMap::new();
		ins_attr.insert("attribute".to_string(), "name".to_string());
		assert_eq!(from_attr, ins_attr);
	}
}
