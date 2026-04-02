use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::ContainerHexarchInput;
use crate::inventory::push_success;

const ID: &str = "RS-HEXARCH-05";

pub fn check(input: &ContainerHexarchInput<'_>, results: &mut Vec<CheckResult>) {
    let has_real_dirs = input
        .dirs
        .iter()
        .any(|dir| !input.symlink_dirs.iter().any(|symlink| symlink == dir));
    if has_real_dirs || input.has_gitkeep {
        push_success(
            results,
            ID,
            format!(
                "Service `{}` container {} is populated",
                input.app_name, input.label
            ),
            format!(
                "Service `{}` keeps container `{}` non-empty with subdirectories or `.gitkeep`.",
                input.app_name, input.rel_path
            ),
            Some(input.rel_path.to_owned()),
        );
        return;
    }

    let listed_files: Vec<_> = input
        .symlink_dirs
        .iter()
        .cloned()
        .chain(input.files.iter().cloned())
        .chain(input.symlink_files.iter().cloned())
        .collect();

    let detail = if listed_files.is_empty() {
        "is empty".to_owned()
    } else {
        format!(
            "contains files ({}) but no subdirectories",
            listed_files.join(", ")
        )
    };

    results.push(CheckResult::from_parts(
    ID.to_owned(),
    Severity::Error,
    format!("Service `{}` empty container {}/", input.app_name, input.label),
    format!(
            "Service `{}` container `{}/` {detail}. Add module subdirectories or a `.gitkeep` if this layer is not needed yet.",
            input.app_name, input.label
        ),
    Some(input.rel_path.to_owned()),
    None,
    false,
    ));
}

#[cfg(test)]
pub(crate) fn results_for_test_root(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_tree(&test_support::walk(root))
}
#[cfg(test)]

mod tests;
