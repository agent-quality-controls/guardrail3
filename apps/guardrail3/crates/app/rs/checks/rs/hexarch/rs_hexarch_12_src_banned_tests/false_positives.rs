use super::super::super::test_support::{copy_fixture, errors_by_id, run_family, write_file};

#[test]
fn src_file_not_directory_is_not_error() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "apps/devctl/src", "not a directory");

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-12");
    assert!(
        errors.is_empty(),
        "file named src should not trigger rule 12: {errors:#?}"
    );
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
    assert!(
        errors.is_empty(),
        "inner hex src should not trigger rule 12: {errors:#?}"
    );
}
