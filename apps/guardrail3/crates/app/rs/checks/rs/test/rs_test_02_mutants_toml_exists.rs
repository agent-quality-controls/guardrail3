use crate::domain::report::{CheckResult, Severity};

use super::inputs::RootTestInput;

const ID: &str = "RS-TEST-02";

pub fn check(input: &RootTestInput<'_>, results: &mut Vec<CheckResult>) {
    if input.root.mutants_exists {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "mutants config exists".to_owned(),
                message: format!("Found `{}`.", input.root.mutants_rel_path),
                file: Some(input.root.mutants_rel_path.clone()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    } else {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: "mutants config missing".to_owned(),
            message: format!(
                "{} `{}` is missing `.cargo/mutants.toml`.",
                input.root.kind.label(),
                display_root(&input.root.rel_dir)
            ),
            file: Some(input.root.mutants_rel_path.clone()),
            line: None,
            inventory: false,
        });
    }
}

fn display_root(rel_dir: &str) -> String {
    if rel_dir.is_empty() {
        "project root".to_owned()
    } else {
        format!("`{rel_dir}`")
    }
}

#[cfg(test)]
#[path = "rs_test_02_mutants_toml_exists_tests.rs"]
mod tests;
