use super::super::test_support::{copy_fixture, create_dir, errors_by_id, run_family, write_file};

#[test]
fn app_level_src_dir_is_error() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "apps/devctl/src/main.rs", "fn main() {}");

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-12");
    assert_eq!(errors.len(), 1, "expected one app-level src error: {errors:#?}");
    assert!(errors[0].title.contains("has src/ directory"));
    assert_eq!(errors[0].file.as_deref(), Some("apps/devctl/src"));
}

#[test]
fn empty_src_dir_is_still_error() {
    let tmp = copy_fixture();
    create_dir(tmp.path(), "apps/devctl/src");

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-12");
    assert_eq!(errors.len(), 1, "expected one empty-src error: {errors:#?}");
}

#[test]
fn src_file_not_directory_is_not_error() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "apps/devctl/src", "not a directory");

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-12");
    assert!(errors.is_empty(), "file named src should not trigger rule 12: {errors:#?}");
}

#[test]
fn inner_hex_src_does_not_trigger_app_level_rule() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/crates/adapters/inbound/mcp/src/main.rs",
        "fn main() {}",
    );

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-12");
    assert!(errors.is_empty(), "inner hex src should not trigger rule 12: {errors:#?}");
}
