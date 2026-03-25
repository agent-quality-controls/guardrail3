use guardrail3_domain_report::Severity;

use super::super::super::inputs::ConfigDenyInput;
use super::super::super::test_support::{
    canonical_deny_toml_service, config_facts, remove_section_key,
};
use super::super::check;

#[test]
fn errors_when_baseline_advisory_values_are_missing() {
    let deny = remove_section_key(
        &remove_section_key(&canonical_deny_toml_service(), "advisories", "unmaintained"),
        "advisories",
        "yanked",
    );
    let config = config_facts(&deny);
    let input = ConfigDenyInput { config: &config };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 2);

    let titles = results.iter().map(|r| r.title.as_str()).collect::<Vec<_>>();
    assert_eq!(
        titles,
        vec![
            "advisories `unmaintained` missing",
            "advisories `yanked` missing",
        ]
    );

    let messages = results
        .iter()
        .map(|r| r.message.as_str())
        .collect::<Vec<_>>();
    assert_eq!(
        messages,
        vec![
            "`deny.toml` must set `[advisories].unmaintained = \"workspace\"`.",
            "`deny.toml` must set `[advisories].yanked = \"warn\"`.",
        ]
    );

    for result in &results {
        assert_eq!(result.id, "RS-DENY-05");
        assert_eq!(result.severity, Severity::Error);
        assert_eq!(result.file.as_deref(), Some("deny.toml"));
        assert!(!result.inventory);
    }
}
