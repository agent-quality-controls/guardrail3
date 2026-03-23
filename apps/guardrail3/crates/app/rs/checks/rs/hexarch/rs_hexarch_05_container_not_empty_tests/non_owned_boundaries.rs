use super::super::super::test_support::{copy_fixture, errors_by_id, run_family, write_file};

#[test]
fn packages_lookalikes_stay_out_of_scope() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "packages/shared-types/app/README.md", "# stray");

    let results = run_family(tmp.path());
    let rule_05 = errors_by_id(&results, "RS-HEXARCH-05");
    assert!(
        rule_05.is_empty(),
        "packages lookalikes must not be owned by rule 05: {rule_05:#?}"
    );
}

#[test]
fn non_rust_apps_lookalikes_stay_out_of_scope() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "apps/admin/crates/app/README.md", "# stray");

    let results = run_family(tmp.path());
    let rule_05 = errors_by_id(&results, "RS-HEXARCH-05");
    assert!(
        rule_05.is_empty(),
        "non-Rust app lookalikes must not be owned by rule 05: {rule_05:#?}"
    );
}
