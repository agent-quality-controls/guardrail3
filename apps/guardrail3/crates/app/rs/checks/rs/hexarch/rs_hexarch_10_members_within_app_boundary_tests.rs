use super::super::test_support::{copy_fixture, errors_by_id, run_family, write_file};

#[test]
fn workspace_member_outside_app_boundary_is_error() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/devctl/Cargo.toml",
        "[workspace]\nmembers = [\n    \"crates/domain/types\",\n    \"crates/app/core\",\n    \"crates/ports/outbound/traits\",\n    \"crates/adapters/inbound/cli\",\n    \"crates/adapters/outbound/fs\",\n    \"../../packages/shared-types\",\n]\nresolver = \"2\"\n",
    );

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-10");
    assert_eq!(errors.len(), 1, "expected one outside-boundary error: {errors:#?}");
    assert!(errors[0].title.contains("../../packages/shared-types"));
}
