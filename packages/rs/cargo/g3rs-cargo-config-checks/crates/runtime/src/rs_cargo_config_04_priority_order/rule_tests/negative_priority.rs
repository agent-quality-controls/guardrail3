use g3rs_cargo_config_checks_assertions::rs_cargo_config_04_priority_order::rule as assertions;
use super::helpers::run_check;

#[test]
fn warns_when_specific_lint_uses_negative_priority() {
    let results = run_check(
        include_str!("fixtures/golden_workspace.toml")
            .replace("unwrap_used = \"deny\"", "unwrap_used = { level = \"deny\", priority = -2 }")
            .as_str(),
    );

    assertions::assert_has_warn(&results, "specific lint `unwrap_used` has negative priority", false);
}
