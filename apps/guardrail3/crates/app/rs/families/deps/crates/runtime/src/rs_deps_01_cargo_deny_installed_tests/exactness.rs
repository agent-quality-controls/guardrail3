use super::{collected_facts, project_tree};
use guardrail3_app_rs_family_deps_assertions::rs_deps_01_cargo_deny_installed as assertions;

#[test]
fn missing_cargo_deny_only_hits_its_own_rule() {
    let facts = collected_facts(
        &project_tree(Vec::new(), Vec::new()),
        &["cargo-machete", "cargo-dupes", "gitleaks"],
    );
    let results = super::run_with_facts(&facts);
    assertions::assert_exactness_summary(&results);
}
