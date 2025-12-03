use dioxus::core_macro::component;
use dioxus::prelude::*;
use slicer_toolbox_core::Coord;
use slicer_toolbox_core::landmarks::Landmark;

#[component]
pub fn CoordTable(#[props(default)] data: ReadSignal<Vec<Coord>>) -> Element {
	rsx! {
		div { class: "overflow-x-auto h-94",
			table { class: "table table-zebra table-pin-cols",
				thead {
					tr {
						td { "Name" }
						td { "R" }
						td { "A" }
						td { "S" }
					}
				}
				tbody {
					for (index , row) in Landmark::all_variants()
					    .iter()
					    .map(|landmark| {
					        data.read().iter().find(|coord| coord.landmark == *landmark).copied()
					    })
					    .enumerate()
					{

						if let Some(Coord { landmark, a, s, r }) = row {
							tr {
								td { {landmark.as_str()} }
								td { {format!("{:.2}", r)} }
								td { {format!("{:.2}", a)} }
								td { {format!("{:.2}", s)} }
							}
						} else {
							tr {
								td { {Landmark::all_variants()[index].as_str()} }
								td { "" }
								td { "" }
								td { "" }
							}
						}
					}
				}
			}
		}
	}
}
