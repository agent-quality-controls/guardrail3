use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::AuxiliaryRootInput;

const ID: &str = "RS-ARCH-08";

pub fn check(input: &AuxiliaryRootInput<'_>, results: &mut Vec<CheckResult>) {
    results.push(
        CheckResult {
            id: ID.to_owned(),
            severity: Severity::Info,
            title: format!(
                "Rust root `{}` is declared auxiliary",
                display_dir(&input.root.rel_dir)
            ),
            message: format!(
                "`{}` is outside `apps/*` and `packages/*`, but is explicitly marked with `arch_role = \"auxiliary\"` in Cargo metadata.",
                input.root.cargo_rel_path
            ),
            file: Some(input.root.cargo_rel_path.clone()),
            line: None,
            inventory: false,
        }
        .as_inventory(),
    );
}

fn display_dir(rel_dir: &str) -> &str {
    if rel_dir.is_empty() { "." } else { rel_dir }
}

#[cfg(test)]
pub(crate) fn check_results(
    tree: &guardrail3_domain_project_tree::ProjectTree,
) -> Vec<guardrail3_domain_report::CheckResult> {
    crate::check_test_tree(tree)
}

#[cfg(test)]
#[path = "rs_arch_08_auxiliary_roots_declared_tests/mod.rs"]
mod rs_arch_08_auxiliary_roots_declared_tests;
