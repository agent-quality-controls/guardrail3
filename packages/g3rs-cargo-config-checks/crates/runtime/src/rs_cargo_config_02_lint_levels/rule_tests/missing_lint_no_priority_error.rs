use super::helpers::run_check;

#[test]
fn no_priority_error_when_group_lint_is_missing() {
    // Remove the `all` group lint entirely. Check 01 should flag it as missing,
    // but check 02 should NOT produce a "wrong priority" error for a nonexistent lint.
    let results = run_check(
        include_str!("../../rs_cargo_config_01_workspace_lints/rule_tests/fixtures/golden_workspace.toml")
            .replace("all = { level = \"deny\", priority = -1 }\n", "")
            .as_str(),
    );

    let priority_errors: Vec<_> = results
        .iter()
        .filter(|result| result.id() == "RS-CARGO-CONFIG-02" && result.title().contains("wrong priority"))
        .collect();

    assert!(
        priority_errors.is_empty(),
        "check 02 should not report wrong priority for a lint that is completely missing; \
         check 01 handles missing lints. Got: {priority_errors:?}"
    );
}
