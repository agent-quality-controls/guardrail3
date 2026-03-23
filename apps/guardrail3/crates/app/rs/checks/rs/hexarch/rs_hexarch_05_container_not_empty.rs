use crate::domain::report::{CheckResult, Severity};

use super::inputs::ContainerHexarchInput;

const ID: &str = "RS-HEXARCH-05";

pub fn check(input: &ContainerHexarchInput<'_>, results: &mut Vec<CheckResult>) {
    if !input.dirs.is_empty() || input.has_gitkeep {
        return;
    }

    let detail = if input.files.is_empty() {
        "is empty".to_owned()
    } else {
        format!("contains files ({}) but no subdirectories", input.files.join(", "))
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
#[path = "rs_hexarch_05_container_not_empty_tests.rs"]
mod tests;
