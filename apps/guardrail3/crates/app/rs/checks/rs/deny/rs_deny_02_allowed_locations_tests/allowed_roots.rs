use crate::domain::modules::deny::build_deny_toml;

use super::super::super::test_support::{copy_fixture, run_family, write_file};

#[test]
fn does_not_flag_deny_configs_at_allowed_validation_and_workspace_roots() {
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
    let forbidden = results
        .iter()
        .filter(|result| result.id == "RS-DENY-02")
        .collect::<Vec<_>>();

    assert!(forbidden.is_empty(), "{forbidden:#?}");
}
