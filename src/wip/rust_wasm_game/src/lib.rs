mod game;
//#[macro_use]
mod browser;
mod engine;
use engine::GameLoop;
use game::WalkTheDog;
use wasm_bindgen::prelude::*;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue,> {
	// redirect any panic to browser's console
	console_error_panic_hook::set_once();

	// Your code goes here!
	browser::spawn_local(async move {
		let game = WalkTheDog::new();
		GameLoop::start(game,).await.expect("Could not start game loop",);
	},);
	Ok((),)
}
