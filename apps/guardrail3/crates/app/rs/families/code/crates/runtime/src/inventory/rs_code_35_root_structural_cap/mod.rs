use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::StructuralCapInput;

const ID: &str = "RS-CODE-35";

pub fn check(input: &StructuralCapInput<'_>, results: &mut Vec<CheckResult>) {
    let mut exceeded = Vec::new();
    if input.max_module_depth > 6 {
        exceeded.push(format!("module depth {} > 6", input.max_module_depth));
    }
    if input.max_sibling_dirs > 12 {
        exceeded.push(format!(
            "sibling source directories {} > 12",
            input.max_sibling_dirs
        ));
    }
    if input.max_sibling_rs_files > 20 {
        exceeded.push(format!(
            "sibling .rs files {} > 20",
            input.max_sibling_rs_files
        ));
    }
    if exceeded.is_empty() {
        return;
    }
    results.push(CheckResult::from_parts(
        ID.to_owned(),
        Severity::Error,
        "crate source tree exceeds structural caps".to_owned(),
        format!(
            "Rust root `{}` exceeds structural caps: {}.",
            input.root_rel_dir,
            exceeded.join(", ")
        ),
        Some(input.cargo_rel_path.to_owned()),
        None,
        false,
    ));
}

#[cfg(test)]
pub(crate) fn run_family(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_root(root)
}

#[cfg(test)]

mod rs_code_35_root_structural_cap_tests;
