use super::super::test_support::{copy_fixture, errors_by_id, run_family, write_file};

#[test]
fn root_workspace_including_app_is_error() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "Cargo.toml",
        "[workspace]\nmembers = [\"packages/shared-types\", \"apps/devctl\"]\nresolver = \"2\"\n",
    );

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-11");
    assert_eq!(errors.len(), 1, "expected one root-workspace app inclusion error: {errors:#?}");
    assert!(errors[0].title.contains("apps/devctl"));
    assert_eq!(errors[0].file.as_deref(), Some("Cargo.toml"));
}

#[test]
fn malformed_root_cargo_is_error() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "Cargo.toml", "[workspace");

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-11");
    assert_eq!(errors.len(), 1, "expected one root parse error: {errors:#?}");
    assert!(errors[0].title.contains("parse error"));
}
