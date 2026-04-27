use super::helpers::run_check;
use g3rs_cargo_config_checks_assertions::lint_levels::rule as assertions;

#[test]
fn errors_when_required_allow_lint_is_set_to_deny() {
    let results = run_check(
        include_str!("fixtures/golden_workspace.toml")
            .replace(
                "redundant_pub_crate = \"allow\"",
                "redundant_pub_crate = \"deny\"",
            )
            .as_str(),
    );

    assertions::assert_has_error(&results, "lint `redundant_pub_crate` must be allow", false);
}

#[test]
fn passes_when_required_allow_lint_is_correctly_set() {
    let results = run_check(include_str!("fixtures/golden_workspace.toml"));

    assertions::assert_title_absent(&results, "lint `redundant_pub_crate` must be allow");
}
