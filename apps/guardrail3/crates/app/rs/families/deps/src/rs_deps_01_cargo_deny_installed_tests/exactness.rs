use crate::run_with_facts;
use crate::test_support::{collected_facts, project_tree};
use guardrail3_domain_report::Severity;

#[test]
fn missing_cargo_deny_only_hits_its_own_rule() {
    let facts = collected_facts(
        &project_tree(Vec::new(), Vec::new()),
        &["cargo-machete", "cargo-dupes", "gitleaks"],
    );
    let results = run_with_facts(&facts);
    let summary = results
        .iter()
        .map(|result| (result.id.as_str(), result.severity, result.inventory))
        .collect::<Vec<_>>();

    assert_eq!(
        summary,
        vec![
            ("RS-DEPS-01", Severity::Error, false),
            ("RS-DEPS-02", Severity::Info, true),
            ("RS-DEPS-03", Severity::Info, true),
            ("RS-DEPS-04", Severity::Info, true),
        ]
    );
}
