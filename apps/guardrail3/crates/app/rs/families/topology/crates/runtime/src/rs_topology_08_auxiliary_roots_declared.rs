use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::AuxiliaryRootInput;

const ID: &str = "RS-TOPOLOGY-08";

pub fn check(input: &AuxiliaryRootInput<'_>, results: &mut Vec<CheckResult>) {
    results.push(
        CheckResult::from_parts(
            ID.to_owned(),
            Severity::Info,
            format!(
                "Rust root `{}` is declared auxiliary",
                display_dir(&input.root.rel_dir)
            ),
            format!(
                "`{}` is outside `apps/*` and `packages/*`, but is explicitly marked with `topology_role = \"auxiliary\"` in Cargo metadata.",
                input.root.cargo_rel_path
            ),
            Some(input.root.cargo_rel_path.clone()),
            None,
            false,
        )
        .as_inventory(),
    );
}

fn display_dir(rel_dir: &str) -> &str {
    if rel_dir.is_empty() { "." } else { rel_dir }
}

#[cfg(test)]
pub(crate) fn check_results(
    tree: &guardrail3_app_rs_family_view::FamilyView,
) -> Vec<guardrail3_domain_report::CheckResult> {
    crate::check_test_tree(tree)
}

#[cfg(test)]
#[path = "rs_topology_08_auxiliary_roots_declared_tests/mod.rs"]
mod rs_topology_08_auxiliary_roots_declared_tests;
