pub mod aggregations;
pub mod export;
pub mod landmarks;
mod slicer_data;

use crate::landmarks::Landmark;
use crate::slicer_data::SlicerMarkup;
use anyhow::{Context, Result, anyhow};
use icu_collator::options::{AlternateHandling, CollatorOptions};
use icu_collator::preferences::CollationNumericOrdering;
use icu_collator::{Collator, CollatorPreferences};
use itertools::Itertools;
use rc_zip_sync::ReadZip;
use regex::Regex;
use statrs::statistics::Statistics;
use std::fs::File;
use std::ops::Neg;
use std::path::PathBuf;
use std::str::FromStr;
use walkdir::WalkDir;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct MarkedPoint {
	pub landmark: Landmark,
	pub r: f64,
	pub a: f64,
	pub s: f64,
}

impl MarkedPoint {
	pub fn mean(&self) -> f64 {
		[self.r, self.a, self.s].mean()
	}
}

#[derive(Debug, Clone)]
pub struct Subject {
	pub name: String,
	pub landmarks: Vec<MarkedPoint>,
}

impl PartialEq for Subject {
	fn eq(&self, other: &Self) -> bool {
		other.name == self.name
	}
}

#[derive(Debug, Clone)]
pub struct ReviewerData {
	pub name: String,
	pub subjects: Vec<Subject>,
}

pub fn parse_from_slicer_data(path: &PathBuf) -> Result<Vec<Subject>> {
	let end_name = Regex::new(r"-\d*$")?;

	let mut all_file_coords = Vec::new();
	for entry_result in WalkDir::new(path).max_depth(1).into_iter().filter(|x| {
		x.as_ref()
			.is_ok_and(|x1| x1.file_name().to_str().is_some_and(|t| t.ends_with(".mrb")))
	}) {
		let entry = entry_result?;
		let file_name = entry
			.file_name()
			.to_str()
			.context("Failed to read the name of the file")?
			.to_string();

		let file = File::open(entry.path()).context(format!(
			"Could not open file: {}",
			entry.file_name().display()
		))?;

		let mut errors = Vec::new();
		let mut landmarks = Vec::new();
		for file in file
			.read_zip()?
			.entries()
			.filter(|entry| entry.name.ends_with(".mrk.json"))
		{
			let landmark_file_name = file
				.name
				.rsplit_once("/")
				.into_iter()
				.next_back()
				.and_then(|(_, file)| file.split_once(".").map(|(file_name, _)| file_name))
				.context("Could not split file name")?;

			let slicer_markup = serde_json::from_reader::<_, SlicerMarkup>(file.reader())?;
			for markup in slicer_markup.markups {
				let coords = markup.coordinate_system;
				if coords.len() != 3 {
					return Err(anyhow!("Invalid coordinate system. Should be 3 characters"));
				}

				let chars = coords.chars().collect_vec();
				for control_point in markup.control_points {
					let r = convert_to_ras(&chars, &'r', &'l', control_point.position)?;
					let a = convert_to_ras(&chars, &'a', &'p', control_point.position)?;
					let s = convert_to_ras(&chars, &'s', &'i', control_point.position)?;

					let label_name = end_name.replace(&control_point.label, "");

					let Ok(landmark) =
						Landmark::from_str(landmark_file_name).or(Landmark::from_str(&label_name))
					else {
						errors.push(anyhow!(
							"Could not find landmark: {} or {} in {} part of {}",
							label_name,
							landmark_file_name,
							file_name,
							file.name
						));
						continue;
					};

					landmarks.push(MarkedPoint { landmark, r, a, s });
				}
			}
		}

		if !errors.is_empty() {
			return Err(anyhow!(
				"Errors while parsing:\n{}",
				errors.into_iter().join("\n")
			));
		}
		landmarks.sort_by(|lhs, rhs| lhs.landmark.cmp(&rhs.landmark));

		all_file_coords.push(Subject {
			name: file_name,
			landmarks,
		});
	}

	let mut options = CollatorOptions::default();
	let mut pref = CollatorPreferences::default();
	pref.numeric_ordering = Some(CollationNumericOrdering::True);
	options.alternate_handling = Some(AlternateHandling::Shifted);
	let collator = Collator::try_new(pref, options).map_err(|e| anyhow!(e))?;
	all_file_coords.sort_by(|lhs, rhs| collator.compare(&lhs.name, &rhs.name));

	Ok(all_file_coords)
}

fn convert_to_ras(
	actual: &[char],
	positive: &char,
	negative: &char,
	positions: [f64; 3],
) -> Result<f64> {
	actual
		.iter()
		.find_position(|char| {
			char.eq_ignore_ascii_case(positive) || char.eq_ignore_ascii_case(negative)
		})
		.and_then(|(pos, c)| {
			positions
				.get(pos)
				.map(|pos| if c.eq(negative) { pos.neg() } else { *pos })
		})
		.context(anyhow!(
			"Could not find either {} or {} in coordinates",
			positive,
			negative
		))
}
