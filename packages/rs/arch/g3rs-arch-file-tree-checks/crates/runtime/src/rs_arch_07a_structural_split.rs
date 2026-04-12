use g3rs_arch_types::G3RsArchFileTreeCrate;
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "RS-ARCH-07A";

const MAX_MODULE_DEPTH: usize = 3;
const MAX_SIBLING_DIRS: usize = 4;
const MAX_SIBLING_RS_FILES: usize = 10;

pub(crate) fn check(node: &G3RsArchFileTreeCrate, results: &mut Vec<G3CheckResult>) {
    if node.cargo_parse_error.is_some() || !node.has_package {
        return;
    }

    let mut reasons = Vec::new();

    if node.max_module_depth > MAX_MODULE_DEPTH {
        reasons.push(format!(
            "module depth {} (max {})",
            node.max_module_depth, MAX_MODULE_DEPTH
        ));
    }
    if node.sibling_dir_count > MAX_SIBLING_DIRS {
        reasons.push(format!(
            "{} sibling directories (max {})",
            node.sibling_dir_count, MAX_SIBLING_DIRS
        ));
    }
    if node.sibling_rs_file_count > MAX_SIBLING_RS_FILES {
        reasons.push(format!(
            "{} sibling .rs files (max {})",
            node.sibling_rs_file_count, MAX_SIBLING_RS_FILES
        ));
    }

    if reasons.is_empty() {
        return;
    }

    results.push(G3CheckResult::new(
        ID.to_owned(),
        G3Severity::Error,
        "crate structure too complex, must split".to_owned(),
        format!(
            "Crate `{}` exceeds structural complexity thresholds: {}. Extract groups of related modules into sub-crates under a `crates/` directory.",
            node.rel_dir,
            reasons.join(", ")
        ),
        Some(node.cargo_rel_path.clone()),
        None,
    ));
}

#[cfg(test)]
#[path = "rs_arch_07a_structural_split_tests/mod.rs"]
mod tests;
