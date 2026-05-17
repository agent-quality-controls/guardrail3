use std::collections::BTreeSet;

use g3rs_clippy_types::G3RsClippyConfigChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::support::{
    clippy_document, expected_type_bans, garde_enabled, parse_ban_section, rust_policy_valid,
    rust_profile,
};

/// I D const.
const ID: &str = "g3rs-clippy/extra-type-ban";

/// check fn.
pub(crate) fn check(input: &G3RsClippyConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    if !rust_policy_valid(input) {
        return;
    }
    let Some(document) = clippy_document(input) else {
        return;
    };

    let section = parse_ban_section(document, "disallowed-types");
    let mut malformed_count = 0usize;
    for malformed in &section.malformed_messages {
        malformed_count = malformed_count.saturating_add(1);
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "disallowed-types section malformed".to_owned(),
            malformed.clone(),
            Some(input.clippy_rel_path.clone()),
            None,
        ));
    }

    let expected: BTreeSet<_> = expected_type_bans(rust_profile(input), garde_enabled(input))
        .into_iter()
        .collect();
    let mut extra_count = 0usize;
    for found in section.entries.into_iter().map(|entry| entry.path) {
        if !expected.contains(found.as_str()) {
            extra_count = extra_count.saturating_add(1);
            results.push(
                G3CheckResult::new(
                    ID.to_owned(),
                    G3Severity::Info,
                    "extra type ban".to_owned(),
                    format!("Additional type ban `{found}` beyond baseline."),
                    Some(input.clippy_rel_path.clone()),
                    None,
                )
                .into_inventory(),
            );
        }
    }

    if malformed_count == 0 && extra_count == 0 {
        results.push(
            G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Info,
                "no extra type bans".to_owned(),
                "No additional type bans beyond the managed baseline.".to_owned(),
                Some(input.clippy_rel_path.clone()),
                None,
            )
            .into_inventory(),
        );
    }
}
