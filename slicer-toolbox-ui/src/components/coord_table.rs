use dioxus::core_macro::component;
use dioxus::prelude::*;
use slicer_toolbox_core::Coord;

#[component]
pub fn CoordTable(#[props(default)] data: ReadSignal<Vec<Coord>>) -> Element {
	rsx! {
		div { class: "overflow-x-auto h-82",
			table { class: "table table-zebra table-pin-cols",
				thead {
					tr {
						td { "Name" }
						td { "A" }
						td { "S" }
						td { "R" }
					}
				}
				tbody {
					for Coord { landmark , a , s , r } in data.read().iter() {
						tr {
							td { {landmark.as_str()} }
							td { {format!("{:.2}", a)} }
							td { {format!("{:.2}", s)} }
							td { {format!("{:.2}", r)} }
						}
					}
				}
			}
		}
	}
}
