const FRAME_SIZE: f32 = 1.0 / 60.0 * 1000.0;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Mutex;

use super::browser;
use super::browser::LoopClosure;
use anyhow::Result;
use anyhow::anyhow;
use futures::channel::mpsc::UnboundedReceiver;
use futures::channel::oneshot;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;
use web_sys::HtmlImageElement;

pub trait Game {
	async fn init(&mut self,) -> Result<(),>;
	fn update(&mut self, keystat: &KeyState,);
	fn draw(&self, rndr: &Renderer,);
}

pub struct GameLoop {
	last_frame:        f64,
	accumulated_delta: f32,
}

pub struct Renderer {
	context: CanvasRenderingContext2d,
}

impl Renderer {
	pub fn clear(&self, rct: &Rect,) {
		self.context.clear_rect(rct.x.into(), rct.y.into(), rct.w.into(), rct.h.into(),);
	}

	pub fn draw_image(&self, img: &HtmlImageElement, frame: &Rect, dst: &Rect,) {
		self.context
			.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
				&img,
				frame.x.into(),
				frame.y.into(),
				frame.w.into(),
				frame.h.into(),
				dst.x.into(),
				dst.y.into(),
				dst.w.into(),
				dst.h.into(),
			)
			.expect("Drawing is throwing exceptions! Unrecoverable error",);
	}
}

pub struct Rect {
	pub x: f32,
	pub y: f32,
	pub w: f32,
	pub h: f32,
}

enum KeyPress {
	KeyDown(web_sys::KeyboardEvent,),
	KeyUp(web_sys::KeyboardEvent,),
}

pub struct KeyState {
	pressed: HashMap<String, web_sys::KeyboardEvent,>,
}

impl KeyState {
	fn new() -> Self {
		KeyState { pressed: HashMap::new(), }
	}

	pub fn is_pressed(&self, code: &str,) -> bool {
		self.pressed.iter().for_each(|(_key, _val,)| {
			//log!("key:----------------------------"); log!("{key}");
		},);
		self.pressed.contains_key(code,)
	}

	fn set_pressed(&mut self, code: &str, evt: web_sys::KeyboardEvent,) {
		self.pressed.insert(code.into(), evt,);
	}

	fn set_released(&mut self, code: &str,) {
		self.pressed.remove(code,);
	}
}

type SharedLoopClosure = Rc<RefCell<Option<LoopClosure,>,>,>;

impl GameLoop {
	pub async fn start(mut game: impl Game + 'static,) -> Result<(),> {
		let mut keyevent_rx = prepare_input()?;
		let _ = game.init().await;
		let mut game_loop = GameLoop { last_frame: browser::now()?, accumulated_delta: 0.0, };
		let rndrr = Renderer { context: browser::context()?, };

		let f: SharedLoopClosure = Rc::new(RefCell::new(None,),);
		let g = f.clone();

		let mut keystat = KeyState::new();
		*g.as_ref().borrow_mut() = Some(browser::create_raf_closure(move |perf| {
			process_input(&mut keystat, &mut keyevent_rx,);
			game_loop.accumulated_delta += (perf - game_loop.last_frame) as f32;
			while game_loop.accumulated_delta > FRAME_SIZE {
				game.update(&keystat,);
				game_loop.accumulated_delta -= FRAME_SIZE;
			}
			game_loop.last_frame = perf;
			game.draw(&rndrr,);

			let _a = browser::req_anim_frame(&f.as_ref().borrow().as_ref().unwrap(),);
		},),);

		browser::req_anim_frame(
			g.as_ref().borrow_mut().as_ref().expect("GameLoop: Loop is None",),
		)?;
		Ok((),)
	}
}

#[derive(Clone, Copy,)]
pub struct Point {
	pub x: i16,
	pub y: i16,
}

pub async fn load_img(src: &str,) -> Result<HtmlImageElement,> {
	let img = browser::new_img()?;
	let (complete_tx, complete_rx,) = oneshot::channel::<Result<(),>,>();
	let success_tx = Rc::new(Mutex::new(Some(complete_tx,),),);
	let error_tx = Rc::clone(&success_tx,);

	let success_callback = browser::closure_once(move || {
		if let Some(success_tx,) = success_tx.lock().ok().and_then(|mut opt| opt.take(),) {
			let _ = success_tx.send(Ok((),),);
		}
	},);
	let error_callback = browser::closure_once(move |e: JsValue| {
		if let Some(error_tx,) = error_tx.lock().ok().and_then(|mut opt| opt.take(),) {
			let _ = error_tx.send(Err(anyhow!("Error Loading Image: {:#?}", e),),);
		}
	},);
	img.set_onload(Some(success_callback.as_ref().unchecked_ref(),),);
	img.set_onerror(Some(error_callback.as_ref().unchecked_ref(),),);
	img.set_src(src,);

	complete_rx.await??;

	Ok(img,)
}

fn prepare_input() -> Result<futures::channel::mpsc::UnboundedReceiver<KeyPress,>,> {
	let (keyevent_tx, keyevent_rx,) = futures::channel::mpsc::unbounded();

	// TODO: `Mutex` doesnt required?
	let keydown_tx = Rc::new(RefCell::new(keyevent_tx,),);
	let keyup_tx = keydown_tx.clone();

	let onkey_down = browser::closure_wrap(Box::new(move |keycode: web_sys::KeyboardEvent| {
		let _ = keydown_tx.borrow_mut().start_send(KeyPress::KeyDown(keycode,),);
	},) as Box<dyn FnMut(web_sys::KeyboardEvent,),>,);
	let onkey_up = browser::closure_wrap(Box::new(move |keycode: web_sys::KeyboardEvent| {
		let _ = keyup_tx.borrow_mut().start_send(KeyPress::KeyUp(keycode,),);
	},) as Box<dyn FnMut(web_sys::KeyboardEvent,),>,);

	browser::window()?.set_onkeydown(Some(onkey_down.as_ref().unchecked_ref(),),);
	browser::window()?.set_onkeyup(Some(onkey_up.as_ref().unchecked_ref(),),);
	onkey_down.forget();
	onkey_up.forget();
	Ok(keyevent_rx,)
}

fn process_input(stat: &mut KeyState, keyevent_rx: &mut UnboundedReceiver<KeyPress,>,) {
	loop {
		match keyevent_rx.try_next() {
			Ok(None,) | Err(_,) => break,
			Ok(Some(evt,),) => match evt {
				KeyPress::KeyUp(evt,) => stat.set_released(&evt.code(),),
				KeyPress::KeyDown(evt,) => stat.set_pressed(&evt.code(), evt,),
			},
		}
	}
}
