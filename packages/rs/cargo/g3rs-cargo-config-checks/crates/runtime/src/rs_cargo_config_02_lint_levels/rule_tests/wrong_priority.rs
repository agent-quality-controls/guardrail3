use super::helpers::run_check;
use g3rs_cargo_config_checks_assertions::rs_cargo_config_02_lint_levels::rule as assertions;

#[test]
fn errors_when_group_priority_is_wrong() {
    let results = run_check(
        include_str!("fixtures/golden_workspace.toml")
            .replace(
                "all = { level = \"deny\", priority = -1 }",
                "all = { level = \"deny\", priority = 0 }",
            )
            .as_str(),
    );

    assertions::assert_has_error(&results, "lint `all` has wrong priority", false);
}
