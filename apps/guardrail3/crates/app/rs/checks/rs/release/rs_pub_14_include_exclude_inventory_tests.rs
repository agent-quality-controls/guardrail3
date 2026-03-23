use super::super::test_support::{crate_facts, crate_input};
use super::check;

#[test]
fn emits_info_when_include_exclude_missing() {
    let mut facts = crate_facts("x");
    facts.include_exclude_present = false;
    let input = crate_input(&facts);
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].inventory, false);
}
