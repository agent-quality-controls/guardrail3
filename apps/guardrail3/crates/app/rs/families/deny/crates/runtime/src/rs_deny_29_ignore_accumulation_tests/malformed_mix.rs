use guardrail3_domain_report::Severity;

use super::super::ConfigDenyInput;
use super::super::config_facts;

#[test]
fn counts_entries_by_container_length_even_when_some_ignore_entries_are_malformed() {
    let deny = config_facts(
        "[advisories]\nignore = [\"A\", { reason = \"good enough reason text\" }, \"C\", \"D\", \"E\", \"F\"]\n",
    );
    let input = ConfigDenyInput { config: &deny };
    let mut results = Vec::new();

    super::super::check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DENY-29");
    assert_eq!(result.severity, Severity::Warn);
    assert_eq!(result.title, "advisory ignore list is large");
    assert_eq!(
        result.message,
        "`deny.toml` has 6 `[advisories].ignore` entries (threshold: 5)."
    );
}
