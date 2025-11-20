use dioxus::prelude::*;

#[component]
pub fn Stack(
	#[props(default)] row: bool,
	children: Element,
	#[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
	rsx! {
		div {
			display: "flex",
			flex_direction: if row { "row" } else { "column" },
			gap: ".5rem",
			..attributes,
			{children}
		}
	}
}
