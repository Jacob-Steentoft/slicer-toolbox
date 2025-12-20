use crate::landmarks::Landmark;
use crate::Subject;
use csv::Writer;
use std::fs::File;
use std::path::Path;

pub fn write_statistics_to_csv(path: &Path, subjects: &[Subject]) -> anyhow::Result<()> {
	let mut writer = Writer::from_writer(File::create(path)?);

	let landmarks = Landmark::all_variants();
	writer.write_field("Samples")?;
	for landmark in landmarks {
		writer.write_field(format!("{landmark}__R"))?;
		writer.write_field(format!("{landmark}__A"))?;
		writer.write_field(format!("{landmark}__S"))?;
	}

	new_line(&mut writer)?;

	for subject in subjects {
		writer.write_field(&subject.name)?;

		for landmark in landmarks {
			match subject
				.landmarks
				.iter()
				.find(|coord| coord.landmark == *landmark)
			{
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
