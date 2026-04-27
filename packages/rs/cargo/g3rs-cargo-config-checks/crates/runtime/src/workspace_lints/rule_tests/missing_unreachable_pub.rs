use super::helpers::run_check;
use g3rs_cargo_config_checks_assertions::workspace_lints::rule as assertions;

#[test]
fn errors_when_library_only_rust_lint_is_missing() {
    let results = run_check(
        include_str!("fixtures/golden_workspace.toml")
            .replace("unreachable_pub = \"deny\"\n", "")
            .as_str(),
    );

    assertions::assert_has_error(&results, "missing rust lint `unreachable_pub`", false);
}
