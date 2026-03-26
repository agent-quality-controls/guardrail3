use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::RootTestInput;

const ID: &str = "RS-TEST-11";

pub fn check(input: &RootTestInput<'_>, results: &mut Vec<CheckResult>) {
    if input.cargo_mutants_installed {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "cargo-mutants installed".to_owned(),
                message: "`cargo-mutants` is available on PATH.".to_owned(),
                file: Some(input.root.cargo_rel_path.clone()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    } else {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: "cargo-mutants missing".to_owned(),
            message: "`cargo-mutants` was not found on PATH for an active mutation-testing setup."
                .to_owned(),
            file: Some(input.root.cargo_rel_path.clone()),
            line: None,
            inventory: false,
        });
    }
}

#[cfg(test)]
#[allow(dead_code)]
pub(crate) fn run_family(root: &std::path::Path) -> Vec<CheckResult> {
    let tree = test_support::walk(root);
    super::check_test_tree(&tree, &test_support::StubToolChecker::default())
}

#[cfg(test)]
#[allow(dead_code)]
#[allow(dead_code)]
pub(crate) fn run_family_with_tool(
    root: &std::path::Path,
    cargo_mutants_installed: bool,
) -> Vec<CheckResult> {
    let tree = test_support::walk(root);
    let checker = if cargo_mutants_installed {
        test_support::StubToolChecker::with_tools(["cargo-mutants"])
    } else {
        test_support::StubToolChecker::default()
    };
    super::check_test_tree(&tree, &checker)
}

#[cfg(test)]
#[path = "rs_test_11_cargo_mutants_installed_tests/mod.rs"]
mod rs_test_11_cargo_mutants_installed_tests;