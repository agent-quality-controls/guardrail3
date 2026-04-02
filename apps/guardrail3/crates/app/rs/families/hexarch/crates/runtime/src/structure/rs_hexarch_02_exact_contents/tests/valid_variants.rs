use super::{copy_fixture, remove_dir, write_file};
use guardrail3_app_rs_family_hexarch_assertions::structure::rs_hexarch_02_exact_contents as assertions;

#[test]
fn crates_with_only_gitkeep_still_defer_to_missing_required_dirs_not_rule_01() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/devctl/crates");
    write_file(tmp.path(), "apps/devctl/crates/.gitkeep", "");

    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "RS-HEXARCH-01");

    assertions::assert_error_count_matching_file(
        &results,
        "",
        "apps/devctl/crates",
        4,
        &[],
        &[],
        &[],
        &[],
    );
}

#[test]
fn optional_macros_dir_is_allowed() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "apps/devctl/crates/macros/.gitkeep", "");

    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "");
}
