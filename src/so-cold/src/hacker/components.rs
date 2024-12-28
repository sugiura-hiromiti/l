//! collection of components which is used by app()

use crate::hacker::*;
use crate::*;
use fetch::hackernews;

pub fn stories() -> Element {
	let stories = use_resource(move || hackernews::top_stories(10,),);
	match &*stories.read_unchecked() {
		Some(Ok(list,),) => {
			rsx! {
				div {
					for story in list {
						post::story_list { story: story.clone() }
					}
				}
			}
		},
		Some(Err(e,),) => rsx! { "An error occurred while fetching stories: {e}" },
		None => rsx! { "loading items" },
	}
}

pub fn preview() -> Element {
	use data::PreviewState;

	let p = format!("p{}", TW_PAD_W);
	let state = consume_context::<Signal<PreviewState,>,>();
	match state() {
		PreviewState::Unset => rsx! { "hover over a story to preview it" },
		PreviewState::Loading => rsx! { "loading🫠" },
		PreviewState::Loaded(story,) => rsx! {
			div { class: "{p}",
				div { class: TW_FONT_L,
					a { href: story.item.url, "{story.item.title}" }
				}
				div { dangerous_inner_html: story.item.text }
				for c in story.comments {
					self::comment { com: c.clone() }
				}
			}
		},
	}
}

#[component]
fn comment(com: data::CommentData,) -> Element {
	let p = format!("p{TW_PAD_W}");
	rsx! {
		div { class: "{p}",
			div { class: TW_GRAY, "by {com.by}" }
			div { dangerous_inner_html: "{com.text}" }
			for kid in &com.sub_comments {
				self::comment { com: kid.clone() }
			}
		}
	}
}
