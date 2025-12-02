use crate::Coord;
use csv::Writer;
use itertools::Itertools;
use std::fs::File;
use std::path::Path;
use crate::landmarks::Landmark;

pub fn write_statistics_to_csv(
	path: &Path,
	all_file_coords: &[(String, Vec<Coord>)],
) -> anyhow::Result<()> {
	let mut writer = Writer::from_writer(File::create(path)?);

	let landmarks = get_landmarks(all_file_coords);
	writer.write_field("Samples")?;
	for landmark in &landmarks {
		writer.write_field(format!("{}__A", landmark))?;
		writer.write_field(format!("{}__S", landmark))?;
	}

	new_line(&mut writer)?;

	for (name, coord_per_land) in all_file_coords {
		writer.write_field(name)?;
		for landmark in &landmarks {
			match coord_per_land.iter().find(|x| x.landmark == *landmark) {
				None => {
					write_filler_lines(&mut writer, 2)?;
				}
				Some(coord) => {
					writer.write_field(coord.a.to_string())?;
					writer.write_field(coord.s.to_string())?;
				}
			}
		}
		new_line(&mut writer)?;
	}

	writer.flush()?;
	Ok(())
}

pub fn write_data_to_csv(
	path: &Path,
	all_file_coords: &[(String, Vec<Coord>)],
) -> anyhow::Result<()> {
	let mut writer = Writer::from_writer(File::create(path)?);

	for (name, _) in all_file_coords {
		writer.write_field(name)?;
		writer.write_field("R")?;
		writer.write_field("A")?;
		writer.write_field("S")?;
	}
	new_line(&mut writer)?;

	for landmark in get_landmarks(all_file_coords) {
		for (_, coord_per_land) in all_file_coords {
			writer.write_field(&landmark.as_str())?;
			match coord_per_land.iter().find(|x| x.landmark == landmark) {
				None => {
					write_filler_lines(&mut writer, 3)?;
				}
				Some(coord) => {
					writer.write_field(coord.r.to_string())?;
					writer.write_field(coord.a.to_string())?;
					writer.write_field(coord.s.to_string())?;
				}
			}
		}

		new_line(&mut writer)?;
	}

	writer.flush()?;
	Ok(())
}

fn get_landmarks(all_file_coords: &[(String, Vec<Coord>)]) -> Vec<Landmark> {
	all_file_coords
		.iter()
		.flat_map(|(_, data)| data.iter().map(|x| &x.landmark).cloned())
		.dedup()
		.collect_vec()
}

fn write_filler_lines(writer: &mut Writer<File>, count: usize) -> anyhow::Result<()> {
	for _ in 0..count {
		writer.write_field("")?;
	}
	Ok(())
}

fn new_line(writer: &mut Writer<File>) -> anyhow::Result<()> {
	writer.write_record(None::<&[u8]>)?;
	Ok(())
}
