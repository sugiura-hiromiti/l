//! definition of [hacker news](https://news.ycombinator.com) api

use crate::*;
use futures::future;

pub static BASE_API_URL: &str = "https://hacker-news.firebaseio.com/v0/";
pub static ITEM_API: &str = "item/";
pub static USER_API: &str = "user/";
const COMMENT_DEPTH: i64 = 2;

pub async fn story_preview(id: i64,) -> Result<data::StoryItem, reqwest::Error,> {
	info!("enter story_preview");
	let url = format!("{BASE_API_URL}{ITEM_API}{id}.json");
	reqwest::get(&url,).await?.json().await
}

pub async fn top_stories(count: usize,) -> Result<Vec<data::StoryItem,>, reqwest::Error,> {
	info!("enter top_stories");
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

#[async_recursion::async_recursion(?Send)]
pub async fn comment_with_depth(
	id: i64, depth: i64,
) -> Result<data::CommentData, reqwest::Error,> {
	info!("enter comment_with_depth");
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
	info!("enter comment");
	comment_with_depth(id, COMMENT_DEPTH,).await
}
