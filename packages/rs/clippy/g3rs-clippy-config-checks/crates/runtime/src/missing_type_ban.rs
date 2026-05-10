use std::collections::BTreeSet;

use g3rs_clippy_types::G3RsClippyConfigChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::support::{
    clippy_document, expected_required_type_bans, garde_enabled, parse_ban_section,
    rust_policy_valid,
};

/// I D const.
const ID: &str = "g3rs-clippy/missing-type-ban";

/// check fn.
pub(crate) fn check(input: &G3RsClippyConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    if !rust_policy_valid(input) {
        return;
    }
    let Some(document) = clippy_document(input) else {
        return;
    };

    let section = parse_ban_section(document, "disallowed-types");
    for malformed in &section.malformed_messages {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "disallowed-types section malformed".to_owned(),
            malformed.clone(),
            Some(input.clippy_rel_path.clone()),
            None,
        ));
    }

    let found: BTreeSet<_> = section
        .entries
        .into_iter()
        .map(|entry| entry.path)
        .collect();
    for expected in expected_required_type_bans(garde_enabled(input)) {
        if found.contains(expected) {
            results.push(
                G3CheckResult::new(
                    ID.to_owned(),
                    G3Severity::Info,
                    "type ban present".to_owned(),
                    format!("`{expected}` is banned."),
                    Some(input.clippy_rel_path.clone()),
                    None,
                )
                .into_inventory(),
            );
        } else {
            results.push(G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Error,
                "missing type ban".to_owned(),
                format!(
                    "`{expected}` is not present in `disallowed-types`. Add it to `disallowed-types` in clippy.toml."
                ),
                Some(input.clippy_rel_path.clone()),
                None,
            ));
        }
    }
}

#[cfg(test)]
#[path = "missing_type_ban_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod missing_type_ban_tests;
