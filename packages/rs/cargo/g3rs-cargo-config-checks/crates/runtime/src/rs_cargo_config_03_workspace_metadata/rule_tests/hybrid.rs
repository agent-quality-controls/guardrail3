use super::helpers::run_check;
use g3rs_cargo_config_checks_assertions::rs_cargo_config_03_workspace_metadata::rule as assertions;

#[test]
fn inventories_when_hybrid_root_falls_back_to_package_edition() {
    let results = run_check(
        "[workspace]\nmembers = []\nresolver = \"3\"\n\n[package]\nname = \"hybrid\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    assertions::assert_has_info(&results, "edition policy satisfied", true);
}
