use crate::domain::report::Severity;

use super::super::test_support::{crate_facts, crate_input};
use super::check;

#[test]
fn warns_on_stub_readme() {
    let mut facts = crate_facts("x");
    facts.readme_content = Some("# x\nshort".to_owned());
    let input = crate_input(&facts);
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
}

#[test]
fn inventories_good_readme() {
    let facts = crate_facts("x");
    let input = crate_input(&facts);
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}
