use super::super::check;
use super::super::test_support::config_hygiene_tree;

#[test]
fn warns_when_test_relaxations_are_enabled() {
    let results = check(&config_hygiene_tree());
    assert!(results.iter().any(|r| r.id == "RS-CLIPPY-17" && !r.inventory));
}
