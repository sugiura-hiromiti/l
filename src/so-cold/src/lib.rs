//! library part of so_cold crate

pub(crate) use consts::*;
pub(crate) use dioxus::prelude::*;
pub(crate) use dioxus_logger::tracing::info;

pub mod components;
pub mod data;
pub mod fetch;
pub mod post;

pub mod consts {
	pub const TW_GRAY: &str = "text-gray-500";
	pub const TW_PAD_W: &str = "-2";
	pub const TW_ANCHOR: &str = "text-blue-400 font-light";
	pub const TW_FONT_L: &str = "text-2xl";
	pub const TW_FONT_XL: &str = "text-6xl";
	pub const TW_FLEX_ROW: &str = "flex flex-row";
}
