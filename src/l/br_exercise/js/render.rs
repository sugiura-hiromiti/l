use super::dom::NodeType;
use super::layout::BoxProps;
use super::layout::BoxType;
use super::layout::LayoutBox;
use cursive::view::IntoBoxedView;
use cursive::view::View;
use cursive::view::ViewWrapper;
use cursive::views::DummyView;
use cursive::views::LinearLayout;
use cursive::views::Panel;
use cursive::views::TextView;

pub type ElementContainer = Box<dyn View>;

pub fn new_element_container() -> ElementContainer {
	(DummyView {}).into_boxed_view()
}

pub fn to_element_container(layout: LayoutBox<'_>) -> ElementContainer {
	match layout.box_type {
		BoxType::BlockBox(p) | BoxType::InlineBox(p) => match p {
			BoxProps { node_type: NodeType::Element(element), .. } => {
				let mut p = Panel::new(LinearLayout::vertical()).title(element.tag_name.clone());
				match element.tag_name.as_str() {
					_ => {
						for child in layout.children.into_iter() {
							p.with_view_mut(|v| v.add_child(to_element_container(child)));
						}
					},
				};

				p.into_boxed_view()
			},
			BoxProps { node_type: NodeType::Text(t), .. } => {
				// NOTE: This is puppy original behaviour, not a standard one.
				// For your information, CSS Text Module Level 3 specifies how to process
				// whitespaces. See https://www.w3.org/TR/css-text-3/#white-space-processing for further information.
				let text_to_display = t.data.clone();
				let text_to_display = text_to_display.replace("\n", "");
				let text_to_display = text_to_display.trim();
				if !text_to_display.is_empty() {
					TextView::new(text_to_display).into_boxed_view()
				} else {
					(DummyView {}).into_boxed_view()
				}
			},
		},
		BoxType::AnonymousBox => {
			let mut p = Panel::new(LinearLayout::horizontal());

			for child in layout.children.into_iter() {
				p.with_view_mut(|v| v.add_child(to_element_container(child)));
			}

			p.into_boxed_view()
		},
	}
}
