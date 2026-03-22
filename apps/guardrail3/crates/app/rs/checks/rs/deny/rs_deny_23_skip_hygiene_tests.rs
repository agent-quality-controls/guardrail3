use crate::domain::report::Severity;

use super::super::check;
use super::super::test_support::{canonical_deny_toml_service, root_tree_with_deny};

#[test]
fn warns_on_malformed_skip_entries_without_inventorying_them() {
    let deny = canonical_deny_toml_service().replace("skip = []", "skip = [{ version = \"1.0.0\" }]");
    let results = check(&root_tree_with_deny(&deny));

    assert!(results.iter().any(|result| {
        result.id == "RS-DENY-23"
            && result.severity == Severity::Warn
            && result.title == "malformed skip entry"
            && result.file.as_deref() == Some("deny.toml")
    }));
    assert!(!results.iter().any(|result| {
        result.id == "RS-DENY-23" && result.inventory && result.title == "skip entry"
    }));
}

#[test]
fn warns_on_missing_skip_reason_without_inventorying_the_entry() {
    let deny = canonical_deny_toml_service().replace(
        "skip = []",
        "skip = [{ crate = \"serde@1.0.0\" }]",
    );
    let results = check(&root_tree_with_deny(&deny));

    assert!(results.iter().any(|result| {
        result.id == "RS-DENY-23"
            && result.severity == Severity::Warn
            && result.title == "skip entry missing reason"
            && result.message.contains("serde")
    }));
    assert!(!results.iter().any(|result| {
        result.id == "RS-DENY-23" && result.inventory && result.message.contains("serde")
    }));
}

#[test]
fn inventories_valid_skip_entries() {
    let deny = canonical_deny_toml_service().replace(
        "skip = []",
        "skip = [{ crate = \"serde@1.0.0\", reason = \"good enough reason text\" }]",
    );
    let results = check(&root_tree_with_deny(&deny));

    assert!(results.iter().any(|result| {
        result.id == "RS-DENY-23"
            && result.inventory
            && result.title == "skip entry"
            && result.message.contains("serde")
    }));
}
