use super::helpers::run_check;
use g3rs_cargo_config_checks_assertions::rs_cargo_config_03_workspace_metadata::rule as assertions;

#[test]
fn errors_when_edition_is_older_than_minimum() {
    let results = run_check("[package]\nname = \"pkg\"\nedition = \"2018\"\n");
    assertions::assert_has_error(&results, "edition below minimum", false);
}
