use g3rs_cargo_config_checks_assertions::rs_cargo_config_03_workspace_metadata::rule as assertions;
use super::helpers::run_check;

#[test]
fn errors_when_edition_is_not_a_string() {
    let results = run_check(
        "[workspace]\nmembers = []\n\n[package]\nname = \"pkg\"\nedition.workspace = true\n",
    );
    assertions::assert_has_error(&results, "edition invalid", false);
}
