use crate::domain::report::Severity;

use super::super::test_support::{crate_facts, crate_input};
use super::check;

#[test]
fn warns_without_categories() {
    let mut facts = crate_facts("x");
    facts.categories_count = None;
    let input = crate_input(&facts);
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
}
