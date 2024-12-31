//! library part of so_cold crate

pub use consts::*;
pub use dioxus::document;
pub use dioxus::logger::tracing;
pub use dioxus::prelude::*;
//pub use dioxus::web;

pub mod dprguec;
pub mod entity;
pub mod hacker;

pub mod consts {
	pub const TW_GRAY: &str = "text-gray-500";
	pub const TW_DIAG_COLORS: [&str; 4] =
		["text-rose-500", "text-yellow-500", "text-lime-500", "text-neutral-500",];
	pub const TW_BG_NEUTRAL: &str = "bg-neutral-300";
	pub const TW_ANCHOR: &str = "text-blue-400 font-light";
	pub const TW_FONT_L: &str = "text-2xl";
	pub const TW_FONT_XL: &str = "text-4xl";
	pub const TW_FLEX_ROW: &str = "flex flex-row";
	pub const TW_CENTERIZE: &str = "flex justify-center items-center";
	pub const TW_REVERT_COLOR: &str = "bg-green-950 text-white";
	pub const TW_INPUT: &str = "border rounded border-slate-800";
	pub const TW_PAD_S: &str = "p-3";
	pub const TW_PAD_M: &str = "p-9";
	pub const TW_PAD_L: &str = "p-25";
	pub const TW_MARGIN_S: &str = "m-3";
	pub const TW_MARGIN_M: &str = "m-9";
	pub const TW_MARGIN_L: &str = "m-25";
	pub const TW_HOVER_TRANS: &str =
		"transition-all hover:opacity-80 hover:brightness-110 hover:shadow-xl";
	pub const TW_BUTTON_COMMON: [&str; 5] =
		[TW_REVERT_COLOR, TW_PAD_M, TW_HOVER_TRANS, TW_CENTERIZE, "cursor-pointer",];
}
