use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::LeafHexarchInput;
use super::inventory::push_success;

const ID: &str = "RS-HEXARCH-06";

pub fn check(input: &LeafHexarchInput<'_>, results: &mut Vec<CheckResult>) {
    if input.has_cargo && input.has_crates_dir {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: format!(
                "Service `{}` subdirectory {}/ has both Cargo.toml and crates/",
                input.app_name, input.label
            ),
            message: format!(
                "Service `{}` has `{}/` with both `Cargo.toml` and `crates/`. A subdirectory must be either a crate or a hex-in-hex, not both.",
                input.app_name, input.label
            ),
            file: Some(input.rel_path.to_owned()),
            line: None,
            inventory: false,
        });
        return;
    }

    if input.has_cargo || input.has_crates_dir || input.gitkeep_only {
        push_success(
            results,
            ID,
            format!("Service `{}` leaf {} has valid ownership shape", input.app_name, input.label),
            format!(
                "Service `{}` keeps leaf `{}` as a crate, nested hex root, or placeholder.",
                input.app_name, input.rel_path
            ),
            Some(input.rel_path.to_owned()),
        );
        return;
    }

    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Error,
        title: format!(
            "Service `{}` subdirectory {}/ missing Cargo.toml",
            input.app_name, input.label
        ),
        message: format!(
            "Service `{}` has `{}/` but it has no `Cargo.toml` and no `crates/` directory. Every subdirectory in a container folder must be its own crate, a hex-in-hex with its own `crates/` structure, or a placeholder with `.gitkeep`.",
            input.app_name, input.label
        ),
        file: Some(input.rel_path.to_owned()),
        line: None,
        inventory: false,
    });
}

#[cfg(test)]
#[allow(dead_code)]
pub(super) fn results_for_test_root(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_tree(&test_support::walk(root))
}

#[cfg(test)]
#[allow(dead_code)]
pub(super) fn results_for_test_tree(
    tree: &guardrail3_domain_project_tree::ProjectTree,
) -> Vec<CheckResult> {
    crate::check_test_tree(tree)
}

#[cfg(test)]
#[path = "rs_hexarch_06_leaf_valid_tests/mod.rs"]
mod rs_hexarch_06_leaf_valid_tests;
