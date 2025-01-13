use anyhow::Result;
use anyhow::anyhow;
use futures::Future;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::closure::WasmClosure;
use wasm_bindgen::closure::WasmClosureFnOnce;
use wasm_bindgen_futures::JsFuture;
use web_sys::CanvasRenderingContext2d;
use web_sys::Document;
use web_sys::HtmlCanvasElement;
use web_sys::HtmlImageElement;
use web_sys::Response;
use web_sys::Window;

pub type LoopClosure = Closure<dyn FnMut(f64,),>;

#[macro_export]
macro_rules! log {
    ($($t:tt)*) => {
        web_sys::console::log_1(&format!($($t)*).into());
    };
}

pub fn window() -> Result<Window,> {
	web_sys::window().ok_or_else(|| anyhow!("No window found"),)
}

pub fn document() -> Result<Document,> {
	window()?.document().ok_or_else(|| anyhow!("No document found"),)
}

pub fn canvas() -> Result<HtmlCanvasElement,> {
	document()?
		.get_element_by_id("canvas",)
		.ok_or_else(|| anyhow!("No Canvas Element found with ID 'canvas'"),)?
		.dyn_into::<HtmlCanvasElement>()
		.map_err(|elem| anyhow!("Error converting {:#?} to HtmlCanvasElement", elem),)
}

pub fn context() -> Result<CanvasRenderingContext2d,> {
	canvas()?
		.get_context("2d",)
		.map_err(|js_val| anyhow!("Error getting 2d context {:#?}", js_val),)?
		.ok_or_else(|| anyhow!("No 2d context found"),)?
		.dyn_into::<CanvasRenderingContext2d>()
		.map_err(|elem| anyhow!("Error converting {:#?} to CanvasRenderingContext2d", elem),)
}

pub fn spawn_local<F: Future<Output = (),> + 'static,>(future: F,) {
	wasm_bindgen_futures::spawn_local(future,);
}

pub async fn fetch_with_str(resource: &str,) -> Result<JsValue,> {
	JsFuture::from(window()?.fetch_with_str(resource,),)
		.await
		.map_err(|e| anyhow!("error fetching {:#?}", e),)
}

pub async fn fetch_json(json_path: &str,) -> Result<JsValue,> {
	let rsp_val = fetch_with_str(json_path,).await?;
	let rsp: Response =
		rsp_val.dyn_into().map_err(|elem| anyhow!("Error converting {:#?} to Response", elem),)?;
	JsFuture::from(rsp.json().map_err(|e| anyhow!("Could not get JSON from response {:#?}", e),)?,)
		.await
		.map_err(|e| anyhow!("error fetching JSON {:#?}", e),)
}

pub fn new_img() -> Result<HtmlImageElement,> {
	HtmlImageElement::new().map_err(|e| anyhow!("Could not create HtmlImageElement: {:#?}", e),)
}

pub fn closure_once<F, A, R,>(fn_once: F,) -> Closure<F::FnMut,>
where F: 'static + WasmClosureFnOnce<A, R,> {
	Closure::once(fn_once,)
}

pub fn closure_wrap<T: WasmClosure + ?Sized,>(data: Box<T,>,) -> Closure<T,> {
	Closure::wrap(data,)
}

/// `LoopClosure = wasm_bindgen::closure::Closure<dyn  FnMut(f64)>`
pub fn req_anim_frame(cb: &LoopClosure,) -> Result<i32,> {
	window()?
		.request_animation_frame(cb.as_ref().unchecked_ref(),)
		.map_err(|e| anyhow!("Cannot request animation frame {:#?}", e),)
}

pub fn create_raf_closure(f: impl FnMut(f64,) + 'static,) -> LoopClosure {
	closure_wrap(Box::new(f,),)
}

pub fn now() -> Result<f64,> {
	Ok(window()?.performance().ok_or_else(|| anyhow!("Performance object not found"),)?.now(),)
}
