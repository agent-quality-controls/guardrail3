use crate::domain::report::Severity;

use super::super::super::inputs::ConfigDenyInput;
use super::super::super::test_support::{
    add_allowed_license, canonical_deny_toml_service, config_facts,
};
use super::super::check;

#[test]
fn warns_once_per_copyleft_license_in_allow_list() {
    let deny = add_allowed_license(
        &add_allowed_license(&canonical_deny_toml_service(), "GPL-3.0"),
        "LGPL-3.0",
    );
    let config = config_facts(&deny);
    let input = ConfigDenyInput { config: &config };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 2);

    let messages = results
        .iter()
        .map(|r| r.message.as_str())
        .collect::<Vec<_>>();
    assert_eq!(
        messages,
        vec![
            "`deny.toml` allows copyleft license `GPL-3.0`.",
            "`deny.toml` allows copyleft license `LGPL-3.0`.",
        ]
    );

    for result in &results {
        assert_eq!(result.id, "RS-DENY-16");
        assert_eq!(result.severity, Severity::Warn);
        assert_eq!(result.title, "copyleft license allowed");
        assert_eq!(result.file.as_deref(), Some("deny.toml"));
        assert!(!result.inventory);
    }
}
