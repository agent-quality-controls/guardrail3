use guardrail3_domain_report::Severity;

use super::super::super::inputs::ConfigDenyInput;
use super::super::super::test_support::{
    canonical_deny_toml_service, config_facts, remove_section_key, set_section_string,
};
use super::super::check;

#[test]
fn warns_when_wildcards_is_missing_or_project_specific() {
    let missing = config_facts(&remove_section_key(
        &canonical_deny_toml_service(),
        "bans",
        "wildcards",
    ));
    let custom = config_facts(&set_section_string(
        &canonical_deny_toml_service(),
        "bans",
        "wildcards",
        "deny",
    ));

    for (config, expected) in [
        (&missing, "`deny.toml` sets `[bans].wildcards = <missing>`."),
        (&custom, "`deny.toml` sets `[bans].wildcards = deny`."),
    ] {
        let input = ConfigDenyInput { config };
        let mut results = Vec::new();

        check(&input, &mut results);

        assert_eq!(results.len(), 1);
        let result = &results[0];
        assert_eq!(result.id, "RS-DENY-13");
        assert_eq!(result.severity, Severity::Warn);
        assert_eq!(result.title, "wildcards differs from baseline");
        assert_eq!(result.message, expected);
        assert_eq!(result.file.as_deref(), Some("deny.toml"));
        assert!(!result.inventory);
    }
}
