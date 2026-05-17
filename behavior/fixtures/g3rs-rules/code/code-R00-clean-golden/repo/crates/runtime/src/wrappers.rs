use std::collections::BTreeMap;

use deny_toml_parser::types::BanDenyEntry;
use g3rs_deny_types::G3RsDenyConfigChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::support::expectations::{BanExpectation, expected_bans};
use crate::support::identities::{ban_name, join_set, wrappers};
use crate::support::policy::{managed_profile_name, rust_policy_valid};

/// Rule identifier emitted by this check.
const ID: &str = "g3rs-deny/wrappers";

/// Runs the rule and appends any findings to `results`.
pub(crate) fn check(input: &G3RsDenyConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    if !rust_policy_valid(input) {
        return;
    }
    let Some(bans) = input.deny.bans.as_ref() else {
        return;
    };
    let expected = expected_bans(managed_profile_name(input));

    for entry in &bans.deny {
        check_entry(input, &expected, entry, results);
    }
}

/// Validates a single `[[bans.deny]]` entry against the managed expectation map.
fn check_entry(
    input: &G3RsDenyConfigChecksInput,
    expected: &BTreeMap<String, BanExpectation>,
    entry: &BanDenyEntry,
    results: &mut Vec<G3CheckResult>,
) {
    let actual_wrappers = wrappers(entry);
    let Some(name) = ban_name(entry) else {
        if !actual_wrappers.is_empty() {
            results.push(G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Error,
                "ban wrappers entry malformed".to_owned(),
                format!(
                    "`{}` has `[bans].deny` entry with wrappers but no usable `name` or `crate`.",
                    input.deny_rel_path
                ),
                Some(input.deny_rel_path.clone()),
                None,
            ));
        }
        return;
    };
    let Some(expected_ban) = expected.get(&name) else {
        if !actual_wrappers.is_empty() {
            results.push(
                G3CheckResult::new(
                    ID.to_owned(),
                    G3Severity::Info,
                    "project-specific ban wrappers".to_owned(),
                    format!(
                        "`{}` ban `{name}` adds project-specific wrappers `{}`.",
                        input.deny_rel_path,
                        join_set(&actual_wrappers)
                    ),
                    Some(input.deny_rel_path.clone()),
                    None,
                )
                .into_inventory(),
            );
        }
        return;
    };

    if actual_wrappers != expected_ban.wrappers {
        let (severity, message) = if actual_wrappers.is_superset(&expected_ban.wrappers) {
            (
                G3Severity::Warn,
                format!(
                    "`{}` ban `{name}` adds local wrappers `{}`.",
                    input.deny_rel_path,
                    join_set(&actual_wrappers)
                ),
            )
        } else {
            (
                G3Severity::Error,
                format!(
                    "`{}` ban `{name}` must keep wrappers `{}`.",
                    input.deny_rel_path,
                    join_set(&expected_ban.wrappers)
                ),
            )
        };
        results.push(G3CheckResult::new(
            ID.to_owned(),
            severity,
            "managed ban wrappers changed".to_owned(),
            message,
            Some(input.deny_rel_path.clone()),
            None,
        ));
    }
}

#[cfg(test)]
#[path = "wrappers_tests/mod.rs"]
mod wrappers_tests;
