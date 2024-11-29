use self::red_hat_boy_states::*;
use crate::browser;
use crate::engine;
use crate::engine::Game;
use crate::engine::Point;
use crate::engine::Rect;
use crate::engine::Renderer;
use crate::log;
use anyhow::Result;
use core::panic;
use gloo_utils::format::JsValueSerdeExt;
use serde::Deserialize;
use std::collections::HashMap;
use web_sys::HtmlImageElement;

pub struct WalkTheDog {
	rhb: Option<RedHatBoy,>,
}

impl WalkTheDog {
	pub fn new() -> Self { WalkTheDog { rhb: None, } }
}

impl Game for WalkTheDog {
	async fn init(&mut self,) -> Result<(),> {
		let img = engine::load_img("rhb.png",).await?;
		let sheet: Sheet = browser::fetch_json("rhb.json",)
			.await
			.expect("Could not fetch rhb.json",)
			.into_serde()?;
		self.rhb = Some(RedHatBoy::new(sheet, img,),);
		Ok((),)
	}

	fn update(&mut self, keystat: &crate::engine::KeyState,) {
		let mut vel = Point { x: 0, y: 0, };
		if keystat.is_pressed("ArrowDown",) {
			vel.y += 3;
		}
		if keystat.is_pressed("ArrowUp",) {
			vel.y -= 3;
		}
		if keystat.is_pressed("ArrowRight",) {
			vel.x += 3;
			self.rhb.as_mut().unwrap().run_right();
		}
		if keystat.is_pressed("ArrowLeft",) {
			vel.x -= 3;
		}
		self.rhb.as_mut().unwrap().update();
	}

	fn draw(&self, rndr: &Renderer,) { self.rhb.as_ref().unwrap().draw(rndr,); }
}

#[derive(Deserialize, Clone,)]
struct SheetRect {
	x: i16,
	y: i16,
	w: i16,
	h: i16,
}

#[derive(Deserialize, Clone,)]
struct Cell {
	frame: SheetRect,
}

#[derive(Deserialize, Clone,)]
pub struct Sheet {
	frames: HashMap<String, Cell,>,
}

struct RedHatBoy {
	state_machine: RedHatBoyStateMachine,
	sprite_sheet:  Sheet,
	image:         HtmlImageElement,
}

impl RedHatBoy {
	fn new(sheet: Sheet, image: HtmlImageElement,) -> Self {
		RedHatBoy {
			state_machine: RedHatBoyStateMachine::Idle(RedHatBoyState::new(),),
			sprite_sheet: sheet,
			image,
		}
	}

	fn draw(&self, renderer: &Renderer,) {
		let frame_name = format!(
			"{} ({}).png",
			self.state_machine.frame_name(),
			(self.state_machine.context().frame / 3) + 1,
		);
		let sprite = self
			.sprite_sheet
			.frames
			.get(&frame_name,)
			.expect("Cell not found",);

		let frame = &Rect {
			x: sprite.frame.x.into(),
			y: sprite.frame.y.into(),
			w: sprite.frame.w.into(),
			h: sprite.frame.h.into(),
		};

		renderer.clear(frame,);
		renderer.draw_image(
			&self.image,
			frame,
			&Rect {
				x: self.state_machine.context().position.x.into(),
				y: self.state_machine.context().position.y.into(),
				w: sprite.frame.w.into(),
				h: sprite.frame.h.into(),
			},
		);
		renderer.clear(frame,);
	}

	fn update(&mut self,) { self.state_machine = self.state_machine.update(); }

	fn run_right(&mut self,) {
		self.state_machine = self.state_machine.transition(Event::Run,);
	}
}

#[derive(Copy, Clone,)]
enum RedHatBoyStateMachine {
	Idle(RedHatBoyState<Idle,>,),
	Running(RedHatBoyState<Running,>,),
}

// TODO: try changing `From<A> for B` to `From<B> for A`. What will happen?
impl From<RedHatBoyState<Running,>,> for RedHatBoyStateMachine {
	fn from(state: RedHatBoyState<Running,>,) -> Self {
		RedHatBoyStateMachine::Running(state,)
	}
}

impl RedHatBoyStateMachine {
	fn transition(self, event: Event,) -> Self {
		match (self, event,) {
			(RedHatBoyStateMachine::Idle(state,), Event::Run,) => {
				state.run().into()
			},
			_ => self,
		}
	}

	// TODO: implement these 2 fn by using macro
	fn frame_name(&self,) -> &str {
		match self {
			RedHatBoyStateMachine::Idle(state,) => state.frame_name(),
			RedHatBoyStateMachine::Running(state,) => state.frame_name(),
		}
	}

	fn context(&self,) -> &RedHatBoyContext {
		match self {
			RedHatBoyStateMachine::Idle(state,) => state.context(),
			RedHatBoyStateMachine::Running(state,) => state.context(),
		}
	}

	fn update(self,) -> Self {
		match self {
			RedHatBoyStateMachine::Idle(mut state,) => {
				state.update();
				RedHatBoyStateMachine::Idle(state,)
			},
			RedHatBoyStateMachine::Running(mut state,) => {
				state.update();
				RedHatBoyStateMachine::Running(state,)
			},
		}
	}
}

pub enum Event {
	Run,
}

mod red_hat_boy_states {
	use crate::engine::Point;
	const FLOOR: i16 = 475;
	const IDLE_FRAME_NAME: &str = "Idle";
	const RUN_FRAME_NAME: &str = "Run";
	const IDLE_FRAMES: u8 = 29;
	const RUNNING_FRAMES: u8 = 23;
	const RUNNING_SPEED: i16 = 3;

	#[derive(Copy, Clone,)]
	pub struct RedHatBoyState<S,> {
		context: RedHatBoyContext,
		_state:  S,
	}
	impl<S,> RedHatBoyState<S,> {
		pub fn context(&self,) -> &RedHatBoyContext { &self.context }
	}

	impl RedHatBoyState<Idle,> {
		pub fn new() -> Self {
			RedHatBoyState {
				context: RedHatBoyContext {
					frame:    0,
					position: Point { x: 0, y: FLOOR, },
					velocity: Point { x: 0, y: 0, },
				},
				_state:  Idle {},
			}
		}

		pub fn run(self,) -> RedHatBoyState<Running,> {
			RedHatBoyState {
				context: self.context.reset_frame().run_right(),
				_state:  Running {},
			}
		}

		pub fn frame_name(&self,) -> &str { IDLE_FRAME_NAME }

		pub fn update(&mut self,) {
			//			crate::log!(
			//				"frame: {}, pos: ({}, {}), vel: ({}, {})",
			//				self.context.frame,
			//				self.context.position.x,
			//				self.context.position.y,
			//				self.context.velocity.x,
			//				self.context.velocity.y
			//			);
			self.context = self.context.update(IDLE_FRAMES,);
		}
	}

	impl RedHatBoyState<Running,> {
		pub fn frame_name(&self,) -> &str { RUN_FRAME_NAME }

		pub fn update(&mut self,) {
			self.context = self.context.update(RUNNING_FRAMES,);
		}
	}

	#[derive(Clone, Copy,)]
	pub struct RedHatBoyContext {
		pub frame:    u8,
		pub position: Point,
		pub velocity: Point,
	}

	impl RedHatBoyContext {
		pub fn update(mut self, frame_count: u8,) -> Self {
			if self.frame < frame_count {
				self.frame += 1;
			} else {
				self.frame = 0;
			}

			self.position.x += self.velocity.x;
			self.position.y += self.velocity.y;
			self
		}

		fn reset_frame(mut self,) -> Self {
			self.frame = 0;
			self
		}

		fn run_right(mut self,) -> Self {
			self.velocity.x += RUNNING_SPEED;
			self
		}
	}

	#[derive(Clone, Copy,)]
	pub struct Idle;
	#[derive(Clone, Copy,)]
	pub struct Running;
}
