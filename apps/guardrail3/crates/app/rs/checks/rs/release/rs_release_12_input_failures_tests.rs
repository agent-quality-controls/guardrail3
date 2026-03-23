use crate::domain::report::Severity;

use super::super::test_support::{failure, failure_input};
use super::check;

#[test]
fn errors_on_input_failures() {
    let failure = failure("release-plz.toml", "parse failed");
    let input = failure_input(&failure);
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Error);
}
