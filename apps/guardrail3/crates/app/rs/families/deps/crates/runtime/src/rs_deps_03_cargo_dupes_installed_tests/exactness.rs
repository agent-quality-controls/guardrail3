use super::{collected_facts, project_tree};
use guardrail3_app_rs_family_deps_assertions::rs_deps_03_cargo_dupes_installed as assertions;

#[test]
fn missing_cargo_dupes_keeps_exact_warn_severity() {
    let facts = collected_facts(
        &project_tree(Vec::new(), Vec::new()),
        &["cargo-deny", "cargo-machete", "gitleaks"],
    );
    let results = super::run_with_facts(&facts);
    assertions::assert_exactness_summary(&results);
}
