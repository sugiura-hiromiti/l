//! provides ui like page itself & routing

use crate::*;
use std::rc::Rc;

use super::helper;

#[component]
pub fn Entrypoint() -> Element {
	let enter_way = vec!["sign_up", "sign_in"];

	let button_style = TW_BUTTON_COMMON.join(" ",);
	rsx! {
		div { class: "{TW_CENTERIZE} my-10 {TW_FONT_L}", "welcome" }
		{enter_way.iter().map(|w| rsx! {
			Link {
				to: "/entry/{w}",
				id: format!("{}{}{}", module_path!(), line!(), helper::dequerize(w)),
				class: "w-40 mx-auto my-10 py-1 rounded-md {button_style}",
				{w}
			}
		})}
	}
}

#[component]
pub fn PageNotFound(segments: Vec<String,>,) -> Element {
	rsx! {
	   div { class: TW_FONT_XL, "Page Not Found" }
		br {}
		{segments.into_iter().map(|s| rsx! {
			{format!("/{s}")}
		})}
	}
}

#[component]
pub fn SignUp() -> Element {
	helper::entry_ui("sign_up".to_string(), vec!["name", "email", "password"],)
}

#[component]
pub fn SignIn() -> Element {
	helper::entry_ui("sign_in".to_string(), vec!["email", "password"],)
}

#[component]
pub fn Header() -> Element {
	rsx! {
		div { "sidebar is currently implementing" }
		Outlet::<dprguec::front::Route> {}
	}
}

#[component]
pub fn Home() -> Element {
	let user_cx = use_context::<Signal<entity::User,>,>();
	rsx! {
		"home"
		div { {format!("{:?}", user_cx.read())} }
	}
}

#[component]
pub fn Article(id: i32,) -> Element {
	rsx! {
		document::Stylesheet { href: asset!("./assets/tailwind.css") }
		document::Title { "original title" }
		Title {}
		Logo {}
		GoTop {}

		{(0..100).map(|i| rsx! {
			Content { ttl: "abc", body: "def", id: i }
		})}
	}
}

#[component]
pub fn Title() -> Element {
	let mut sig = use_context::<Signal<Option<Rc<MountedData,>,>,>,>();
	tracing::debug!("{}title", module_path!());

	let mount = move |e: Event<MountedData,>| {
		tracing::debug!("mounted title");
		sig.set(Some(e.data(),),);
	};

	rsx! {
		h1 { id: id!(), onmounted: mount, class: TW_FONT_XL,
			"Does People Really Move Underground in Extreme Cold?"
		}
	}
}

#[component]
pub fn Logo() -> Element {
	tracing::debug!("{}logo", module_path!());
	rsx! {
		img { src: "https:www.mothandrust.co.uk/files/underground.png" }
	}
}

#[component]
pub fn GoTop() -> Element {
	let sig = use_context::<Signal<Option<Rc<MountedData,>,>,>,>();
	tracing::debug!("{}go_top", module_path!());
	tracing::debug!("1----------{}", sig.as_ref().is_some());
	let top = move |event| async move {
		tracing::debug!("{:?}", event);
		tracing::debug!("clicked!");
		let a = dprguec::back::get_article(30,).await.expect("failed to unwrap id",);
		tracing::debug!("a is {a}");
		if let Some(ttl,) = sig.as_ref() {
			let _a =
				ttl.scroll_to(ScrollBehavior::Smooth,).await.expect("failed to scroll to top",);
		} else {
			tracing::error!("no signals")
		}
	};

	rsx! {
		button {
			id: id!(),
			onclick: top,
			class: "{TW_FONT_XL} fixed bottom-0 right-0 w-20 h-20 {TW_CENTERIZE} cursor-pointer",
			" "
		}
	}
}

#[component]
pub fn Content(ttl: String, body: String, id: i32,) -> Element {
	rsx! {
		h2 { id: id!(), class: "{TW_FONT_L}", {ttl} }
		p { id: id!(), {body} }
	}
}
