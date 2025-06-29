use anyhow::{anyhow, Context, Result};
use icu_collator::options::{AlternateHandling, CollatorOptions};
use icu_collator::preferences::CollationNumericOrdering;
use icu_collator::{Collator, CollatorPreferences};
use itertools::Itertools;
use merge_whitespace_utils::merge_whitespace;
use rc_zip_sync::ReadZip;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::ops::Neg;
use std::path::PathBuf;
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

pub struct Coord {
    pub r: f64,
    pub a: f64,
    pub s: f64,
}

pub fn parse_from_slicer_data(path: &PathBuf) -> Result<Vec<(String, HashMap<String, Coord>)>> {
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

        let mut all_coords = HashMap::new();
        for file in file
            .read_zip()?
            .entries()
            .filter(|entry| entry.name.ends_with(".mrk.json"))
        {
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

                    let label = merge_whitespace(control_point.label.trim()).to_string();

                    all_coords.insert(label, Coord { r, a, s });
                }
            }
        }

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
