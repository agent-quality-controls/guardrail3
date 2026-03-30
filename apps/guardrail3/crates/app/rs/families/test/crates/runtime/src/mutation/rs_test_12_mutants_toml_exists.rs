use crate::{CheckResult, Severity};

use super::inputs::RootTestInput;

const ID: &str = "RS-TEST-12";

pub fn check(input: &RootTestInput<'_>, results: &mut Vec<CheckResult>) {
    if input.root.mutants_exists && input.root.mutants_parsed.is_some() {
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "mutants config exists".to_owned(),
                format!("Found `{}`.", input.root.mutants_rel_path),
                Some(input.root.mutants_rel_path.clone()),
                None,
                false,
            )
            .as_inventory(),
        );
    } else if !input.root.mutants_exists {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "mutants config missing".to_owned(),
            format!(
                "{} is missing required mutation config `{}`.",
                display_root(&input.root.rel_dir),
                input.root.mutants_rel_path
            ),
            Some(input.root.mutants_rel_path.clone()),
            None,
            false,
        ));
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
pub(crate) fn run_family(root: &std::path::Path) -> Vec<CheckResult> {
    let tree = test_support::walk(root);
    super::check_test_tree(&tree, &test_support::StubToolChecker::default())
}
#[cfg(test)]
#[path = "rs_test_12_mutants_toml_exists_tests/mod.rs"]
mod rs_test_12_mutants_toml_exists_tests;
