use super::Exporter;
use crate::benchmark::benchmark_result::BenchmarkResult;
use crate::benchmark::relative_speed;
use crate::export::markup::markup_results;
use crate::export::markup::markup_table;
use crate::export::markup::markup_unit;
use crate::export::markup::MarkupType;
use crate::util::units::Unit;

use anyhow::{anyhow, Result};

#[derive(Default)]
pub struct MarkdownExporter {}

impl Exporter for MarkdownExporter {
    fn serialize(&self, results: &[BenchmarkResult], unit: Option<Unit>) -> Result<Vec<u8>> {
        let entries = relative_speed::compute(results);
        if entries.is_none() {
            return Err(anyhow!(
                "Relative speed comparison is not available for Markdown export."
            ));
        }

        let kind = MarkupType::Markdown;
        let unit = markup_unit(results, unit);
        let data = markup_results(&kind, &entries.unwrap(), unit);
        let table = markup_table(&kind, &data);
        Ok(table.as_bytes().to_vec())
    }
}

/// Test helper function to create unit-based header and horizontal line
/// independently from the markup functionality for Markdown.
#[cfg(test)]
fn test_table_header(unit_short_name: String) -> String {
    format!(
        "| Command | Mean [{unit}] | Min [{unit}] | Max [{unit}] | Relative |\n|:---|---:|---:|---:|---:|\n",
        unit = unit_short_name
    )
}

/// Ensure the markdown output includes the table header and the multiple
/// benchmark results as a table. The list of actual times is not included
/// in the output.
///
/// This also demonstrates that the first entry's units (ms) are used to set
/// the units for all entries when the time unit is not given.
#[test]
fn test_markdown_format_ms() {
    use std::collections::BTreeMap;
    let exporter = MarkdownExporter::default();

    let timing_results = vec![
        BenchmarkResult {
            command: String::from("sleep 0.1"),
            mean: 0.1057,
            stddev: Some(0.0016),
            median: 0.1057,
            user: 0.0009,
            system: 0.0011,
            min: 0.1023,
            max: 0.1080,
            times: Some(vec![0.1, 0.1, 0.1]),
            exit_codes: vec![Some(0), Some(0), Some(0)],
            parameters: BTreeMap::new(),
        },
        BenchmarkResult {
            command: String::from("sleep 2"),
            mean: 2.0050,
            stddev: Some(0.0020),
            median: 2.0050,
            user: 0.0009,
            system: 0.0012,
            min: 2.0020,
            max: 2.0080,
            times: Some(vec![2.0, 2.0, 2.0]),
            exit_codes: vec![Some(0), Some(0), Some(0)],
            parameters: BTreeMap::new(),
        },
    ];

    let formatted = String::from_utf8(exporter.serialize(&timing_results, None).unwrap()).unwrap();

    let formatted_expected = format!(
        "{}\
| `sleep 0.1` | 105.7 ± 1.6 | 102.3 | 108.0 | 1.00 |
| `sleep 2` | 2005.0 ± 2.0 | 2002.0 | 2008.0 | 18.97 ± 0.29 |
",
        test_table_header("ms".to_string())
    );

    assert_eq!(formatted_expected, formatted);
}

/// This (again) demonstrates that the first entry's units (s) are used to set
/// the units for all entries when the time unit is not given.
#[test]
fn test_markdown_format_s() {
    use std::collections::BTreeMap;
    let exporter = MarkdownExporter::default();

    let timing_results = vec![
        BenchmarkResult {
            command: String::from("sleep 2"),
            mean: 2.0050,
            stddev: Some(0.0020),
            median: 2.0050,
            user: 0.0009,
            system: 0.0012,
            min: 2.0020,
            max: 2.0080,
            times: Some(vec![2.0, 2.0, 2.0]),
            exit_codes: vec![Some(0), Some(0), Some(0)],
            parameters: BTreeMap::new(),
        },
        BenchmarkResult {
            command: String::from("sleep 0.1"),
            mean: 0.1057,
            stddev: Some(0.0016),
            median: 0.1057,
            user: 0.0009,
            system: 0.0011,
            min: 0.1023,
            max: 0.1080,
            times: Some(vec![0.1, 0.1, 0.1]),
            exit_codes: vec![Some(0), Some(0), Some(0)],
            parameters: BTreeMap::new(),
        },
    ];

    let formatted = String::from_utf8(exporter.serialize(&timing_results, None).unwrap()).unwrap();

    let formatted_expected = format!(
        "{}\
| `sleep 2` | 2.005 ± 0.002 | 2.002 | 2.008 | 18.97 ± 0.29 |
| `sleep 0.1` | 0.106 ± 0.002 | 0.102 | 0.108 | 1.00 |
",
        test_table_header("s".to_string())
    );

    assert_eq!(formatted_expected, formatted);
}

/// The given time unit (s) is used to set the units for all entries.
#[test]
fn test_markdown_format_time_unit_s() {
    use std::collections::BTreeMap;
    let exporter = MarkdownExporter::default();

    let timing_results = vec![
        BenchmarkResult {
            command: String::from("sleep 0.1"),
            mean: 0.1057,
            stddev: Some(0.0016),
            median: 0.1057,
            user: 0.0009,
            system: 0.0011,
            min: 0.1023,
            max: 0.1080,
            times: Some(vec![0.1, 0.1, 0.1]),
            exit_codes: vec![Some(0), Some(0), Some(0)],
            parameters: BTreeMap::new(),
        },
        BenchmarkResult {
            command: String::from("sleep 2"),
            mean: 2.0050,
            stddev: Some(0.0020),
            median: 2.0050,
            user: 0.0009,
            system: 0.0012,
            min: 2.0020,
            max: 2.0080,
            times: Some(vec![2.0, 2.0, 2.0]),
            exit_codes: vec![Some(0), Some(0), Some(0)],
            parameters: BTreeMap::new(),
        },
    ];

    let formatted = String::from_utf8(
        exporter
            .serialize(&timing_results, Some(Unit::Second))
            .unwrap(),
    )
    .unwrap();

    let formatted_expected = format!(
        "{}\
| `sleep 0.1` | 0.106 ± 0.002 | 0.102 | 0.108 | 1.00 |
| `sleep 2` | 2.005 ± 0.002 | 2.002 | 2.008 | 18.97 ± 0.29 |
",
        test_table_header("s".to_string())
    );

    assert_eq!(formatted_expected, formatted);
}

/// This (again) demonstrates that the given time unit (ms) is used to set
/// the units for all entries.
#[test]
fn test_markdown_format_time_unit_ms() {
    use std::collections::BTreeMap;
    let exporter = MarkdownExporter::default();

    let timing_results = vec![
        BenchmarkResult {
            command: String::from("sleep 2"),
            mean: 2.0050,
            stddev: Some(0.0020),
            median: 2.0050,
            user: 0.0009,
            system: 0.0012,
            min: 2.0020,
            max: 2.0080,
            times: Some(vec![2.0, 2.0, 2.0]),
            exit_codes: vec![Some(0), Some(0), Some(0)],
            parameters: BTreeMap::new(),
        },
        BenchmarkResult {
            command: String::from("sleep 0.1"),
            mean: 0.1057,
            stddev: Some(0.0016),
            median: 0.1057,
            user: 0.0009,
            system: 0.0011,
            min: 0.1023,
            max: 0.1080,
            times: Some(vec![0.1, 0.1, 0.1]),
            exit_codes: vec![Some(0), Some(0), Some(0)],
            parameters: BTreeMap::new(),
        },
    ];

    let formatted = String::from_utf8(
        exporter
            .serialize(&timing_results, Some(Unit::MilliSecond))
            .unwrap(),
    )
    .unwrap();

    let formatted_expected = format!(
        "{}\
| `sleep 2` | 2005.0 ± 2.0 | 2002.0 | 2008.0 | 18.97 ± 0.29 |
| `sleep 0.1` | 105.7 ± 1.6 | 102.3 | 108.0 | 1.00 |
",
        test_table_header("ms".to_string())
    );

    assert_eq!(formatted_expected, formatted);
}
