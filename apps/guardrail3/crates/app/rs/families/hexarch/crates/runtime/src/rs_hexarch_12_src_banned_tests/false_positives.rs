use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_12_src_banned as assertions;
use test_support::{copy_fixture, write_file};

#[test]
fn golden_fixture_has_no_rule_12_errors() {
    let tmp = copy_fixture();

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-12");
    assert!(
        errors.is_empty(),
        "golden fixture should stay clean for rule 12: {errors:#?}"
    );
}

#[test]
fn src_file_not_directory_is_not_error() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "apps/devctl/src", "not a directory");

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-12");
    assert!(
        errors.is_empty(),
        "file named src should not trigger rule 12: {errors:#?}"
    );
}

#[test]
fn nested_inner_crate_src_does_not_trigger_app_level_rule() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/crates/adapters/inbound/mcp/crates/adapters/inbound/transport/src/extra.rs",
        "pub fn extra() {}",
    );

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-12");
    assert!(
        errors.is_empty(),
        "nested inner crate src should not trigger rule 12: {errors:#?}"
    );
}

#[test]
fn ts_apps_with_src_do_not_trigger_rule_12() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/admin/src/guard.ts",
        "export const guard = true;\n",
    );
    write_file(
        tmp.path(),
        "apps/landing/src/page.tsx",
        "export default function Page() { return null; }\n",
    );
    write_file(
        tmp.path(),
        "apps/portal/src/feature.ts",
        "export const feature = true;\n",
    );

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-12");
    assert!(
        errors.is_empty(),
        "ts apps should stay out of rule 12: {errors:#?}"
    );
}
