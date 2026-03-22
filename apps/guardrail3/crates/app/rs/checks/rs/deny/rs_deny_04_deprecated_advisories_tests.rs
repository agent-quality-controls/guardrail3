use super::super::check;
use super::super::test_support::{canonical_deny_toml_service, root_tree_with_deny};

#[test]
fn warns_on_deprecated_advisory_fields() {
    let deny = canonical_deny_toml_service().replace("[advisories]\n", "[advisories]\nvulnerability = \"deny\"\n");
    let results = check(&root_tree_with_deny(&deny));
    assert!(results.iter().any(|r| r.id == "RS-DENY-04"));
}
