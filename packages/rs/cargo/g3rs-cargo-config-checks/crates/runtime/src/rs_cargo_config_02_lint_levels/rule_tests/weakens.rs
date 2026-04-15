use g3rs_cargo_config_checks_assertions::rs_cargo_config_02_lint_levels::rule as assertions;
use super::helpers::run_check;

#[test]
fn errors_when_expected_deny_is_weakened() {
    let results = run_check(
        include_str!("fixtures/golden_workspace.toml")
            .replace("unwrap_used = \"deny\"", "unwrap_used = \"warn\"")
            .as_str(),
    );

    assertions::assert_has_error(&results, "lint `unwrap_used` weakens policy", false);
}
