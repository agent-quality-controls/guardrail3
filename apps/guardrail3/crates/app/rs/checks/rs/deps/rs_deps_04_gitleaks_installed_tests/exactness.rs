use crate::app::rs::checks::rs::deps::run_with_facts;
use crate::app::rs::checks::rs::deps::test_support::{collected_facts, project_tree};
use crate::domain::report::Severity;

#[test]
fn missing_gitleaks_only_hits_its_own_rule() {
    let facts = collected_facts(
        &project_tree(Vec::new(), Vec::new()),
        &["cargo-deny", "cargo-machete", "cargo-dupes"],
    );
    let results = run_with_facts(&facts);
    let summary = results
        .iter()
        .map(|result| (result.id.as_str(), result.severity, result.inventory))
        .collect::<Vec<_>>();

    assert_eq!(
        summary,
        vec![
            ("RS-DEPS-01", Severity::Info, true),
            ("RS-DEPS-02", Severity::Info, true),
            ("RS-DEPS-03", Severity::Info, true),
            ("RS-DEPS-04", Severity::Error, false),
        ]
    );
}
