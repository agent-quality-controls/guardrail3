use g3rs_cargo_config_checks_assertions::rs_cargo_config_02_lint_levels::rule as assertions;
use super::helpers::run_check;

#[test]
fn no_priority_error_when_group_lint_is_missing() {
    // Remove the `all` group lint entirely. Check 01 should flag it as missing,
    // but check 02 should NOT produce a "wrong priority" error for a nonexistent lint.
    let results = run_check(
        include_str!("fixtures/golden_workspace.toml")
            .replace("all = { level = \"deny\", priority = -1 }\n", "")
            .as_str(),
    );

    assertions::assert_title_absent(&results, "lint `all` has wrong priority");
}
