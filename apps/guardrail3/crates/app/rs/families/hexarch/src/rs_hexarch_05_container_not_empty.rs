use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::ContainerHexarchInput;

const ID: &str = "RS-HEXARCH-05";

pub fn check(input: &ContainerHexarchInput<'_>, results: &mut Vec<CheckResult>) {
    let has_real_dirs = input
        .dirs
        .iter()
        .any(|dir| !input.symlink_dirs.iter().any(|symlink| symlink == dir));
    if has_real_dirs || input.has_gitkeep {
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

    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Error,
        title: format!("Service `{}` empty container {}/", input.app_name, input.label),
        message: format!(
            "Service `{}` container `{}/` {detail}. Add module subdirectories or a `.gitkeep` if this layer is not needed yet.",
            input.app_name, input.label
        ),
        file: Some(input.rel_path.to_owned()),
        line: None,
        inventory: false,
    });
}

#[cfg(test)]
#[path = "rs_hexarch_05_container_not_empty_tests/mod.rs"]
mod tests;
