use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use dioxus_logger::tracing::Level;
use so_cold::consts::*;
use so_cold::*;

fn main() {
	// Init logger
	dioxus_logger::init(Level::INFO,).expect("failed to init logger",);
	info!("starting app");
	launch(app,);
}

fn app() -> Element {
	info!("enter app");
	use_context_provider(|| Signal::new(data::PreviewState::Unset,),);
	rsx! {
		div { class: format!("{TW_FLEX_ROW} w-full"),
			div { class: "w-1/2", components::stories {} }
			div { class: "w-1/2", components::preview {} }
		}
	}
}
