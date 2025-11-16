use dioxus::prelude::*;

#[component]
pub fn Row(children: Element, #[props(default)] attributes: Vec<Attribute>) -> Element {
	rsx! {
		div {
			display: "flex",
			flex_direction: "row",
			flex_grow: "1",
			gap: ".5rem",
			..attributes,
			{children}
		}
	}
}