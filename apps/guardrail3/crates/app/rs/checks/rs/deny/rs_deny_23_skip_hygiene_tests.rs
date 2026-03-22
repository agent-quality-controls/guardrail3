use crate::domain::report::Severity;

use super::super::inputs::ConfigDenyInput;
use super::super::test_support::config_facts;
use super::check;

fn skip_toml(skip: &str) -> String {
    format!("[bans]\nskip = {skip}\n")
}

#[test]
fn warns_on_malformed_skip_entries_without_inventorying_them() {
    let config = config_facts(&skip_toml(r#"[{ reason = "good enough reason text" }]"#));
    let input = ConfigDenyInput { config: &config };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DENY-23");
    assert_eq!(result.severity, Severity::Warn);
    assert_eq!(result.title, "malformed skip entry");
    assert_eq!(
        result.message,
        "`deny.toml` has `[bans.skip]` entry without a valid crate identifier."
    );
    assert_eq!(result.file.as_deref(), Some("deny.toml"));
    assert!(!result.inventory);
}

#[test]
fn warns_on_missing_skip_reason_without_inventorying_the_entry() {
    let config = config_facts(&skip_toml(r#"[{ crate = "serde@1.0.0" }]"#));
    let input = ConfigDenyInput { config: &config };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DENY-23");
    assert_eq!(result.severity, Severity::Warn);
    assert_eq!(result.title, "skip entry missing reason");
    assert_eq!(
        result.message,
        "`deny.toml` skips `serde` without a `reason`."
    );
    assert_eq!(result.file.as_deref(), Some("deny.toml"));
    assert!(!result.inventory);
}

#[test]
fn inventories_valid_skip_entries() {
    let config = config_facts(&skip_toml(
        r#"[{ crate = "serde@1.0.0", reason = "good enough reason text" }]"#,
    ));
    let input = ConfigDenyInput { config: &config };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DENY-23");
    assert_eq!(result.severity, Severity::Info);
    assert_eq!(result.title, "skip entry");
    assert_eq!(result.message, "`deny.toml` has skip entry `serde`.");
    assert_eq!(result.file.as_deref(), Some("deny.toml"));
    assert!(result.inventory);
}
