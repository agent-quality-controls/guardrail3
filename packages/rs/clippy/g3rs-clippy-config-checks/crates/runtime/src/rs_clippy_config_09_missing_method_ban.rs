use std::collections::BTreeSet;

use g3rs_clippy_types::G3RsClippyConfigChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::support::{
    expected_method_bans, garde_enabled, parse_ban_section, raw_clippy, rust_policy_valid,
};

const ID: &str = "RS-CLIPPY-CONFIG-09";

pub(crate) fn check(input: &G3RsClippyConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    if !rust_policy_valid(input) {
        return;
    }
    let Some(parsed) = raw_clippy(input) else {
        return;
    };

    let section = parse_ban_section(parsed, "disallowed-methods");
    for malformed in &section.malformed_messages {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "disallowed-methods section malformed".to_owned(),
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
    for expected in expected_method_bans(garde_enabled(input)) {
        if found.contains(expected) {
            results.push(
                G3CheckResult::new(
                    ID.to_owned(),
                    G3Severity::Info,
                    "method ban present".to_owned(),
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
                "missing method ban".to_owned(),
                format!(
                    "`{expected}` is not present in `disallowed-methods`. Add it to `disallowed-methods` in clippy.toml."
                ),
                Some(input.clippy_rel_path.clone()),
                None,
            ));
        }
    }
}
