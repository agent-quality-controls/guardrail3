use crate::domain::report::Severity;

use super::super::super::inputs::ConfigDenyInput;
use super::super::super::test_support::config_facts;
use super::super::check;

#[test]
fn warns_when_skip_container_is_not_an_array() {
    let config = config_facts("[bans]\nskip = \"serde\"\n");
    let input = ConfigDenyInput { config: &config };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DENY-23");
    assert_eq!(result.severity, Severity::Warn);
    assert_eq!(result.title, "malformed skip container");
    assert_eq!(
        result.message,
        "`deny.toml` must use an array for `[bans].skip` entries."
    );
    assert_eq!(result.file.as_deref(), Some("deny.toml"));
}
