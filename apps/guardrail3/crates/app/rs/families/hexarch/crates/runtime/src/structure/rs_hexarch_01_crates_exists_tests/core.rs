use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_01_crates_exists as assertions;

use super::super::check_with_top_level_entries_for_tests;

#[test]
fn passes_when_top_level_crates_dir_has_entries() {
    let results = check_with_top_level_entries_for_tests(1);

    assertions::assert_no_error(&results, "");
}

#[test]
fn fails_when_top_level_crates_dir_has_no_entries() {
    let results = check_with_top_level_entries_for_tests(0);

    assertions::assert_error_summary(
        &results,
        "",
        1,
        ["apps/backend"],
        Some(Some("apps/backend")),
        Some(&["missing crates/"]),
        None,
        None,
    );
}
