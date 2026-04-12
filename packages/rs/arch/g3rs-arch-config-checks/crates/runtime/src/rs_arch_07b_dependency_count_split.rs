use g3rs_arch_types::G3RsArchConfigCrate;
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "RS-ARCH-07B";
const MAX_DEPENDENCIES: usize = 12;

pub(crate) fn check(node: &G3RsArchConfigCrate, results: &mut Vec<G3CheckResult>) {
    if node.dependency_count <= MAX_DEPENDENCIES {
        return;
    }

    results.push(G3CheckResult::new(
        ID.to_owned(),
        G3Severity::Error,
        "crate has too many direct dependencies, must split".to_owned(),
        format!(
            "Crate `{}` has {} direct dependencies (max {}). Extract groups of related modules into sub-crates under a `crates/` directory.",
            node.rel_dir, node.dependency_count, MAX_DEPENDENCIES
        ),
        Some(node.cargo_rel_path.clone()),
        None,
    ));
}

#[cfg(test)]
#[path = "rs_arch_07b_dependency_count_split_tests/mod.rs"]
mod tests;
