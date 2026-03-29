use guardrail3_app_rs_family_deny_assertions::rs_deny_09_ban_baseline_complete as assertions;

use super::super::{build_fixture_deny_toml, set_deny_ban_wrappers};

#[test]
fn errors_when_canonical_managed_wrappers_change() {
    let results = super::super::run_check(&set_deny_ban_wrappers(
        &build_fixture_deny_toml("service"),
        "regex",
        &["tree-sitter"],
    ));

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "managed ban wrappers changed",
            "`deny.toml` ban `regex` no longer matches the canonical managed entry.",
            "deny.toml",
            false,
        )],
    );
}
