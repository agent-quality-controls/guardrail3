use guardrail3_app_rs_family_deny_assertions::rs_deny_30_wrappers as assertions;

use super::super::{copy_fixture, set_deny_ban_wrappers, write_file, build_fixture_deny_toml};

#[test]
fn local_wrapper_drift_only_reports_for_the_owned_library_root() {
    let tmp = copy_fixture("../../../../../../../tests/fixtures/r_arch_01/golden");
    write_file(
        tmp.path(),
        "deny.toml",
        &build_fixture_deny_toml("service"),
    );
    write_file(
        tmp.path(),
        "apps/devctl/deny.toml",
        &set_deny_ban_wrappers(
            &build_fixture_deny_toml("service"),
            "regex",
            &["tree-sitter"],
        ),
    );

    let results = super::super::run_family(tmp.path());
    assert!(!results.is_empty());
    assertions::assert_findings(
        &results,
        &[assertions::error(
            "managed ban wrappers changed",
            "`apps/devctl/deny.toml` ban `regex` must keep wrappers `globset, ignore, tree-sitter`.",
            "apps/devctl/deny.toml",
            false,
        )],
    );
}
