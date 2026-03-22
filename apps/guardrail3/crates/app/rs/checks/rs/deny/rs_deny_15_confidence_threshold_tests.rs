use super::super::check;
use super::super::test_support::{canonical_deny_toml_service, root_tree_with_deny};

#[test]
fn warns_when_confidence_threshold_is_weaker() {
    let deny = canonical_deny_toml_service().replace("confidence-threshold = 0.8", "confidence-threshold = 0.7");
    let results = check(&root_tree_with_deny(&deny));
    assert!(results.iter().any(|r| r.id == "RS-DENY-15"));
}
