use super::super::super::test_support::{crate_facts, crate_input};
use super::super::check;

#[test]
fn does_not_warn_for_non_publishable_internal_crates() {
    let mut facts = crate_facts("internal");
    facts.publishable = false;
    facts.description_present = false;
    facts.license_present = false;
    facts.repository_present = false;
    let input = crate_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(results.is_empty());
}

#[test]
fn does_not_warn_when_any_release_metadata_is_present() {
    let mut facts = crate_facts("public");
    facts.description_present = true;
    facts.license_present = false;
    facts.repository_present = false;
    let input = crate_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(results.is_empty());
}
