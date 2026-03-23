use super::super::test_support::{copy_fixture, errors_by_id, run_family, write_file};

#[test]
fn phantom_workspace_member_is_error() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/devctl/Cargo.toml",
        "[workspace]\nmembers = [\n    \"crates/domain/types\",\n    \"crates/app/core\",\n    \"crates/ports/outbound/traits\",\n    \"crates/adapters/inbound/cli\",\n    \"crates/adapters/outbound/fs\",\n    \"crates/domain/phantom\",\n]\nresolver = \"2\"\n",
    );

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-09");
    assert_eq!(errors.len(), 1, "expected one phantom-member error: {errors:#?}");
    assert!(errors[0].title.contains("crates/domain/phantom"));
}
