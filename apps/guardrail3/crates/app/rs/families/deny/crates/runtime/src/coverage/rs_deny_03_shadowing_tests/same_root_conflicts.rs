use guardrail3_app_rs_family_deny_assertions::rs_deny_03_shadowing as assertions;

use super::super::check_same_root_conflict;
use super::super::{collected_facts, same_root_conflict_input, same_root_conflict_tree};

#[test]
fn errors_on_same_root_precedence_conflict() {
    let facts = collected_facts(&same_root_conflict_tree());
    let input = same_root_conflict_input(&facts, "");
    let mut results = Vec::new();

    check_same_root_conflict(&input, &mut results);

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "multiple deny configs at one policy root",
            "`.` has multiple accepted deny configs: .cargo/deny.toml, .deny.toml, deny.toml.",
            ".cargo/deny.toml",
            false,
        )],
    );
}
