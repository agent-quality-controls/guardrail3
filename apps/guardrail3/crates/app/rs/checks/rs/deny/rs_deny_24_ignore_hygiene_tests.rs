use crate::domain::report::Severity;

use super::super::inputs::ConfigDenyInput;
use super::super::test_support::config_facts;
use super::check;

fn ignore_toml(ignore: &str) -> String {
    format!("[advisories]\nignore = {ignore}\n")
}

#[test]
fn warns_on_malformed_ignore_entries_without_inventorying_them() {
    let config = config_facts(&ignore_toml(r#"[{ reason = "good enough reason text" }]"#));
    let input = ConfigDenyInput { config: &config };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DENY-24");
    assert_eq!(result.severity, Severity::Warn);
    assert_eq!(result.title, "malformed advisory ignore entry");
    assert_eq!(
        result.message,
        "`deny.toml` has an `[advisories].ignore` entry without a valid advisory id."
    );
    assert_eq!(result.file.as_deref(), Some("deny.toml"));
    assert!(!result.inventory);
}

#[test]
fn warns_on_missing_ignore_reason_without_inventorying_the_entry() {
    let config = config_facts(&ignore_toml(r#"[{ id = "RUSTSEC-2026-0001" }]"#));
    let input = ConfigDenyInput { config: &config };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DENY-24");
    assert_eq!(result.severity, Severity::Warn);
    assert_eq!(result.title, "advisory ignore missing reason");
    assert_eq!(
        result.message,
        "`deny.toml` ignores advisory `RUSTSEC-2026-0001` without a `reason`."
    );
    assert_eq!(result.file.as_deref(), Some("deny.toml"));
    assert!(!result.inventory);
}

#[test]
fn inventories_valid_ignore_entries() {
    let config = config_facts(&ignore_toml(
        r#"[{ id = "RUSTSEC-2026-0001", reason = "good enough reason text" }]"#,
    ));
    let input = ConfigDenyInput { config: &config };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DENY-24");
    assert_eq!(result.severity, Severity::Info);
    assert_eq!(result.title, "advisory ignore entry");
    assert_eq!(
        result.message,
        "`deny.toml` ignores advisory `RUSTSEC-2026-0001`."
    );
    assert_eq!(result.file.as_deref(), Some("deny.toml"));
    assert!(result.inventory);
}
