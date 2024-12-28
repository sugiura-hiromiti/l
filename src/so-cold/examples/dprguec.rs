//! :TODO
//! - [ ] create entrypoint
//! - [ ] connect to db
//! - [ ] route
//! - [ ] implement markdown parser

use so_cold::dprguec::*;
use so_cold::*;
use std::rc::Rc;

fn main() {
	dioxus::launch(app,);
}

#[component]
fn app() -> Element {
	use_context_provider(|| Signal::new(None::<Rc<MountedData,>,>,),);
	tracing::debug!("{}app", module_path!());
	let mut x = 0;
	let _ = async {
		x = back::get_article(30,).await.expect("failed to get id",);
	};

	rsx! {
		document::Stylesheet { href: asset!("./assets/tailwind.css") }
		document::Title{ "original title" }
		//div { {x.to_string()} }
		front::title {}
		front::logo {}
		front::go_top {}

		{(0..100).map(|i| rsx! {
			front::article { ttl: "abc", body: "def", id: i }
		})}
	}
}
