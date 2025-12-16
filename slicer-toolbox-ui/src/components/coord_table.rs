use dioxus::core_macro::component;
use dioxus::prelude::*;
use slicer_toolbox_core::landmarks::Landmark;
use slicer_toolbox_core::{MarkedPoint, Subject};

#[component]
pub fn CoordTable(subject: ReadSignal<Subject>) -> Element {
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
							subject.read().landmarks.iter().find(|coord| coord.landmark == *landmark).copied()
						})
						.enumerate()
					{

						if let Some(MarkedPoint { landmark, a, s, r }) = row {
							tr {
								td { {landmark.to_string()} }
								td { {format!("{:.2}", r)} }
								td { {format!("{:.2}", a)} }
								td { {format!("{:.2}", s)} }
							}
						} else {
							tr {
								td { {Landmark::all_variants()[index].to_string()} }
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
