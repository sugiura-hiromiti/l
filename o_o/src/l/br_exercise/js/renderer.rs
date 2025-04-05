use super::css;
use super::dom::Node;
use super::dom::NodeType;
use super::javascript::JavaScriptRuntime;
use super::layout::to_layout_box;
use super::render::ElementContainer;
use super::render::to_element_container;
use super::style::to_styled_node;
use cursive::Vec2;
use cursive::direction::Direction;
use cursive::event::AnyCb;
use cursive::event::Event;
use cursive::event::EventResult;
use cursive::view::CannotFocus;
use cursive::view::Selector;
use cursive::view::View;
use cursive::view::ViewNotFound;
use std::sync::Arc;
use std::sync::Mutex;

const DEFAULT_STYLESHEET: &str = r#"
script,style{
    display: none;
}
p,div{
    display:block;
}
"#;

fn collect_tag_inners(node: &Box<Node>, tag_name: &str) -> Vec<String> {
	if let NodeType::Element(ref element) = node.node_type {
		if element.tag_name.as_str() == tag_name {
			return vec![node.inner_text()];
		}
	}
	node.children
		.iter()
		.map(|child| collect_tag_inners(child, tag_name))
		.collect::<Vec<Vec<String>>>()
		.into_iter()
		.flatten()
		.collect()
}

pub struct Renderer {
	view: ElementContainer,                  //actual Cursive view used to render
	document_element: Arc<Mutex<Box<Node>>>, //Original DOM tree of view
	js_runtime_instance: JavaScriptRuntime,  //JS executable instance
}

unsafe impl Send for Renderer {}
unsafe impl Sync for Renderer {}

//Use Renderer as Cursive's view
impl View for Renderer {
	fn draw(&self, printer: &cursive::Printer) {
		self.view.draw(printer)
	}

	fn layout(&mut self, v: Vec2) {
		self.view.layout(v)
	}

	fn needs_relayout(&self) -> bool {
		self.view.needs_relayout()
	}

	fn required_size(&mut self, constraint: Vec2) -> Vec2 {
		self.view.required_size(constraint)
	}

	fn on_event(&mut self, e: Event) -> EventResult {
		self.view.on_event(e)
	}

	fn call_on_any(&mut self, s: &Selector<'_>, cb: AnyCb<'_>) {
		self.view.call_on_any(s, cb)
	}

	fn focus_view(&mut self, s: &Selector<'_>) -> Result<EventResult, ViewNotFound> {
		self.view.focus_view(s)
	}

	fn take_focus(&mut self, source: Direction) -> Result<EventResult, CannotFocus> {
		self.view.take_focus(source)
	}
}

impl Renderer {
	pub fn new(document_element: Box<Node>) -> Self {
		let stylesheet = css::parse(&format!(
			"{DEFAULT_STYLESHEET}\n{}",
			collect_tag_inners(&document_element, "style").join("\n")
		));
		let view = to_styled_node(&document_element, &stylesheet)
			.map(|styled_node| to_layout_box(styled_node))
			.map(|layout_box| to_element_container(layout_box))
			.unwrap();
		let document_element = Arc::new(Mutex::new(document_element));
		let _document_element_ref = document_element.clone();
		Self {
			view,
			document_element,
			js_runtime_instance: JavaScriptRuntime::new(
							// document_element_ref,
							// Rc::new(RendererAPI::new(ui_cb_sink,),),
						),
		}
	}

	///Update view according to DOM tree (self.document_element)
	pub fn rerender(&mut self) {
		let document_element = self.document_element.lock().unwrap();
		let stylesheet = css::parse(&format!(
			"{DEFAULT_STYLESHEET}\n{}",
			collect_tag_inners(&document_element, "style").join("\n")
		));
		self.view = to_styled_node(&document_element, &stylesheet)
			.map(|styled_node| to_layout_box(styled_node))
			.map(|layout_box| to_element_container(layout_box))
			.unwrap();
	}

	///Execute JS contained in DOM tree (self.document_element)
	pub fn execute_inline_scripts(&mut self) {
		let scripts = {
			let document_element = self.document_element.lock().unwrap();
			collect_tag_inners(&document_element, "script").join("\n")
		};
		self.js_runtime_instance.execute("(inline)", scripts.as_str()).unwrap();
	}
}
