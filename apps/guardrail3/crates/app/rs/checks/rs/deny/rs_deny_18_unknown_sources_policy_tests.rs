use crate::domain::report::Severity;

use super::super::inputs::ConfigDenyInput;
use super::super::test_support::{canonical_deny_toml_service, config_facts};

#[test]
fn errors_when_unknown_git_policy_is_weakened() {
    let deny = config_facts(
        &canonical_deny_toml_service().replace("unknown-git = \"deny\"", "unknown-git = \"allow\""),
    );
    let input = ConfigDenyInput { config: &deny };
    let mut results = Vec::new();

    super::check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DENY-18");
    assert_eq!(result.severity, Severity::Error);
    assert_eq!(result.title, "sources `unknown-git` has wrong value");
    assert_eq!(
        result.message,
        "`deny.toml` must set `[sources].unknown-git = \"deny\"`."
    );
}

#[test]
fn errors_when_sources_section_is_missing() {
    let deny = config_facts(&canonical_deny_toml_service().replace("[sources]\n", "[removed]\n"));
    let input = ConfigDenyInput { config: &deny };
    let mut results = Vec::new();

    super::check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DENY-18");
    assert_eq!(result.severity, Severity::Error);
    assert_eq!(result.title, "[sources] section missing");
    assert_eq!(result.message, "`deny.toml` has no `[sources]` section.");
}
