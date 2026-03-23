use super::super::test_support::{assert_no_error, copy_fixture, empty_dir, errors_by_id, run_family, write_file};

#[test]
fn golden_has_no_rule_04_errors() {
    let tmp = copy_fixture();
    let results = run_family(tmp.path());
    assert_no_error(&results, "RS-HEXARCH-04");
}

#[test]
fn loose_file_in_app_containers_hits_outer_and_inner_hex() {
    let tmp = copy_fixture();
    for path in [
        "apps/devctl/crates/app",
        "apps/backend/crates/app",
        "apps/worker/crates/app",
        "apps/backend/crates/adapters/inbound/mcp/crates/app",
    ] {
        write_file(tmp.path(), &format!("{path}/mod.rs"), "// stray");
    }

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-04");
    assert_eq!(errors.len(), 4, "expected one loose-file error per app root: {errors:#?}");
    for error in &errors {
        assert!(error.title.contains("loose files"));
        assert!(error.message.contains("mod.rs"));
    }
}

#[test]
fn files_only_container_is_owned_by_rule_05_not_rule_04() {
    let tmp = copy_fixture();
    empty_dir(tmp.path(), "apps/devctl/crates/domain");
    write_file(tmp.path(), "apps/devctl/crates/domain/README.md", "# stray");

    let results = run_family(tmp.path());
    let rule_04 = errors_by_id(&results, "RS-HEXARCH-04");
    let rule_05 = errors_by_id(&results, "RS-HEXARCH-05");
    assert!(rule_04.is_empty(), "rule 04 should not double-fire on files-only container: {rule_04:#?}");
    assert_eq!(rule_05.len(), 1, "expected rule 05 to own files-only container: {rule_05:#?}");
    assert!(rule_05[0].message.contains("README.md"));
}
