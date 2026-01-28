use itertools::Itertools;
use statrs::statistics::Statistics;

use crate::{ReviewerData, landmarks::Landmark};

#[derive(Debug, Clone)]
pub struct ReviewerAggregation {
	pub subject: String,
	pub landmark: Landmark,
	pub mean: f64,
	pub std_dev: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct LandmarkAggregation {
	pub landmark: Landmark,
	pub icc: f64,
}

pub fn calculate_subject_aggregations(reviewer_data: &[ReviewerData]) -> Vec<ReviewerAggregation> {
	let unique_subjects = reviewer_data
		.iter()
		.flat_map(|x| x.subjects.iter().map(|y| y.name.as_str()))
		.dedup()
		.collect_vec();
	let landmarks = Landmark::all_variants();
	let mut reviewer_aggregations = Vec::new();

	for unique_subject in unique_subjects {
		let subjects = reviewer_data
			.iter()
			.flat_map(|reviewer_data| {
				reviewer_data
					.subjects
					.iter()
					.filter(|subject| subject.name.eq(unique_subject))
			})
			.collect_vec();

		for &landmark in landmarks {
			let subject_landmarks = subjects
				.iter()
				.flat_map(|subject| {
					subject
						.landmarks
						.iter()
						.filter(|marked_point| marked_point.landmark == landmark)
				})
				.collect_vec();

			let map = subject_landmarks.into_iter().map(|p| p.mean());

			reviewer_aggregations.push(ReviewerAggregation {
				subject: unique_subject.to_string(),
				landmark,
				std_dev: map.clone().std_dev(),
				mean: map.mean(),
			})
		}
	}

	reviewer_aggregations
}

pub fn calculate_landmark_icc(reviewer_data: &[ReviewerData]) -> Vec<LandmarkAggregation> {
	let landmarks = Landmark::all_variants();
	let mut landmark_aggregations = Vec::new();

	// Get all unique subject names
	let unique_subjects = reviewer_data
		.iter()
		.flat_map(|reviewer| {
			reviewer
				.subjects
				.iter()
				.map(|subject| subject.name.as_str())
		})
		.dedup()
		.collect_vec();

	for &landmark in landmarks {
		let mut subject_measurements = Vec::new();

		for subject_name in &unique_subjects {
			let mut reviewer_measurements = Vec::new();

			for reviewer in reviewer_data {
				if let Some(marked_point) = reviewer
					.subjects
					.iter()
					.find(|subject| subject.name.eq(subject_name))
					.and_then(|subject| subject.landmarks.iter().find(|mp| mp.landmark == landmark))
				{
					reviewer_measurements.push(marked_point.mean());
				}
			}

			if reviewer_measurements.len() == reviewer_data.len() {
				subject_measurements.push(reviewer_measurements);
			}
		}

		let icc = if subject_measurements.is_empty() || subject_measurements[0].is_empty() {
			0.
		} else {
			landmark_icc_calculation(&subject_measurements)
		};

		landmark_aggregations.push(LandmarkAggregation { landmark, icc });
	}

	landmark_aggregations
}

/// Calculate ICC(3,k) - Two-Way Mixed Effects, Average Measurement
/// Use when: Same fixed raters evaluate all subjects, interested in average of k measurements
fn landmark_icc_calculation(subject_measurements: &[Vec<f64>]) -> f64 {
	let subject_count = subject_measurements.len() as f64;
	let reviewer_count = subject_measurements[0].len() as f64;

	let grand_mean = subject_measurements
		.iter()
		.flat_map(|reviewer_measurements| reviewer_measurements.iter())
		.mean();

	let subject_means = subject_measurements
		.iter()
		.map(|reviewer_measurements| reviewer_measurements.iter().mean())
		.collect_vec();

	let rater_means = (0..reviewer_count as usize)
		.map(|index| subject_measurements.iter().map(|row| row[index]).mean())
		.collect_vec();

	let between_subject_mean_square = reviewer_count
		* subject_means
			.iter()
			.map(|subject_mean| (subject_mean - grand_mean).powi(2))
			.sum::<f64>()
		/ (subject_count - 1.0);

	if between_subject_mean_square == 0. {
		return 0.;
	}

	let error_mean_square = subject_measurements
		.iter()
		.enumerate()
		.map(|(subject_index, row)| {
			row.iter()
				.enumerate()
				.map(|(reviewer_index, reviewer_measurement)| {
					(reviewer_measurement
						- subject_means[subject_index]
						- rater_means[reviewer_index]
						+ grand_mean)
						.powi(2)
				})
				.sum::<f64>()
		})
		.sum::<f64>()
		/ ((subject_count - 1.0) * (reviewer_count - 1.0));

	(between_subject_mean_square - error_mean_square) / between_subject_mean_square
}
