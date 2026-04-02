use crate::{CheckResult, Severity};

use crate::inputs::RootTestInput;

const ID: &str = "RS-TEST-11";

pub fn check(input: &RootTestInput<'_>, results: &mut Vec<CheckResult>) {
    if input.cargo_mutants_installed {
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "cargo-mutants installed".to_owned(),
                "`cargo-mutants` is available on PATH.".to_owned(),
                Some(input.root.cargo_rel_path.clone()),
                None,
                false,
            )
            .as_inventory(),
        );
    } else {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "cargo-mutants missing".to_owned(),
            "`cargo-mutants` was not found on PATH for an active mutation-testing setup."
                .to_owned(),
            Some(input.root.cargo_rel_path.clone()),
            None,
            false,
        ));
    }
}

#[cfg(test)]
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
    crate::check_test_tree(&tree, &checker)
}

#[cfg(test)]
#[path = "rs_test_11_cargo_mutants_installed_tests/mod.rs"]
mod rs_test_11_cargo_mutants_installed_tests;
