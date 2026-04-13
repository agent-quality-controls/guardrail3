use std::collections::BTreeSet;

use g3rs_clippy_config_checks_types::G3RsClippyConfigChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::support::{EXPECTED_LIBRARY_GLOBAL_STATE_TYPES, parse_ban_section, policy_context_valid, profile_name, raw_clippy};

const ID: &str = "RS-CLIPPY-CONFIG-14";

pub(crate) fn check(input: &G3RsClippyConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    if !policy_context_valid(input) || profile_name(input) != Some("library") {
        return;
    }
    let Some(parsed) = raw_clippy(input) else {
        return;
    };

    let section = parse_ban_section(parsed, "disallowed-types");
    let mut malformed_count = 0usize;
    for malformed in &section.malformed_messages {
        malformed_count += 1;
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "disallowed-types section malformed".to_owned(),
            malformed.clone(),
            Some(input.clippy_rel_path.clone()),
            None,
        ));
    }

    let found: BTreeSet<_> = section.entries.into_iter().map(|entry| entry.path).collect();
    let mut missing_count = 0usize;
    for expected in EXPECTED_LIBRARY_GLOBAL_STATE_TYPES {
        if !found.contains(*expected) {
            missing_count += 1;
            results.push(G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Error,
                "library clippy.toml missing global-state type ban".to_owned(),
                format!(
                    "Library profile must ban `{expected}` in `disallowed-types`. Add it to `disallowed-types` in clippy.toml."
                ),
                Some(input.clippy_rel_path.clone()),
                None,
            ));
        }
    }

    if malformed_count == 0 && missing_count == 0 {
        results.push(
            G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Info,
                "library global-state bans present".to_owned(),
                "Library profile includes all managed global-state type bans.".to_owned(),
                Some(input.clippy_rel_path.clone()),
                None,
            )
            .into_inventory(),
        );
    }
}

#[cfg(test)]
#[path = "rs_clippy_config_14_library_global_state_tests/mod.rs"]
mod tests;
