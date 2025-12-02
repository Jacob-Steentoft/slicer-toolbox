use dioxus::prelude::*;
use tailwind_fuse::tw_merge;

#[component]
pub fn Stack(
	#[props(default)] row: bool,
	children: Element,
	#[props(default, into)]
	class: String,
	#[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
	let direction = if row { "flex-row" } else { "flex-col" };
	let class = tw_merge!{ class, "flex gap-2", direction};
	rsx! {
		div { class, ..attributes, {children} }
	}
}
