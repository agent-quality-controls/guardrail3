use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_05_container_not_empty as assertions;
use crate::test_support::{copy_fixture, write_file};

#[test]
fn packages_lookalikes_stay_out_of_scope() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "packages/shared-types/app/README.md", "# stray");

    let results = assertions::run_family(tmp.path());
    let rule_05 = assertions::errors_by_id(&results, "RS-HEXARCH-05");
    assert!(
        rule_05.is_empty(),
        "packages lookalikes must not be owned by rule 05: {rule_05:#?}"
    );
}

#[test]
fn non_rust_apps_lookalikes_stay_out_of_scope() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "apps/admin/crates/app/README.md", "# stray");

    let results = assertions::run_family(tmp.path());
    let rule_05 = assertions::errors_by_id(&results, "RS-HEXARCH-05");
    assert!(
        rule_05.is_empty(),
        "non-Rust app lookalikes must not be owned by rule 05: {rule_05:#?}"
    );
}
