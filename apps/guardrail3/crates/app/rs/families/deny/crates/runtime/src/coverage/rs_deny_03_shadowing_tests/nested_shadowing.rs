use guardrail3_app_rs_family_deny_assertions::rs_deny_03_shadowing as assertions;

use super::super::check_forbidden;
use super::super::{collected_facts, forbidden_input, nested_member_shadow_tree};

#[test]
fn reports_nested_shadowing_for_every_deny_filename_variant() {
    let mut results = Vec::new();

    for file_name in ["deny.toml", ".deny.toml", ".cargo/deny.toml"] {
        let facts = collected_facts(&nested_member_shadow_tree(file_name));
        let input = forbidden_input(&facts, &format!("workspace/crates/core/{file_name}"));
        check_forbidden(&input, &mut results);
    }

    assert!(!results.is_empty());

    assertions::assert_findings(
        &results,
        &[
            assertions::error(
                "nested deny config shadows parent policy",
                "`workspace/crates/core/.cargo/deny.toml` shadows deny policy rooted at `workspace`.",
                "workspace/crates/core/.cargo/deny.toml",
                false,
            ),
            assertions::error(
                "nested deny config shadows parent policy",
                "`workspace/crates/core/.deny.toml` shadows deny policy rooted at `workspace`.",
                "workspace/crates/core/.deny.toml",
                false,
            ),
            assertions::error(
                "nested deny config shadows parent policy",
                "`workspace/crates/core/deny.toml` shadows deny policy rooted at `workspace`.",
                "workspace/crates/core/deny.toml",
                false,
            ),
        ],
    );
}
