use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::WorkspaceCoverageHexarchInput;
use crate::inventory::push_success;

const ID: &str = "RS-HEXARCH-27";

pub fn check(input: &WorkspaceCoverageHexarchInput<'_>, results: &mut Vec<CheckResult>) {
    if input.cargo_parse_error.is_some() || !input.is_workspace {
        return;
    }

    let before = results.len();
    for cargo_root in &input.app_local_cargo_roots {
        if !cargo_root.is_workspace {
            continue;
        }

        results.push(CheckResult::from_parts(
    ID.to_owned(),
    Severity::Error,
    format!(
                "Service `{}` nested workspace `{}` is forbidden",
                input.app_name, cargo_root.rel_dir
            ),
    format!(
                "Service `{}` contains nested workspace `{}` at `{}`. The app root Cargo.toml must be the only workspace root inside the app boundary.",
                input.app_name,
                cargo_root.rel_dir,
                cargo_root.cargo_rel_path
            ),
    Some(cargo_root.cargo_rel_path.to_owned()),
    None,
    false,
        ));
    }

    if results.len() == before {
        push_success(
            results,
            ID,
            format!(
                "Service `{}` has no nested workspaces under the app root",
                input.app_name
            ),
            format!(
                "Service `{}` keeps `{}` as the only workspace root inside the app boundary.",
                input.app_name, input.app_rel_dir
            ),
            Some(format!("{}/Cargo.toml", input.app_rel_dir)),
        );
    }
}

#[cfg(test)]
pub(super) fn results_for_test_root(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_tree(&test_support::walk(root))
}
#[cfg(test)]

mod rs_hexarch_27_nested_workspace_forbidden_tests;
