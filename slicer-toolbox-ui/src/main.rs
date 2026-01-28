use crate::components::coord_table::CoordTable;
use crate::components::stack::Stack;
use anyhow::Result;
use dioxus::prelude::*;
use rfd::FileDialog;
use slicer_toolbox_core::Subject;
use slicer_toolbox_core::export::write_statistics_to_csv;
use std::path::PathBuf;

mod components;

const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
	launch(App);
}

#[component]
fn App() -> Element {
	let mut path = use_signal(String::new);
	let mut subjects = use_store(Vec::<Subject>::new);
	let mut selected_subject = use_signal(String::new);

	rsx! {
		Stylesheet { href: MAIN_CSS }
		Stylesheet { href: TAILWIND_CSS }

		Stack { class: "m-4 h-full",
			input {
				id: "file_path",
				class: "input w-full",
				r#type: "text",
				value: path,
				placeholder: "Import from folder",
				readonly: true,
				onclick: move |_| {
					*path.write() = get_folder_path().unwrap_or(path.to_string());
					let tableData = get_data(&path.read())?;
					*subjects.write() = tableData;
					*selected_subject.write() = subjects().iter().map(|x| x.name.clone()).next().unwrap_or_default();
					Ok(())
				},
			}
			if !subjects.is_empty() {
				button {
					class: "btn btn-primary",
					onclick: move |_| {
						if let Some(save_path) = get_save_path() {
							write_statistics_to_csv(&save_path, &subjects())?;
						}
						Ok(())
					},
					"Export CSV"
				}
				ul { class: "menu menu-horizontal bg-base-200 rounded-box flex-nowrap overflow-x-scroll w-full",
					for subject in subjects() {
						ul {
							button {
								class: "btn btn-sm btn-ghost text-nowrap",
								onclick: move |_| selected_subject.set(subject.name.clone()),
								{subject.name.as_str().replace(".mrb", "")}
							}
						}
					}
				}
				if let Some(subject) = subjects.iter().find(|subject| subject.read().name == *selected_subject.read())
				{
					CoordTable { subject }
				}

			}
		}

	}
}

fn get_save_path() -> Option<PathBuf> {
	FileDialog::new()
		.set_title("Select CSV file to export to")
		.set_file_name("data.csv")
		.save_file()
}

fn get_folder_path() -> Option<String> {
	FileDialog::new()
		.set_title("Select folder to import from")
		.pick_folder()
		.map(|path| path.to_str().unwrap_or_default().to_string())
}

fn get_data(path: &str) -> Result<Vec<Subject>> {
	slicer_toolbox_core::parse_from_slicer_data(&PathBuf::from(path))
}
