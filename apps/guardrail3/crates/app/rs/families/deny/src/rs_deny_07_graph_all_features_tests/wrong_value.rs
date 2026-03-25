use guardrail3_domain_report::Severity;

use super::super::super::inputs::ConfigDenyInput;
use super::super::super::test_support::{
    canonical_deny_toml_service, config_facts, remove_section_key, set_section_bool,
};
use super::super::check;

#[test]
fn errors_when_all_features_is_missing_or_false() {
    let missing = config_facts(&remove_section_key(
        &canonical_deny_toml_service(),
        "graph",
        "all-features",
    ));
    let wrong = config_facts(&set_section_bool(
        &canonical_deny_toml_service(),
        "graph",
        "all-features",
        false,
    ));

    for config in [&missing, &wrong] {
        let input = ConfigDenyInput { config };
        let mut results = Vec::new();

        check(&input, &mut results);

        assert_eq!(results.len(), 1);
        let result = &results[0];
        assert_eq!(result.id, "RS-DENY-07");
        assert_eq!(result.severity, Severity::Error);
        assert_eq!(result.title, "graph all-features must be true");
        assert_eq!(
            result.message,
            "`deny.toml` must set `[graph].all-features = true`."
        );
        assert_eq!(result.file.as_deref(), Some("deny.toml"));
        assert!(!result.inventory);
    }
}
