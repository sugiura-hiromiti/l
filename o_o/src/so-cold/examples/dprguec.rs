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
	let sig = use_signal(|| None::<Rc<MountedData,>,>,);
	let user = use_signal(|| entity::User::new(),);
	use_context_provider(|| sig,);
	use_context_provider(|| user,);
	tracing::debug!("{}app", module_path!());

	rsx! {
		document::Stylesheet { href: asset!("./assets/tailwind.css") }
		document::Title { "original title" }
		Router::<front::Route>{}
	}
}
