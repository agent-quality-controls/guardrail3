use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::ContainerHexarchInput;
use super::inventory::push_success;

const ID: &str = "RS-HEXARCH-04";

pub fn check(input: &ContainerHexarchInput<'_>, results: &mut Vec<CheckResult>) {
    let bad_files: Vec<_> = input
        .files
        .iter()
        .filter(|file| file.as_str() != ".gitkeep")
        .cloned()
        .chain(input.symlink_files.iter().cloned())
        .collect();

    if bad_files.is_empty() {
        if input.dirs.is_empty() && !input.has_gitkeep {
            return;
        }
        push_success(
            results,
            ID,
            format!("Service `{}` has no loose files in {}", input.app_name, input.label),
            format!(
                "Service `{}` keeps structural/container directory `{}` free of loose files.",
                input.app_name, input.rel_path
            ),
            Some(input.rel_path.to_owned()),
        );
        return;
    }

    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Error,
        title: format!("Service `{}` has loose files in {}/", input.app_name, input.label),
        message: format!(
            "Service `{}` has files in `{}/` that don't belong: {}. Only `.gitkeep` is allowed in structural/container directories. Move code into module subdirectories.",
            input.app_name,
            input.label,
            bad_files.join(", ")
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
#[path = "rs_hexarch_04_loose_files_tests/mod.rs"]
mod rs_hexarch_04_loose_files_tests;
