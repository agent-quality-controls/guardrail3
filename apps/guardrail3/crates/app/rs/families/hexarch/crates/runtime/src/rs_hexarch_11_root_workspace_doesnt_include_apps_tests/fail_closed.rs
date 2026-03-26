use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_11_root_workspace_doesnt_include_apps as assertions;
use super::{copy_fixture, write_file};

#[test]
fn malformed_root_cargo_is_error() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "Cargo.toml", "[workspace");

    let results = super::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "");
    assert_eq!(
        errors.len(),
        1,
        "expected one root parse error: {errors:#?}"
    );
    assert!(errors[0].title.contains("parse error"));
}
