use guardrail3_domain_report::Severity;

use super::super::super::inputs::ConfigDenyInput;
use super::super::super::test_support::{
    canonical_deny_toml_service, config_facts, set_section_string,
};
use super::super::check;

#[test]
fn errors_when_advisories_baseline_is_weakened() {
    let deny = set_section_string(
        &set_section_string(
            &canonical_deny_toml_service(),
            "advisories",
            "unmaintained",
            "allow",
        ),
        "advisories",
        "yanked",
        "allow",
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
            "advisories `unmaintained` has wrong value",
            "advisories `yanked` has wrong value",
        ]
    );

    let messages = results
        .iter()
        .map(|r| r.message.as_str())
        .collect::<Vec<_>>();
    assert_eq!(
        messages,
        vec![
            "`deny.toml` must set `[advisories].unmaintained = \"workspace\"`, found `allow`.",
            "`deny.toml` must set `[advisories].yanked = \"warn\"`, found `allow`.",
        ]
    );

    for result in &results {
        assert_eq!(result.id, "RS-DENY-05");
        assert_eq!(result.severity, Severity::Error);
        assert_eq!(result.file.as_deref(), Some("deny.toml"));
        assert!(!result.inventory);
    }
}
