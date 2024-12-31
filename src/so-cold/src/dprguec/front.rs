use crate::*;
use page::*;

mod helper;
pub mod page;

#[derive(Routable, Clone, PartialEq,)]
#[rustfmt::skip]
pub enum Route {
	#[route("/")]
	Entrypoint,

	#[nest("/entry")]
		#[redirect("/:way", |way: String| helper::classify_entry(way))]
		#[route("/")]
		SignUp,
		#[route("/")]
		SignIn,
	#[end_nest]

	#[layout(Header)]
		#[route("/home")]
		Home,
		#[route("/article/:id")]
		Article { id: i32 },
	#[end_layout]


	#[route("/:..segments")]
	PageNotFound { segments: Vec<String,>, },
}
