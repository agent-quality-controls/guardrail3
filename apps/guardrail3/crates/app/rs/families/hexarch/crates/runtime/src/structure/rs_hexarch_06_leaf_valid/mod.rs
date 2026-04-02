use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::LeafHexarchInput;
use crate::inventory::push_success;

const ID: &str = "RS-HEXARCH-06";

pub fn check(input: &LeafHexarchInput<'_>, results: &mut Vec<CheckResult>) {
    if input.has_cargo && input.has_crates_dir {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            format!(
                "Service `{}` subdirectory {}/ has both Cargo.toml and crates/",
                input.app_name, input.label
            ),
            format!(
                "Service `{}` has `{}/` with both `Cargo.toml` and `crates/`. A subdirectory must be either a crate or a hex-in-hex, not both.",
                input.app_name, input.label
            ),
            Some(input.rel_path.to_owned()),
            None,
            false,
        ));
        return;
    }

    if input.has_cargo || input.has_crates_dir || input.gitkeep_only {
        push_success(
            results,
            ID,
            format!(
                "Service `{}` leaf {} has valid ownership shape",
                input.app_name, input.label
            ),
            format!(
                "Service `{}` keeps leaf `{}` as a crate, nested hex root, or placeholder.",
                input.app_name, input.rel_path
            ),
            Some(input.rel_path.to_owned()),
        );
        return;
    }

    results.push(CheckResult::from_parts(
    ID.to_owned(),
    Severity::Error,
    format!(
            "Service `{}` subdirectory {}/ missing Cargo.toml",
            input.app_name, input.label
        ),
    format!(
            "Service `{}` has `{}/` but it has no `Cargo.toml` and no `crates/` directory. Every subdirectory in a container folder must be its own crate, a hex-in-hex with its own `crates/` structure, or a placeholder with `.gitkeep`.",
            input.app_name, input.label
        ),
    Some(input.rel_path.to_owned()),
    None,
    false,
    ));
}

#[cfg(test)]
pub(super) fn results_for_test_root(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_tree(&test_support::walk(root))
}
#[cfg(test)]

mod rs_hexarch_06_leaf_valid_tests;
