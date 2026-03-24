use crate::app::rs::checks::rs::deps::test_support::{failure_facts, failure_input};
use crate::domain::report::Severity;

#[test]
fn emits_error_for_input_failure() {
    let facts = failure_facts("guardrail3.toml", "parse failed");
    let input = failure_input(&facts, "guardrail3.toml");
    let mut results = Vec::new();

    super::super::check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DEPS-11");
    assert_eq!(result.severity, Severity::Error);
    assert_eq!(result.message, "parse failed");
}
