use so_cold::consts::*;
use so_cold::*;

fn main() {
	launch(app,);
}

fn app() -> Element {
	use_context_provider(|| Signal::new(hacker::data::PreviewState::Unset,),);
	let mut num = 0;
	let rand = use_hook(|| {
		num += 5;
		num
	},);
	let mut count = use_signal(|| 0,);

	let items = use_signal(|| vec!["goood", "night", "good"],);
	rsx! {
		document::Stylesheet { href: asset!("./assets/tailwind.css") }
		document::Title { "good night" }
		ul {
			for item in items.iter() {
				li { key: "{item}", "{item} {rand} {rand}" }
			}
		}
		button { onclick: move |_| count += 1, "inc" }
		"{count}"
		"{rand}"
		div { class: format!("{TW_FLEX_ROW} w-full"),
			div { class: "w-1/2", hacker::components::stories {} }
			div { class: "w-1/2", hacker::components::preview {} }
		}
	}
}
