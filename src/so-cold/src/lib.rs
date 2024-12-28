//! library part of so_cold crate

pub use consts::*;
pub use dioxus::document;
pub use dioxus::logger::tracing;
pub use dioxus::prelude::*;
//pub use dioxus::web;

pub mod dprguec;
pub mod hacker;

pub mod consts {
	pub const TW_GRAY: &str = "text-gray-500";
	pub const TW_BG_SLATE: &str = "bg-slate-400";
	pub const TW_PAD_W: &str = "-2";
	pub const TW_ANCHOR: &str = "text-blue-400 font-light";
	pub const TW_FONT_L: &str = "text-2xl";
	pub const TW_FONT_XL: &str = "text-4xl";
	pub const TW_FLEX_ROW: &str = "flex flex-row";
}
