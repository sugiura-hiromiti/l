//! types for components of this dioxus prj

use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize,)]
pub struct StoryPageData {
	#[serde(flatten)]
	pub item:     StoryItem,
	#[serde(default)]
	pub comments: Vec<CommentData,>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize,)]
pub struct CommentData {
	pub id: i64,

	#[serde(default)]
	pub by: String,

	#[serde(default)]
	pub text: String,

	#[serde(with = "chrono::serde::ts_seconds")]
	pub time: DateTime<Utc,>,

	#[serde(default)]
	pub kids: Vec<i64,>,

	#[serde(default)]
	pub sub_comments: Vec<CommentData,>,
	pub ty:           String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize,)]
pub struct StoryItem {
	pub id:    i64,
	pub title: String,
	pub url:   Option<String,>,
	pub text:  Option<String,>,

	#[serde(default)]
	pub by: String,

	#[serde(default)]
	pub score: i64,

	#[serde(default)]
	pub descendants: i64,

	#[serde(with = "chrono::serde::ts_seconds")]
	pub time: DateTime<Utc,>,

	#[serde(default)]
	pub kids:   Vec<i64,>,
	pub r#type: String,
}

#[derive(Debug, Clone,)]
pub enum PreviewState {
	Unset,
	Loading,
	Loaded(StoryPageData,),
}
