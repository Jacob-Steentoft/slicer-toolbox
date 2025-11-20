use crate::components::button::{Button, ButtonVariant};
use crate::components::input::Input;
use crate::components::label::Label;
use crate::components::separator::Separator;
use crate::components::stack::Stack;
use dioxus::prelude::*;
use rfd::FileDialog;
use slicer_toolbox_core::Coord;
use std::collections::HashMap;
use std::path::PathBuf;

mod components;

const COMPONENTS: Asset = asset!("/assets/dx-components-theme.css");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
	launch(App);
}

#[component]
fn App() -> Element {
	let mut path = use_signal(String::new);

	rsx! {
		Stylesheet { href: MAIN_CSS }
		Stylesheet { href: TAILWIND_CSS }
		Stylesheet { href: COMPONENTS }
		Stack { class: "p-4", style: "width: 100%",
			Label { html_for: "file_path", "Search path" }
			Input {
				value: path,
				placeholder: "Choose Folder",
				readonly: true,
				onclick: move |_| *path.write() = get_folder_path().unwrap_or(path.to_string()),
			}
			Separator { horizontal: true, decorative: true }
			Button {
				variant: ButtonVariant::Outline,
				disabled: !is_valid_folder(&path.read()),
				"Import"
			}
		}
	}
}

fn get_folder_path() -> Option<String> {
	FileDialog::new()
		.set_title("Select folder to import from")
		.pick_folder()
		.map(|path| path.to_str().unwrap_or_default().to_string())
}

fn get_data(path: String) -> Vec<(String, HashMap<String, Coord>)> {
	slicer_toolbox_core::parse_from_slicer_data(&PathBuf::from(path)).unwrap()
}

fn is_valid_folder(path: &str) -> bool {
	let buf = PathBuf::from(path);
	buf.exists() && buf.is_dir()
}
