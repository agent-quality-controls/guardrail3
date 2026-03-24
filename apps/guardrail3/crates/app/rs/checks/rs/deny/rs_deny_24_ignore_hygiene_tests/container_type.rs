use crate::domain::report::Severity;

use super::super::super::inputs::ConfigDenyInput;
use super::super::super::test_support::config_facts;
use super::super::check;

#[test]
fn warns_when_ignore_container_is_not_an_array() {
    let config = config_facts("[advisories]\nignore = \"RUSTSEC-2026-0001\"\n");
    let input = ConfigDenyInput { config: &config };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DENY-24");
    assert_eq!(result.severity, Severity::Warn);
    assert_eq!(result.title, "malformed advisory ignore container");
    assert_eq!(
        result.message,
        "`deny.toml` must use an array for `[advisories].ignore` entries."
    );
    assert_eq!(result.file.as_deref(), Some("deny.toml"));
}
