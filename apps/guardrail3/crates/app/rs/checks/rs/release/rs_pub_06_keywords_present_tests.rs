use crate::domain::report::Severity;

use super::super::test_support::{crate_facts, crate_input};
use super::check;

#[test]
fn warns_on_too_many_keywords() {
    let mut facts = crate_facts("x");
    facts.keywords_count = Some(6);
    let input = crate_input(&facts);
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
}
