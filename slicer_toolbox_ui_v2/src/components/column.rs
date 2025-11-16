use dioxus::prelude::*;

#[component]
pub fn Column(#[props(extends = GlobalAttributes)] attributes: Vec<Attribute>, children: Element,) -> Element {
	rsx! {
		div {
			display: "flex",
			flex_direction: "column",
			gap: ".5rem",
			..attributes,
			{children}
		}
	}
}