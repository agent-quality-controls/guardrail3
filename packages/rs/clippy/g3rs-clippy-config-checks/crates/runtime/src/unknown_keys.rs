use std::collections::BTreeSet;

use g3rs_clippy_types::G3RsClippyConfigChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::support::{
    clippy_document, known_top_level_keys, managed_non_threshold_keys, normalized_key_distance,
};

/// I D const.
const ID: &str = "g3rs-clippy/unknown-keys";

/// check fn.
pub(crate) fn check(input: &G3RsClippyConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    let Some(document) = clippy_document(input) else {
        return;
    };

    let mut typo_count = 0usize;
    let known: BTreeSet<_> = known_top_level_keys()
        .into_iter()
        .chain(managed_non_threshold_keys())
        .collect();
    for key in clippy_toml_parser::top_level_keys(document) {
        let looks_like_managed_typo = !known.contains(key)
            && known
                .iter()
                .copied()
                .any(|managed| normalized_key_distance(key, managed) <= 2);
        if looks_like_managed_typo {
            typo_count = typo_count.saturating_add(1);
            results.push(G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Warn,
                "unrecognized clippy.toml key".to_owned(),
                format!(
                    "Top-level key `{key}` looks like a typo of a guardrail-managed clippy key. Check the spelling and correct it."
                ),
                Some(input.clippy_rel_path.clone()),
                None,
            ));
        }
    }

    if typo_count == 0 {
        results.push(
            G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Info,
                "no suspicious managed-key typos".to_owned(),
                "No top-level keys look like typos of guardrail-managed clippy keys.".to_owned(),
                Some(input.clippy_rel_path.clone()),
                None,
            )
            .into_inventory(),
        );
    }
}

#[cfg(test)]
#[path = "unknown_keys_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod unknown_keys_tests;
