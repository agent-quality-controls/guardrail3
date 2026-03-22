use super::super::check;
use super::super::test_support::config_hygiene_tree;

#[test]
fn warns_on_managed_key_typos() {
    let results = check(&config_hygiene_tree());
    assert!(results.iter().any(|r| r.id == "RS-CLIPPY-19" && !r.inventory && r.message.contains("disalowed-methods")));
}
