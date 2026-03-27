use guardrail3_app_rs_family_deny_assertions::rs_deny_02_allowed_locations as assertions;

use super::super::{collected_facts, forbidden_input, nested_member_shadow_tree};
use super::super::check;

#[test]
fn reports_every_forbidden_deny_config_filename_variant() {
    let mut results = Vec::new();

    for file_name in ["deny.toml", ".deny.toml", ".cargo/deny.toml"] {
        let facts = collected_facts(&nested_member_shadow_tree(file_name));
        let input = forbidden_input(&facts, &format!("workspace/crates/core/{file_name}"));
        check(&input, &mut results);
    }

    assert!(!results.is_empty());

    assertions::assert_findings(
        &results,
        &[
            assertions::error(
                "deny config at forbidden location",
                "`workspace/crates/core/.cargo/deny.toml` (.cargo/deny.toml) is at `workspace/crates/core` which is not an allowed deny policy root.",
                "workspace/crates/core/.cargo/deny.toml",
                false,
            ),
            assertions::error(
                "deny config at forbidden location",
                "`workspace/crates/core/.deny.toml` (.deny.toml) is at `workspace/crates/core` which is not an allowed deny policy root.",
                "workspace/crates/core/.deny.toml",
                false,
            ),
            assertions::error(
                "deny config at forbidden location",
                "`workspace/crates/core/deny.toml` (deny.toml) is at `workspace/crates/core` which is not an allowed deny policy root.",
                "workspace/crates/core/deny.toml",
                false,
            ),
        ],
    );
}
