use crate::domain::report::Severity;

use super::super::super::inputs::ConfigDenyInput;
use super::super::super::test_support::{
    canonical_deny_toml_service, config_facts, remove_section_key, set_section_bool,
};
use super::super::check;

#[test]
fn errors_when_allow_wildcard_paths_is_missing_or_false() {
    let missing = config_facts(&remove_section_key(
        &canonical_deny_toml_service(),
        "bans",
        "allow-wildcard-paths",
    ));
    let wrong = config_facts(&set_section_bool(
        &canonical_deny_toml_service(),
        "bans",
        "allow-wildcard-paths",
        false,
    ));

    for config in [&missing, &wrong] {
        let input = ConfigDenyInput { config };
        let mut results = Vec::new();

        check(&input, &mut results);

        assert_eq!(results.len(), 1);
        let result = &results[0];
        assert_eq!(result.id, "RS-DENY-12");
        assert_eq!(result.severity, Severity::Error);
        assert_eq!(result.title, "allow-wildcard-paths must be true");
        assert_eq!(
            result.message,
            "`deny.toml` must set `[bans].allow-wildcard-paths = true`."
        );
        assert_eq!(result.file.as_deref(), Some("deny.toml"));
        assert!(!result.inventory);
    }
}
