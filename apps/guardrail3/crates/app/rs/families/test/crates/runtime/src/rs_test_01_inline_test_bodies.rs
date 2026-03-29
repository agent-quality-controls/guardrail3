use crate::{CheckResult, Severity};

use super::inputs::CfgTestModuleInput;

const ID: &str = "RS-TEST-01";

pub fn check(input: &CfgTestModuleInput<'_>, results: &mut Vec<CheckResult>) {
    if !input.module.has_body {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "inline cfg(test) body absent".to_owned(),
                message: "Owned `#[cfg(test)]` declarations stay as sidecar modules instead of inline bodies.".to_owned(),
                file: Some(input.file.rel_path.clone()),
                line: Some(input.module.line),
                inventory: false,
            }
            .as_inventory(),
        );
        return;
    }

    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Error,
        title: "inline cfg(test) body in src".to_owned(),
        message:
            "Production `src/` files must not contain inline `#[cfg(test)] mod ... { ... }` bodies."
                .to_owned(),
        file: Some(input.file.rel_path.clone()),
        line: Some(input.module.line),
        inventory: false,
    });
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
#[path = "rs_test_01_inline_test_bodies_tests/mod.rs"]
mod rs_test_01_inline_test_bodies_tests;
