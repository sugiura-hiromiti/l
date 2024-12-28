use crate::hacker::*;
use crate::*;
use fetch::hackernews;
use num_traits::PrimInt;

#[component]
pub fn story_list(story: ReadOnlySignal<data::StoryItem,>,) -> Element {
	let data::StoryItem { title, by, score, time, kids, url, id, .. } = story();

	let url = url.as_deref().unwrap_or_default();
	let hostname = url
		.trim_start_matches("https://",)
		.trim_start_matches("http://",)
		.trim_start_matches("www.",);
	let score = count_of(score, "point",);
	let comments = count_of(kids.len(), "comment",);
	let time = time.format("%D %H:%M",);

	let pl = format!("pl{}", TW_PAD_W);
	let full_story = use_signal(|| None,);
	let preview_state = consume_context::<Signal<data::PreviewState,>,>();
	rsx! {
		div {
			class: format!("p{}", TW_PAD_W),
			onmouseenter: move |_| { hackernews::resolve_story(full_story, preview_state, id) },
			div { class: TW_FONT_L,
				a {
					class: TW_ANCHOR,
					href: url,
					onfocus: move |_event| { hackernews::resolve_story(full_story, preview_state, id) },
					"{title}"
				}
				a {
					class: "{TW_GRAY} no-underline",
					href: "https://news.ycombinator.com/from?site={hostname}",
					" ({hostname})"
				}
			}
			div { class: "{TW_FLEX_ROW} {TW_GRAY}",
				div { "{score}" }
				div { class: "w-2" }
				div { class: "{pl}", "by {by}" }
				div { class: "w-2" }
				div { class: "{pl}", "{time}" }
				div { class: "w-2" }
				div { class: "{pl}", "{comments}" }
			}
		}
	}
}

pub fn count_of<N: PrimInt + std::fmt::Display,>(count: N, unit: impl AsRef<str,>,) -> String {
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
