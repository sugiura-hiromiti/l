use crate::*;
use std::rc::Rc;

#[component]
pub fn title() -> Element {
	let mut cx = use_context::<Signal<Option<Rc<MountedData,>,>,>,>();
	tracing::debug!("{}title", module_path!());
	rsx! {
		h1 {
			id: "ttl",
			onmounted: move |e| {
				tracing::debug!("mounted title");
				cx.set(Some(e.data()));
			},
			class: consts::TW_FONT_XL,
			"Does People Really Move Underground in Extreme Cold?"
		}
	}
}

#[component]
pub fn logo() -> Element {
	tracing::debug!("{}logo", module_path!());
	rsx! {
		img { src: "https://www.mothandrust.co.uk/files/underground.png" }
	}
}

#[component]
pub fn go_top() -> Element {
	tracing::debug!("{}go_top", module_path!());
	let cx = use_context::<Signal<Option<Rc<MountedData,>,>,>,>();
	tracing::debug!("1----------{}", cx.as_ref().is_some());
	let top = move |event| async move {
		tracing::debug!("{:?}", event);
		tracing::debug!("clicked!");
		tracing::debug!("in async go_top::scroll_top");
		//let _a = dprguec::back::get_article(30,).await.expect("failed to unwrap id",);
		if let Some(ttl,) = cx.as_ref().as_ref() {
			let _ = ttl.scroll_to(ScrollBehavior::Smooth,).await;
		} else {
			tracing::debug!("no signal-----------------------------------------------------");
			panic!("failed to get signal")
		}
	};

	// let top = move |event| {
	// 	tracing::debug!("{:?}", event);
	// 	tracing::debug!("clicked!");
	// 	async fn scroll_top(cx: Signal<Option<Rc<MountedData,>,>,>,) {
	// 		tracing::debug!("2----------{}", cx.as_ref().is_some());
	// 		tracing::debug!("in async go_top::scroll_top");
	// 		let _a = dprguec::back::get_article(30,).await.expect("failed to unwrap id",);
	// 		if let Some(ttl,) = cx.as_ref().as_ref() {
	// 			let _ = ttl.scroll_to(ScrollBehavior::Smooth,).await;
	// 		} else {
	// 			tracing::debug!("no signal-----------------------------------------------------");
	// 			//panic!("failed to get signal")
	// 		}
	// 	}
	//
	// 	tracing::debug!("3----------{}", cx.as_ref().is_some());
	// 	let _ = async move {
	// 		let _a = scroll_top(cx,).await;
	// 	};
	// };

	rsx! {
		div {
			id: "go_top",
			onclick: top,
			class: "{consts::TW_FONT_XL} fixed bottom-0 right-0 w-20 h-20 flex justify-center items-center cursor-pointer",
			" "
		}
	}
}

#[component]
pub fn article(ttl: String, body: String, id: i32,) -> Element {
	rsx! {
		h2 {
			id: format!("{}h2{id}", module_path!()),
			class: "{consts::TW_FONT_L}",
			{ttl}
		}
		p { id: format!("{}p{id}", module_path!()), {body} }
	}
}
