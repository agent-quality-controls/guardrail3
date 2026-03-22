use crate::domain::report::Severity;

use super::super::inputs::ConfigDenyInput;
use super::super::test_support::{canonical_deny_toml_service, config_facts};
use super::check;

#[test]
fn warns_on_malformed_ignore_entries_without_inventorying_them() {
    let config =
        config_facts(&canonical_deny_toml_service().replace("ignore = []", "ignore = [{ reason = 1 }]"));
    let input = ConfigDenyInput { config: &config };
    let mut results = Vec::new();

    check(&input, &mut results);

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
    let config = config_facts(&canonical_deny_toml_service().replace(
        "ignore = []",
        "ignore = [{ id = \"RUSTSEC-2026-0001\" }]",
    ));
    let input = ConfigDenyInput { config: &config };
    let mut results = Vec::new();

    check(&input, &mut results);

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
    let config = config_facts(&canonical_deny_toml_service().replace(
        "ignore = []",
        "ignore = [{ id = \"RUSTSEC-2026-0001\", reason = \"good enough reason text\" }]",
    ));
    let input = ConfigDenyInput { config: &config };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(results.iter().any(|result| {
        result.id == "RS-DENY-24"
            && result.inventory
            && result.title == "advisory ignore entry"
            && result.message.contains("RUSTSEC-2026-0001")
    }));
}
