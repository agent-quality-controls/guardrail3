use super::{copy_fixture, write_file};
use guardrail3_app_rs_family_hexarch_assertions::structure::rs_hexarch_02_exact_contents as assertions;

#[test]
fn packages_invalid_crates_shape_is_not_owned_by_rule_02() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "packages/phantom/crates/misc/.gitkeep", "");
    write_file(tmp.path(), "packages/phantom/crates/mod.rs", "// stray");

    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "");
}

#[test]
fn stray_app_without_cargo_toml_is_not_owned_by_rule_02() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "apps/fake-service/crates/misc/.gitkeep", "");
    write_file(tmp.path(), "apps/fake-service/crates/mod.rs", "// stray");

    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "");
}

#[test]
fn newly_discovered_rust_app_with_partial_crates_is_owned_by_rule_02() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/scheduler/Cargo.toml",
        "[workspace]\nmembers = []\nresolver = \"2\"\n",
    );
    write_file(tmp.path(), "apps/scheduler/crates/domain/.gitkeep", "");

    let results = super::run_family(tmp.path());
    assertions::assert_error_count_matching_file(
        &results,
        "",
        "apps/scheduler/crates",
        3,
        &[],
        &[],
        &[],
        &[],
    );
    assertions::assert_error_count_matching_file(
        &results,
        "",
        "apps/scheduler/crates",
        1,
        &["adapters/"],
        &[],
        &[],
        &[],
    );
    assertions::assert_error_count_matching_file(
        &results,
        "",
        "apps/scheduler/crates",
        1,
        &["app/"],
        &[],
        &[],
        &[],
    );
    assertions::assert_error_count_matching_file(
        &results,
        "",
        "apps/scheduler/crates",
        1,
        &["ports/"],
        &[],
        &[],
        &[],
    );
}

#[test]
fn non_owned_nested_looking_shape_under_packages_is_still_out_of_scope() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "packages/lookalike/crates/adapters/inbound/mcp/crates/utils/.gitkeep",
        "",
    );
    write_file(
        tmp.path(),
        "packages/lookalike/crates/adapters/inbound/mcp/crates/mod.rs",
        "// stray",
    );

    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "");
}
