use super::super::test_support::{copy_fixture, errors_by_id, run_family, write_file};

#[test]
fn package_app_cargo_is_error() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/devctl/Cargo.toml",
        "[package]\nname = \"devctl\"\nversion = \"0.1.0\"\n",
    );

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-08");
    assert_eq!(errors.len(), 1, "expected one non-workspace app error: {errors:#?}");
    assert!(errors[0].title.contains("must be a workspace"));
    assert_eq!(errors[0].file.as_deref(), Some("apps/devctl/Cargo.toml"));
}

#[test]
fn parse_error_is_error() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "apps/devctl/Cargo.toml", "[workspace");

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-08");
    assert_eq!(errors.len(), 1, "expected one parse error: {errors:#?}");
    assert!(errors[0].title.contains("parse error"));
}
