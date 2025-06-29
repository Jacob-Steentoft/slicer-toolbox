use anyhow::{Result, anyhow};
use csv::Writer;
use itertools::Itertools;
use rfd::FileDialog;
use slicer_toolbox_core::{Coord, parse_from_slicer_data};
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

fn main() -> Result<()> {
    let Some(path) = FileDialog::new()
        .set_title("Select folder to import from")
        .pick_folder()
    else {
        return Err(anyhow!("No directory selected"));
    };

    let all_file_coords = parse_from_slicer_data(&path)?;

	// Get unique landmarks
	let landmarks = all_file_coords
		.iter()
		.flat_map(|(_, data)| data.keys().cloned())
		.dedup()
		.collect_vec();

    // Data creation
    create_main_data(&path, &all_file_coords, &landmarks)?;
    create_statistics(&path, &all_file_coords, &landmarks)?;

    dont_disappear::any_key_to_continue::default();
    Ok(())
}

fn create_statistics(
    path: &Path,
    all_file_coords: &[(String, HashMap<String, Coord>)],
    landmarks: &[String],
) -> Result<()> {
    let path = path.join("statistics.csv");
    let mut writer = Writer::from_writer(File::create(&path)?);

    writer.write_field("Samples")?;
    for landmark in landmarks {
        writer.write_field(format!("{}__A", landmark))?;
        writer.write_field(format!("{}__S", landmark))?;
    }

    new_line(&mut writer)?;

    for (name, coord_per_land) in all_file_coords {
        writer.write_field(name)?;
        for landmark in landmarks {
            match coord_per_land.get(landmark) {
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
    println!("Created statistics file at: {}", path.to_str().unwrap());
    Ok(())
}

fn create_main_data(
    path: &Path,
    all_file_coords: &[(String, HashMap<String, Coord>)],
    landmarks: &[String],
) -> Result<()> {
    let path = path.join("main data.csv");
    let mut writer = Writer::from_writer(File::create(&path)?);

    for (name, _) in all_file_coords {
        writer.write_field(name)?;
        writer.write_field("R")?;
        writer.write_field("A")?;
        writer.write_field("S")?;
    }
    new_line(&mut writer)?;

    for landmark in landmarks {
        for (_, coord_per_land) in all_file_coords {
            writer.write_field(landmark)?;
            match coord_per_land.get(landmark) {
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
    println!("Created main data file at: {}", path.to_str().unwrap());
    Ok(())
}

fn write_filler_lines(writer: &mut Writer<File>, count: usize) -> Result<()> {
    for _ in 0..count {
        writer.write_field("")?;
    }
    Ok(())
}

fn new_line(writer: &mut Writer<File>) -> Result<()> {
    writer.write_record(None::<&[u8]>)?;
    Ok(())
}
