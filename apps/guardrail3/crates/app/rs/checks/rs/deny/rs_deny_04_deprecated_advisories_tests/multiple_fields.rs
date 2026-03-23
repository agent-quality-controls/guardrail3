use crate::domain::report::Severity;

use super::super::super::inputs::ConfigDenyInput;
use super::super::super::test_support::{
    canonical_deny_toml_service, config_facts, set_section_string,
};
use super::super::check;

#[test]
fn warns_once_per_deprecated_advisory_field() {
    let deny = set_section_string(
        &set_section_string(
            &set_section_string(
                &canonical_deny_toml_service(),
                "advisories",
                "vulnerability",
                "deny",
            ),
            "advisories",
            "notice",
            "warn",
        ),
        "advisories",
        "unsound",
        "deny",
    );
    let config = config_facts(&deny);
    let input = ConfigDenyInput { config: &config };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 3);

    let titles = results.iter().map(|r| r.title.as_str()).collect::<Vec<_>>();
    assert_eq!(
        titles,
        vec![
            "deprecated advisory field `vulnerability`",
            "deprecated advisory field `notice`",
            "deprecated advisory field `unsound`",
        ]
    );

    let messages = results
        .iter()
        .map(|r| r.message.as_str())
        .collect::<Vec<_>>();
    assert_eq!(
        messages,
        vec![
            "`deny.toml` uses deprecated `[advisories].vulnerability`.",
            "`deny.toml` uses deprecated `[advisories].notice`.",
            "`deny.toml` uses deprecated `[advisories].unsound`.",
        ]
    );

    for result in &results {
        assert_eq!(result.id, "RS-DENY-04");
        assert_eq!(result.severity, Severity::Warn);
        assert_eq!(result.file.as_deref(), Some("deny.toml"));
        assert!(!result.inventory);
    }
}
