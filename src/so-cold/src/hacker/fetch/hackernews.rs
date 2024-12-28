//! definition of [hacker news](https://news.ycombinator.com) api

use crate::hacker::*;
use crate::*;
use futures::future;

pub static BASE_API_URL: &str = "https://hacker-news.firebaseio.com/v0/";
pub static ITEM_API: &str = "item/";
pub static USER_API: &str = "user/";
const COMMENT_DEPTH: i64 = 2;

pub async fn story_preview(id: i64,) -> Result<data::StoryItem, reqwest::Error,> {
	let url = format!("{BASE_API_URL}{ITEM_API}{id}.json");
	reqwest::get(&url,).await?.json().await
}

pub async fn top_stories(count: usize,) -> Result<Vec<data::StoryItem,>, reqwest::Error,> {
	let url = format!("{BASE_API_URL}topstories.json");
	let story_ids = &reqwest::get(&url,).await?.json::<Vec<i64,>>().await?[..count];
	let story_futures =
		story_ids[..story_ids.len().min(count,)].iter().map(|&story_id| story_preview(story_id,),);

	let stories = future::join_all(story_futures,)
		.await
		.into_iter()
		.filter_map(|story| story.ok(),)
		.collect();
	Ok(stories,)
}

pub async fn story(id: i64,) -> Result<data::StoryPageData, reqwest::Error,> {
	let url = format!("{BASE_API_URL}{ITEM_API}{id}.json");
	let mut story = reqwest::get(&url,).await?.json::<data::StoryPageData>().await?;
	let comment_futures = story.item.kids.iter().map(|&id| comment(id,),);
	let comments =
		future::join_all(comment_futures,).await.into_iter().filter_map(|c| c.ok(),).collect();
	story.comments = comments;
	Ok(story,)
}

pub async fn resolve_story(
	mut full_story: Signal<Option<data::StoryPageData,>,>,
	mut preview_state: Signal<data::PreviewState,>,
	story_id: i64,
) {
	if let Some(cached,) = full_story.as_ref() {
		*preview_state.write() = data::PreviewState::Loaded(cached.clone(),);
		return;
	}

	*preview_state.write() = data::PreviewState::Loading;
	if let Ok(story,) = story(story_id,).await {
		*preview_state.write() = data::PreviewState::Loaded(story.clone(),);
		*full_story.write() = Some(story,);
	}
}

#[async_recursion::async_recursion(?Send)]
pub async fn comment_with_depth(
	id: i64, depth: i64,
) -> Result<data::CommentData, reqwest::Error,> {
	let url = format!("{BASE_API_URL}{ITEM_API}{id}.json");
	let mut comment = reqwest::get(&url,).await?.json::<data::CommentData>().await?;

	if depth > 0 {
		let sub_comment_futures =
			comment.kids.iter().map(|&id| comment_with_depth(id, depth - 1,),);
		comment.sub_comments = future::join_all(sub_comment_futures,)
			.await
			.into_iter()
			.filter_map(|comment| comment.ok(),)
			.collect();
	}
	Ok(comment,)
}

pub async fn comment(id: i64,) -> Result<data::CommentData, reqwest::Error,> {
	comment_with_depth(id, COMMENT_DEPTH,).await
}
