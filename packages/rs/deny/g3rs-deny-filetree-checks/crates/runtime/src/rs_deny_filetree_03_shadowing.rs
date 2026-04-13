use g3rs_deny_filetree_checks_types::G3RsDenyFileTreeChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "RS-DENY-FILETREE-03";

pub(crate) fn check(input: &G3RsDenyFileTreeChecksInput, results: &mut Vec<G3CheckResult>) {
    if input.candidate_deny_rel_paths.len() <= 1 {
        return;
    }

    let mut rel_paths = input.candidate_deny_rel_paths.clone();
    rel_paths.sort();

    results.push(G3CheckResult::new(
        ID.to_owned(),
        G3Severity::Error,
        "multiple deny configs at one policy root".to_owned(),
        format!("`.` has multiple accepted deny configs: {}.", rel_paths.join(", ")),
        rel_paths.first().cloned(),
        None,
    ));
}

#[cfg(test)]
#[path = "rs_deny_filetree_03_shadowing_tests/mod.rs"]
mod tests;
