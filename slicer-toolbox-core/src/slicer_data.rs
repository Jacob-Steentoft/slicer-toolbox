use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct SlicerMarkup {
	pub markups: Vec<Markups>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Markups {
	pub coordinate_system: String,
	pub control_points: Vec<ControlPoint>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct ControlPoint {
	pub label: String,
	pub position: [f64; 3],
}