use super::{copy_fixture, write_file};
use guardrail3_app_rs_family_hexarch_assertions::workspace_policy::rs_hexarch_11_root_workspace_doesnt_include_apps as assertions;

#[test]
fn malformed_root_cargo_is_error() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "Cargo.toml", "[workspace");

    let results = super::run_family(tmp.path());
    assertions::assert_error_title_contains(&results, "", 1, &["Cargo.toml"], &["parse error"]);
}
