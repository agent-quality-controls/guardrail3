use guardrail3_app_rs_family_deny_assertions::rs_deny_30_wrappers as assertions;

use super::super::{build_fixture_deny_toml, set_deny_ban_wrappers};

#[test]
fn local_wrapper_drift_only_reports_for_the_owned_library_root() {
    let results = super::super::run_check(&set_deny_ban_wrappers(
        &build_fixture_deny_toml("service"),
        "regex",
        &["tree-sitter"],
    ));
    assert!(!results.is_empty());
    assertions::assert_findings(
        &results,
        &[assertions::error(
            "managed ban wrappers changed",
            "`deny.toml` ban `regex` must keep wrappers `globset, ignore, tree-sitter`.",
            "deny.toml",
            false,
        )],
    );
}
