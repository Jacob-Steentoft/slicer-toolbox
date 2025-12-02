use crate::components::coord_table::CoordTable;
use crate::components::stack::Stack;
use dioxus::prelude::*;
use rfd::FileDialog;
use slicer_toolbox_core::Coord;
use slicer_toolbox_core::csv::write_data_to_csv;
use std::collections::HashMap;
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
	let mut data = use_signal(Vec::<(String, Vec<Coord>)>::new);
	let mut selected = use_signal(String::new);

	rsx! {
		Stylesheet { href: MAIN_CSS }
		Stylesheet { href: TAILWIND_CSS }

		Stack { class: "m-4 h-full",
			Stack { row: true,
				input {
					id: "file_path",
					class: "input w-full",
					r#type: "text",
					value: path,
					placeholder: "Choose Folder",
					readonly: true,
					onclick: move |_| *path.write() = get_folder_path().unwrap_or(path.to_string()),
				}
			}
			button {
				class: "btn btn-primary w-full",
				disabled: !is_valid_folder(&path.read()),
				onclick: move |_| {
				    let tableData = get_data(&path.read());
				    *selected.write() = tableData.first().map(|i| i.0.clone()).unwrap_or_default();
				    *data.write() = tableData;
				},
				"Import"
			}
			if !data.is_empty() {
				button {
					class: "btn btn-primary",
					onclick: move |_| {
					    if let Some(save_path) = get_save_path() {
					        write_data_to_csv(&save_path, &data.read()).context("Failed to write")?;
					    }
					    Ok(())
					},
					"Export"
				}
				ul { class: "menu menu-horizontal bg-base-200 rounded-box flex-nowrap overflow-x-scroll w-full",
					for (file_name , _) in data() {
						ul {
							button {
								class: "btn btn-sm btn-ghost text-nowrap",
								onclick: move |_| selected.set(file_name.clone()),
								{file_name.as_str().replace(".mrb", "")}
							}
						}
					}
				}
				if let Some((_, tableData)) = data
				    .read()
				    .iter()
				    .find(|(name, _)| *name == *selected.read())
				{
					CoordTable { data: tableData.clone() }
				}
			}
		}

	}
}

fn get_save_path() -> Option<PathBuf> {
	FileDialog::new()
		.set_title("Select CSV file to export to")
		.save_file()
}

fn get_folder_path() -> Option<String> {
	FileDialog::new()
		.set_title("Select folder to import from")
		.pick_folder()
		.map(|path| path.to_str().unwrap_or_default().to_string())
}

fn get_data(path: &str) -> Vec<(String, Vec<Coord>)> {
	slicer_toolbox_core::parse_from_slicer_data(&PathBuf::from(path)).unwrap()
}

fn is_valid_folder(path: &str) -> bool {
	let buf = PathBuf::from(path);
	buf.exists() && buf.is_dir()
}
