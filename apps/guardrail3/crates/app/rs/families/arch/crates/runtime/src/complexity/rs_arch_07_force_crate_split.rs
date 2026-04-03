use guardrail3_domain_report::{CheckResult, Severity};

use crate::facts::CrateNode;

const ID: &str = "RS-ARCH-07";

const MAX_DEPENDENCIES: usize = 12;
const MAX_MODULE_DEPTH: usize = 3;
const MAX_SIBLING_DIRS: usize = 4;
const MAX_SIBLING_RS_FILES: usize = 10;

pub(crate) fn check(node: &CrateNode, results: &mut Vec<CheckResult>) {
    if node.cargo_parse_error.is_some() {
        return;
    }
    if !node.has_package {
        return;
    }

    let mut reasons = Vec::new();

    if node.dependency_count > MAX_DEPENDENCIES {
        reasons.push(format!(
            "{} dependencies (max {})",
            node.dependency_count, MAX_DEPENDENCIES
        ));
    }
    if node.max_module_depth > MAX_MODULE_DEPTH {
        reasons.push(format!(
            "module depth {} (max {})",
            node.max_module_depth, MAX_MODULE_DEPTH
        ));
    }
    if node.sibling_dir_count > MAX_SIBLING_DIRS {
        reasons.push(format!(
            "{} top-level directories under src/ (max {})",
            node.sibling_dir_count, MAX_SIBLING_DIRS
        ));
    }
    if node.sibling_rs_file_count > MAX_SIBLING_RS_FILES {
        reasons.push(format!(
            "{} top-level .rs files under src/ (max {})",
            node.sibling_rs_file_count, MAX_SIBLING_RS_FILES
        ));
    }

    if reasons.is_empty() {
        return;
    }

    results.push(CheckResult::from_parts(
        ID.to_owned(),
        Severity::Error,
        "crate too complex, must split".to_owned(),
        format!(
            "Crate `{}` exceeds complexity thresholds: {}. Extract groups of related modules into sub-crates under a `crates/` directory.",
            node.rel_dir,
            reasons.join(", ")
        ),
        Some(node.cargo_rel_path.clone()),
        None,
        false,
    ));
}
