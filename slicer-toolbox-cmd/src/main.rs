use anyhow::{Result, anyhow};
use rfd::FileDialog;
use slicer_toolbox_core::export::write_statistics_to_csv;
use slicer_toolbox_core::parse_from_slicer_data;

fn main() -> Result<()> {
	let Some(path) = FileDialog::new()
		.set_title("Select folder to import from")
		.pick_folder()
	else {
		return Err(anyhow!("No directory selected"));
	};

	let all_file_coords = parse_from_slicer_data(&path)?;

	// Data creation
	write_statistics_to_csv(&path, &all_file_coords)?;

	dont_disappear::any_key_to_continue::default();
	Ok(())
}
