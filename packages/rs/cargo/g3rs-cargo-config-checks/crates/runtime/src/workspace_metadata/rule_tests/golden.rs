use super::helpers::run_check;
use g3rs_cargo_config_checks_assertions::workspace_metadata::rule as assertions;

#[test]
fn inventories_when_edition_is_supported() {
    let results = run_check("[package]\nname = \"pkg\"\nedition = \"2024\"\n");
    assertions::assert_has_info(&results, "edition policy satisfied", true);
}
