use g3rs_hooks_file_tree_checks_assertions::modular_scripts_executable::rule as assertions;

use super::super::check;
use super::helpers::script;

#[test]
fn reports_non_executable_modular_script() {
    let mut results = Vec::new();
    check(
        &[script(
            ".githooks/pre-commit.d/10-rust.sh",
            1,
            16,
            Some(false),
        )],
        &mut results,
    );

    assertions::assert_contains(
        &results,
        assertions::finding(
            assertions::Severity::Warn,
            "modular hook script is not executable",
            "Modular hook script exists but does not have the executable bit set.",
            Some(".githooks/pre-commit.d/10-rust.sh"),
            false,
        ),
    );
}

#[test]
fn inventories_executable_modular_script() {
    let mut results = Vec::new();
    check(
        &[script(
            ".githooks/pre-commit.d/10-rust.sh",
            1,
            16,
            Some(true),
        )],
        &mut results,
    );

    assertions::assert_contains(
        &results,
        assertions::finding(
            assertions::Severity::Warn,
            "modular hook script is executable",
            "Modular hook script has the executable bit set.",
            Some(".githooks/pre-commit.d/10-rust.sh"),
            true,
        ),
    );
}
