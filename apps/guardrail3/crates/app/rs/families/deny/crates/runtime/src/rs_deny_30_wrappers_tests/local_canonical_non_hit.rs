use guardrail3_app_rs_family_deny_assertions::rs_deny_30_wrappers as assertions;

use super::super::{build_fixture_deny_toml, copy_fixture, write_file};

#[test]
fn local_canonical_wrapper_baseline_does_not_false_positive() {
    let tmp = copy_fixture("../../../../../../../tests/fixtures/r_arch_01/golden");
    write_file(tmp.path(), "deny.toml", &build_fixture_deny_toml("service"));
    write_file(
        tmp.path(),
        "apps/devctl/deny.toml",
        &build_fixture_deny_toml("service"),
    );

    let results = super::super::run_family(tmp.path());
    assert!(!results.is_empty());
    assertions::assert_no_findings(&results);
}
