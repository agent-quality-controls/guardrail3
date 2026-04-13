use g3rs_clippy_filetree_checks_types::G3RsClippyFileTreeChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "RS-CLIPPY-FILETREE-02";

pub(crate) fn check(input: &G3RsClippyFileTreeChecksInput, results: &mut Vec<G3CheckResult>) {
    for conflict in &input.shadowed_same_root_configs {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "same-root clippy config conflict".to_owned(),
            format!(
                "`{}` conflicts with `{}` at the same policy root. Keep only the highest-precedence clippy config file.",
                conflict.rel_path, conflict.preferred_rel_path
            ),
            Some(conflict.rel_path.clone()),
            None,
        ));
    }
}

#[cfg(test)]
#[path = "rs_clippy_filetree_02_same_root_conflict_tests/mod.rs"]
mod tests;
