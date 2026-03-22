use crate::domain::report::Severity;

use super::super::inputs::ConfigDenyInput;
use super::super::test_support::{canonical_deny_toml_service, config_facts};

#[test]
fn errors_when_allow_wildcard_paths_is_false() {
    let deny = config_facts(&canonical_deny_toml_service().replace(
        "allow-wildcard-paths = true",
        "allow-wildcard-paths = false",
    ));
    let input = ConfigDenyInput { config: &deny };
    let mut results = Vec::new();

    super::check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DENY-12");
    assert_eq!(result.severity, Severity::Error);
    assert_eq!(result.title, "allow-wildcard-paths must be true");
    assert_eq!(
        result.message,
        "`deny.toml` must set `[bans].allow-wildcard-paths = true`."
    );
}

#[test]
fn errors_when_bans_section_is_missing() {
    let deny = config_facts(
        &canonical_deny_toml_service()
            .replace("[bans]\n", "[removed]\n")
            .replace("[[bans.features]]", "[[removed.features]]"),
    );
    let input = ConfigDenyInput { config: &deny };
    let mut results = Vec::new();

    super::check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DENY-12");
    assert_eq!(result.severity, Severity::Error);
    assert_eq!(result.title, "[bans] section missing");
    assert_eq!(result.message, "`deny.toml` has no `[bans]` section.");
}
