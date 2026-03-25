use guardrail3_domain_modules::deny::build_deny_toml;

use super::super::super::test_support::{copy_fixture, run_family, write_file};

#[test]
fn local_canonical_wrapper_baseline_does_not_false_positive() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "deny.toml",
        &build_deny_toml("service", "", "", ""),
    );
    write_file(
        tmp.path(),
        "apps/devctl/deny.toml",
        &build_deny_toml("service", "", "", ""),
    );

    let results = run_family(tmp.path());
    let wrapper_results = results
        .iter()
        .filter(|result| result.id == "RS-DENY-30")
        .collect::<Vec<_>>();

    assert!(wrapper_results.is_empty(), "{wrapper_results:#?}");
}
