use std::collections::HashMap;

use dioxus::html::FormValue;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq,)]
pub struct User {
	pub name:     Option<String,>,
	pub email:    Option<String,>,
	pub password: Option<String,>,
}

//  TODO: generate verify_* fn by proc macro
impl User {
	pub fn new() -> Self {
		User {
			name:     Some(Default::default(),),
			email:    Some(Default::default(),),
			password: Some(Default::default(),),
		}
	}

	pub fn diag(&mut self,) -> bool {
		let fill_warn = |o: &mut Option<String,>, msg: &str| {
			o.replace(msg.to_string(),);
			false
		};
		let name = self.name_diag(fill_warn,);
		let email = self.email_diag(fill_warn,);
		let passwd = self.passwd_diag(fill_warn,);
		name && email && passwd
	}

	pub fn verify(&mut self, is_sign_up: bool,) -> bool {
		let nothing = |_o: &mut Option<String,>, _msg: &str| false;
		let name = if is_sign_up { self.name_diag(nothing,) } else { true };
		name && self.email_diag(nothing,) && self.passwd_diag(nothing,)
	}

	fn name_diag(&mut self, f: impl Fn(&mut Option<String,>, &str,) -> bool,) -> bool {
		if self.name.is_some() {
			if self.name.as_ref().unwrap().is_empty() {
				f(&mut self.name, "name can't be blank",)
			} else {
				true
			}
		} else {
			if f(&mut self.name, "name can't be blank",) { self.name_diag(f,) } else { false }
		}
	}

	fn email_diag(&mut self, f: impl Fn(&mut Option<String,>, &str,) -> bool,) -> bool {
		if self.email.is_some() {
			if self.email.as_ref().unwrap().is_empty() {
				f(&mut self.email, "email can't be blank",)
			} else {
				true
			}
		} else {
			if f(&mut self.email, "email can't be blank",) { self.email_diag(f,) } else { false }
		}
	}

	fn passwd_diag(&mut self, f: impl Fn(&mut Option<String,>, &str,) -> bool,) -> bool {
		if self.password.is_some() {
			let pass = self.password.as_ref().unwrap();
			let is_short = pass.len() < 12;
			let miss = !pass.contains(|c| {
				('a' <= c && 'z' >= c) || ('A' <= c && 'Z' >= c) || ('1' <= c && '9' >= c)
			},);
			if is_short || miss {
				f(
					&mut self.password,
					"password must contain each of lower case letter, upper case letter, number \
					 at least once. It also have to satisfy length over 11",
				)
			} else {
				true
			}
		} else {
			if f(
				&mut self.password,
				"password must contain each of lower case letter, upper case letter, number at \
				 least once. It also have to satisfy length over 11",
			) {
				self.passwd_diag(f,)
			} else {
				false
			}
		}
	}

	pub fn get(&self, s: impl AsRef<str,>,) -> &Option<String,> {
		match s.as_ref() {
			"name" => &self.name,
			"email" => &self.email,
			"password" => &self.password,
			_ => panic!("no field named {}", s.as_ref()),
		}
	}

	pub fn db_insert(&self,) -> (String, String, String, String, String,) {
		(
			self.name.clone().unwrap(),
			self.email.clone().unwrap(),
			self.password.clone().unwrap(),
			chrono::Local::now().to_string(),
			chrono::Local::now().to_string(),
		)
	}
}

impl From<&HashMap<String, FormValue,>,> for User {
	fn from(value: &HashMap<String, FormValue,>,) -> Self {
		Self {
			name:     extract_field(value, "name",),
			email:    extract_field(value, "email",),
			password: extract_field(value, "password",),
		}
	}
}

fn extract_field(hm: &HashMap<String, FormValue,>, s: impl AsRef<str,>,) -> Option<String,> {
	hm.get(s.as_ref(),).map(|fv| fv.as_ref().to_vec().join(" ",),)
}

#[cfg(test)]
mod tests {
	use super::*;

	fn valid_user() -> User {
		User {
			name:     Some("aaa".to_string(),),
			email:    Some("pishadon57@gmail.com".to_string(),),
			password: Some("123412341234aA".to_string(),),
		}
	}

	fn diagnosed_user() -> User {
		User {
			name:     Some("name can't be blank".to_string(),),
			email:    Some("email can't be blank".to_string(),),
			password: Some(
				"password must contain each of lower case letter, upper case letter, number at \
				 least once. It also have to satisfy length over 11"
					.to_string(),
			),
		}
	}

	#[test]
	fn user_verify() {
		let mut valid = valid_user();
		let mut invalid = User::new();
		assert!(!User::new().verify(true));
		assert!(!invalid.diag(), "{invalid:?}");
		assert!(valid.diag(), "{valid:?}");
		assert!(valid_user().verify(true));
	}

	#[test]
	fn fill_by_diag() {
		let mut user = User::new();
		user.diag();
		assert_eq!(user, diagnosed_user());
	}
}
