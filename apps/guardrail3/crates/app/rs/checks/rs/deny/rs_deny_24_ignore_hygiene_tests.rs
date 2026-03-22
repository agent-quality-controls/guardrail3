use crate::domain::report::Severity;

use super::super::check;
use super::super::test_support::{canonical_deny_toml_service, root_tree_with_deny};

#[test]
fn warns_on_malformed_ignore_entries_without_inventorying_them() {
    let deny = canonical_deny_toml_service().replace("ignore = []", "ignore = [{ reason = 1 }]");
    let results = check(&root_tree_with_deny(&deny));

    assert!(results.iter().any(|result| {
        result.id == "RS-DENY-24"
            && result.severity == Severity::Warn
            && result.title == "malformed advisory ignore entry"
    }));
    assert!(!results.iter().any(|result| {
        result.id == "RS-DENY-24" && result.inventory && result.title == "advisory ignore entry"
    }));
}

#[test]
fn warns_on_missing_ignore_reason_without_inventorying_the_entry() {
    let deny = canonical_deny_toml_service().replace(
        "ignore = []",
        "ignore = [{ id = \"RUSTSEC-2026-0001\" }]",
    );
    let results = check(&root_tree_with_deny(&deny));

    assert!(results.iter().any(|result| {
        result.id == "RS-DENY-24"
            && result.severity == Severity::Warn
            && result.title == "advisory ignore missing reason"
            && result.message.contains("RUSTSEC-2026-0001")
    }));
    assert!(!results.iter().any(|result| {
        result.id == "RS-DENY-24"
            && result.inventory
            && result.message.contains("RUSTSEC-2026-0001")
    }));
}

#[test]
fn inventories_valid_ignore_entries() {
    let deny = canonical_deny_toml_service().replace(
        "ignore = []",
        "ignore = [{ id = \"RUSTSEC-2026-0001\", reason = \"good enough reason text\" }]",
    );
    let results = check(&root_tree_with_deny(&deny));

    assert!(results.iter().any(|result| {
        result.id == "RS-DENY-24"
            && result.inventory
            && result.title == "advisory ignore entry"
            && result.message.contains("RUSTSEC-2026-0001")
    }));
}
