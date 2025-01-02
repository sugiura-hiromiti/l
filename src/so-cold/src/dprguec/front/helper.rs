use crate::*;
use dprguec::back;
use dprguec::front::Route;

#[macro_export]
macro_rules! id {
	() => {
		id!("")
	};
	($e:expr) => {
		format!("{}_at_{}_{}", module_path!(), line!(), $e)
	};
}

pub fn classify_entry(way: String,) -> Route {
	let way_s = way.as_str();
	if way_s == "sign_up" {
		Route::SignUp {}
	} else if way_s == "sign_in" {
		Route::SignIn {}
	} else {
		Route::PageNotFound { segments: vec![way], }
	}
}

/// # Params
///
/// - page must be form of query (which means no whitespace)
pub fn entry_ui(page: String, inputs: Vec<&str,>,) -> Element {
	let mut user_cx = use_context::<Signal<entity::User,>,>();
	let button_style = TW_BUTTON_COMMON.join(" ",);
	let button = rsx! {
		input {
			id: id!(),
			r#type: "submit",
			class: "{button_style} {TW_FONT_L} rounded-l-lg hover:rounded-lg w-1/3",
			value: dequerize(&page),
		}
	};

	let input_fields = rsx! {
		{
		    inputs
		        .iter()
		        .map(|&i_ty| {
		            let mut display_field_type = i_ty;
		            if i_ty == "name" {
		                display_field_type = "text";
		            }
		            rsx! {
			div { id: id!(), class: TW_MARGIN_S,
				{i_ty}
				div { id: id!(), class: TW_DIAG_COLORS[0], {user_cx().get(i_ty).clone()} }
			}
			input {
				r#type: display_field_type,
				name: i_ty,
				class: "{TW_INPUT} w-full max-w-96",
			}
		}
		        })
		}
		input { r#type: "hidden", name: "way", value: page.clone() }
	};

	let verify = move |e: Event<FormData,>| async move {
		let values = e.values();
		let mut user = entity::User::from(&values,);
		tracing::debug!("{user:?}");
		let way =
			values.get("way",).expect(&format!("`way` key must exist in form data",),).as_value();
		//use_server_future(|| back::verify_user(user, way,),);
		tracing::debug!("way is: {way}");
		let verify_rslt = back::verify_user(user.clone(), way,).await;
		match verify_rslt {
			Ok(_,) => {
				*user_cx.write() = user;

				router().push(Route::Home,);
			},
			Err(e,) => {
				user.diag();
				*user_cx.write() = user;
				tracing::debug!("{e}");
			},
		}
	};

	//let vverify = move |e| tracing::debug!("{e:?}");

	rsx! {
		form { id: id!(), onsubmit: verify, {card1(button, input_fields)} }
	}
}

// fn request_user_verification(e:Event<FormData>, page: String) {
//     let values=e.values();
//     let user=entity::User::from(&values);
//     tracing::debug!("{user:?}");
//     let way=values.get("way").expect(msg)
// }

pub fn card1(left: Element, right: Element,) -> Element {
	rsx! {
		div {
			id: id!(),
			class: "{TW_FLEX_ROW} m-[10%] {TW_BG_NEUTRAL} rounded-lg",
			{left}
			div { id: id!(), class: "w-2/3 {TW_PAD_M}", {right} }
		}
	}
}

// pub fn querize(q: &str,) -> String {
// 	q.replace(" ", "_",)
// }

pub fn dequerize(q: &str,) -> String {
	q.replace("_", " ",)
}
