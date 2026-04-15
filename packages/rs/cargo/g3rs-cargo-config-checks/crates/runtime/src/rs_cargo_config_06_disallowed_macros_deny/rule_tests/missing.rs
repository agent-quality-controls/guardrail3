use g3rs_cargo_config_checks_assertions::rs_cargo_config_06_disallowed_macros_deny::rule as assertions;
use super::helpers::run_check;

#[test]
fn errors_when_disallowed_macros_is_missing() {
    let results = run_check(
        include_str!("fixtures/golden_workspace.toml")
            .replace("disallowed_macros = \"deny\"\n", "")
            .as_str(),
    );

    assertions::assert_has_error(&results, "disallowed macros lint missing", false);
}
