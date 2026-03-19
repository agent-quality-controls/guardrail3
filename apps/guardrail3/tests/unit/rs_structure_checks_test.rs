//! Tests extracted from `app::rs::validate::structure_checks`
#![allow(
    clippy::expect_used,
    clippy::disallowed_methods,
    clippy::indexing_slicing,
    clippy::panic,
    clippy::manual_assert
)] // reason: test crate

use std::path::Path;

use guardrail3::app::rs::validate::structure_checks::{check_file_length, check_use_count};
use guardrail3::domain::report::Severity;

// ---- Bug 4: Test file exemptions for R38 ----

#[test]
fn file_length_skips_test_files() {
    let long_content = "fn x() {}\n".repeat(600);
    let path = Path::new("/project/tests/my_test.rs");
    let mut results = Vec::new();
    check_file_length(path, &long_content, true, &mut results);
    assert!(
        results.is_empty(),
        "Test files should be exempt from length check"
    );
}

#[test]
fn file_length_flags_source_files_over_500() {
    let long_content = "fn x() {}\n".repeat(600);
    let path = Path::new("/project/src/main.rs");
    let mut results = Vec::new();
    check_file_length(path, &long_content, false, &mut results);
    assert!(
        !results.is_empty(),
        "Source files over 500 lines should be flagged"
    );
    assert_eq!(results[0].id, "R38");
    assert_eq!(results[0].severity, Severity::Error);
}

#[test]
fn use_count_skips_test_files() {
    let mut lines: Vec<String> = (0..25).map(|i| format!("use crate::module{i};")).collect();
    lines.push("fn test() {}".to_owned());
    let content = lines.join("\n");
    let path = Path::new("/project/tests/my_test.rs");
    let mut results = Vec::new();
    check_use_count(path, &content, true, &mut results);
    assert!(
        results.is_empty(),
        "Test files should be exempt from use-count check"
    );
}

// ---- R40: use count > 20 is Error ----

#[test]
fn r40_use_count_over_20_is_error() {
    let mut lines: Vec<String> = (0..21).map(|i| format!("use crate::mod{i};")).collect();
    lines.push("fn main() {}".to_owned());
    let content = lines.join("\n");
    let path = Path::new("src/foo.rs");
    let mut results = Vec::new();
    check_use_count(path, &content, false, &mut results);
    assert!(!results.is_empty(), "Should flag >20 use statements");
    assert_eq!(results[0].id, "R40");
    assert_eq!(results[0].severity, Severity::Error);
}

// ---- R41: use count 16-20 is Warn ----

#[test]
fn r41_use_count_16_is_warn() {
    let mut lines: Vec<String> = (0..16).map(|i| format!("use crate::mod{i};")).collect();
    lines.push("fn main() {}".to_owned());
    let content = lines.join("\n");
    let path = Path::new("src/foo.rs");
    let mut results = Vec::new();
    check_use_count(path, &content, false, &mut results);
    assert!(!results.is_empty(), "Should warn at 16 use statements");
    assert_eq!(results[0].id, "R41");
    assert_eq!(results[0].severity, Severity::Warn);
}
