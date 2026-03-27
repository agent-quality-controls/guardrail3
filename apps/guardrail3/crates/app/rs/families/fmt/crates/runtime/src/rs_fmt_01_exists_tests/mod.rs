use guardrail3_app_rs_family_fmt_assertions::rs_fmt_01_exists as assertions;

use super::run_check;

#[test]
fn inventories_when_root_rustfmt_config_exists() {
    let results = run_check(Some("rustfmt.toml"));

    assertions::assert_no_findings(&results);
}

#[test]
fn errors_when_root_rustfmt_config_is_missing() {
    let results = run_check(None);

    assertions::assert_missing_root_config(&results);
}
