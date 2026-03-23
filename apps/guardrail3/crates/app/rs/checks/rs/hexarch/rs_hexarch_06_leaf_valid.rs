use crate::domain::report::{CheckResult, Severity};

use super::inputs::LeafHexarchInput;

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
#[path = "rs_hexarch_06_leaf_valid_tests/mod.rs"]
mod tests;
