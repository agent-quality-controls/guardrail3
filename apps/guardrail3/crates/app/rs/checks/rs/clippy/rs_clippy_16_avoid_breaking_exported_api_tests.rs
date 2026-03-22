use super::super::check;
use super::super::test_support::config_hygiene_tree;

#[test]
fn warns_when_avoid_breaking_exported_api_is_true() {
    let results = check(&config_hygiene_tree());
    assert!(results.iter().any(|r| r.id == "RS-CLIPPY-16" && !r.inventory));
}
