use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_05_container_not_empty as assertions;
use super::{copy_fixture, write_file};

#[test]
fn packages_lookalikes_stay_out_of_scope() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "packages/shared-types/app/README.md", "# stray");

    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "");
}

#[test]
fn non_rust_apps_lookalikes_stay_out_of_scope() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "apps/admin/crates/app/README.md", "# stray");

    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "");
}
