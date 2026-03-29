use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::MisplacedRootInput;

const ID: &str = "RS-ARCH-02";

pub fn check(input: &MisplacedRootInput<'_>, results: &mut Vec<CheckResult>) {
    if !input.reporting_enabled {
        return;
    }

    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Error,
        title: format!(
            "Rust root `{}` is misplaced outside architecture zones",
            display_dir(&input.root.rel_dir)
        ),
        message: format!(
            "`{}` lives outside any `apps/*` or `packages/*` zone while Rust architecture enforcement is active.",
            input.root.cargo_rel_path
        ),
        file: Some(input.root.cargo_rel_path.clone()),
        line: None,
        inventory: false,
    });
}

pub fn check_success(
    reporting_enabled: bool,
    has_misplaced_roots: bool,
    results: &mut Vec<CheckResult>,
) {
    if !reporting_enabled || has_misplaced_roots {
        return;
    }

    results.push(
        CheckResult {
            id: ID.to_owned(),
            severity: Severity::Info,
            title: "No misplaced Rust roots found".to_owned(),
            message:
                "All discovered live Rust roots stay within governed architecture zones or declared auxiliary roots."
                    .to_owned(),
            file: None,
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
#[path = "rs_arch_02_no_misplaced_roots_tests/mod.rs"]
mod rs_arch_02_no_misplaced_roots_tests;
