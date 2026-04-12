use g3rs_code_file_tree_checks_types::G3RsCodeStructuralCapRoot;
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "RS-CODE-FILETREE-35";

pub(crate) fn check(root: &G3RsCodeStructuralCapRoot, results: &mut Vec<G3CheckResult>) {
    let mut exceeded = Vec::new();
    if root.max_module_depth > 6 {
        exceeded.push(format!("module depth {} > 6", root.max_module_depth));
    }
    if root.max_sibling_dirs > 12 {
        exceeded.push(format!(
            "sibling source directories {} > 12",
            root.max_sibling_dirs
        ));
    }
    if root.max_sibling_rs_files > 20 {
        exceeded.push(format!(
            "sibling .rs files {} > 20",
            root.max_sibling_rs_files
        ));
    }

    if exceeded.is_empty() {
        return;
    }

    results.push(G3CheckResult::new(
        ID.to_owned(),
        G3Severity::Error,
        "crate source tree exceeds structural caps".to_owned(),
        format!(
            "Rust root `{}` exceeds structural caps: {}. Restructure the crate into smaller modules or sub-crates.",
            root.root_rel_dir,
            exceeded.join(", ")
        ),
        Some(root.cargo_rel_path.clone()),
        None,
    ));
}

#[cfg(test)]
#[path = "rs_code_filetree_35_root_structural_cap_tests/mod.rs"]
mod tests;
