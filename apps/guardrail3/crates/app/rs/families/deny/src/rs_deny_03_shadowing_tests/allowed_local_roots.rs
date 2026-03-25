use guardrail3_domain_modules::deny::build_deny_toml;

use super::super::super::test_support::{copy_fixture, run_family, write_file};

#[test]
fn does_not_treat_allowed_local_policy_roots_as_shadowing() {
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
    let shadowing = results
        .iter()
        .filter(|result| result.id == "RS-DENY-03")
        .collect::<Vec<_>>();

    assert!(shadowing.is_empty(), "{shadowing:#?}");
}
