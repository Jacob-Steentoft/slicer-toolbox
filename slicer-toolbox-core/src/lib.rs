pub mod csv;
mod landmarks;

use crate::landmarks::{LANDMARK_HAYSTACK, Landmark};
use anyhow::{Context, Result, anyhow};
use fuse_rust::Fuse;
use icu_collator::options::{AlternateHandling, CollatorOptions};
use icu_collator::preferences::CollationNumericOrdering;
use icu_collator::{Collator, CollatorPreferences};
use itertools::Itertools;
use norm::fzf::{FzfParser, FzfV1};
use norm::{CaseSensitivity, Metric};
use rc_zip_sync::ReadZip;
use regex::Regex;
use serde::Deserialize;
use std::fs::File;
use std::ops::Neg;
use std::path::PathBuf;
use std::str::FromStr;
use walkdir::WalkDir;

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

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Coord {
	pub landmark: Landmark,
	pub r: f64,
	pub a: f64,
	pub s: f64,
}

pub fn parse_from_slicer_data(path: &PathBuf) -> Result<Vec<(String, Vec<Coord>)>> {
	let fuse = Fuse::default();
	let regex = Regex::new(r"-\d*$")?;
	let filename_regex = Regex::new(r".*/(\w*)\.")?;

	let mut all_file_coords = Vec::new();
	for entry_result in WalkDir::new(path).into_iter().filter(|x| {
		x.as_ref()
			.is_ok_and(|x1| x1.file_name().to_str().is_some_and(|t| t.ends_with(".mrb")))
	}) {
		let entry = entry_result?;
		let file_name = entry
			.file_name()
			.to_str()
			.context("Failed to read the name of the file")?
			.to_string();

		let file =
			File::open(entry.path()).context(format!("Could not open file: {}", file_name))?;

		let mut all_coords = Vec::new();
		for file in file
			.read_zip()?
			.entries()
			.filter(|entry| entry.name.ends_with(".mrk.json"))
		{
			let (landmark_file_name, _) = file
				.name
				.split_once(".")
				.context("Could not split file name")?;
			let (_, landmark_file_name) = landmark_file_name.rsplit_once("/").into_iter().next_back().context("Could not split file name")?;
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

					let trimmed_landmark = regex.replace(&control_point.label, "");

					let mut results = fuse.search_text_in_iterable(landmark_file_name, LANDMARK_HAYSTACK);
					let vec = fuse.search_text_in_iterable(&trimmed_landmark, LANDMARK_HAYSTACK);
					results.extend(vec);

					let result = results
						.iter()
						.min_by(|result1, result2| {
							result1.score.partial_cmp(&result2.score).unwrap()
						}).context(anyhow!(
							"Could not find landmark: {} in {} part of {}",
							trimmed_landmark,
							file_name,
							file.name
						))?;

					all_coords.push(Coord {
						landmark: Landmark::from_str(LANDMARK_HAYSTACK[result.index])?,
						r,
						a,
						s,
					});
				}
			}
		}
		all_coords.sort_by(|a, b| a.landmark.cmp(&b.landmark));

		all_file_coords.push((file_name, all_coords));
	}

	let mut options = CollatorOptions::default();
	let mut pref = CollatorPreferences::default();
	pref.numeric_ordering = Some(CollationNumericOrdering::True);
	options.alternate_handling = Some(AlternateHandling::Shifted);
	let collator = Collator::try_new(pref, options).map_err(|e| anyhow!(e))?;

	all_file_coords.sort_by(|(lhs, _), (rhs, _)| collator.compare(lhs, rhs));

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
