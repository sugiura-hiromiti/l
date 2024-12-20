use crate::*;
use num_traits::PrimInt;

#[component]
pub fn story_list(story: ReadOnlySignal<data::StoryItem,>,) -> Element {
    info!("enter story_list");
	let data::StoryItem { ttl, by, score, time, kids, url, .. } = &*story.read();

	let url = url.as_deref().unwrap_or_default();
	let hostname = url
		.trim_start_matches("https://",)
		.trim_start_matches("http://",)
		.trim_start_matches("www.",);
	let score = count_of(*score, "point",);
	let comments = count_of(kids.len(), "comment",);
	let time = time.format("%D %H:%M",);

	let pl = format!("pl{}", TW_PAD_W);
	let mut preview_state = consume_context::<Signal<data::PreviewState,>,>();
	rsx! {
		div {
			class: format!("p{} relative", TW_PAD_W),
			onmouseenter: move |_| {
				*preview_state.write() = data::PreviewState::Loaded(data::StoryPageData {
					item: story(),
					comments: vec![],
				});
			},
			div { class: TW_FONT_L,
				a {
					class: TW_ANCHOR,
					href: url,
					onfocus: move |_event| {
						*preview_state.write() = data::PreviewState::Loaded(data::StoryPageData {
							item: story(),
							comments: vec![],
						});
					},
					"{ttl}"
				}
				a {
					class: "{TW_GRAY} no-underline",
					href: "https://new.ycombinator.com/from?site={hostname}",
					" ({hostname})"
				}
			}
			div { class: "{TW_FLEX_ROW} {TW_GRAY}",
				div { "{score}" }
				div { class: "{pl}", "by {by}" }
				div { class: "{pl}", "{time}" }
				div { class: "{pl}", "{comments}" }
			}
		}
	}
}

pub fn count_of<N: PrimInt + std::fmt::Display,>(count: N, unit: impl AsRef<str,>,) -> String {
	info!("count is {count}");
	let type_name = std::any::type_name_of_val(&count,);
	let count = if !type_name.contains('u',) {
		// type of `count` must be signed
		count.unsigned_shl(1,).unsigned_shr(1,)
	} else {
		count
	};
	let count = format!("{count}").parse::<u128>().expect("failed to parse PrimInt into u128",);
	let mut s = unit.as_ref().to_string();
	if 1 < count {
		s.push('s',);
	}
	format!("{count} {s}")
}
