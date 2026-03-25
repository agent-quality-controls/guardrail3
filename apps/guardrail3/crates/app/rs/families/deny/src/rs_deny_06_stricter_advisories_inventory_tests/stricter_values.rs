use guardrail3_domain_report::Severity;

use super::super::super::inputs::ConfigDenyInput;
use super::super::super::test_support::{
    canonical_deny_toml_service, config_facts, set_section_string,
};
use super::super::check;

#[test]
fn inventories_each_advisory_key_that_is_stricter_than_baseline() {
    let deny = set_section_string(
        &set_section_string(
            &canonical_deny_toml_service(),
            "advisories",
            "unmaintained",
            "deny",
        ),
        "advisories",
        "yanked",
        "deny",
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
            "advisories `unmaintained` stricter than baseline",
            "advisories `yanked` stricter than baseline",
        ]
    );

    let messages = results
        .iter()
        .map(|r| r.message.as_str())
        .collect::<Vec<_>>();
    assert_eq!(
        messages,
        vec![
            "`deny.toml` sets `[advisories].unmaintained = \"deny\"`.",
            "`deny.toml` sets `[advisories].yanked = \"deny\"`.",
        ]
    );

    for result in &results {
        assert_eq!(result.id, "RS-DENY-06");
        assert_eq!(result.severity, Severity::Info);
        assert_eq!(result.file.as_deref(), Some("deny.toml"));
        assert!(result.inventory);
    }
}
