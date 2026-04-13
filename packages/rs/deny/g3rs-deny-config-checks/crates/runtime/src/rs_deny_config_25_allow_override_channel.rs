use std::collections::BTreeSet;

use g3rs_deny_config_checks_types::G3RsDenyConfigChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::support::{allow_name, expected_bans, join_set};

const ID: &str = "RS-DENY-CONFIG-25";

pub(crate) fn check(input: &G3RsDenyConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    if !input.policy_context_valid {
        return;
    }

    let Some(bans) = input.deny.bans.as_ref() else {
        return;
    };
    if bans.allow.is_empty() {
        return;
    }

    let allow_names = bans.allow.iter().filter_map(allow_name).collect::<BTreeSet<_>>();
    if allow_names.len() != bans.allow.len() {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "bans allow-list malformed".to_owned(),
            format!(
                "`{}` has malformed `[bans].allow` entries that cannot be matched to crate names.",
                input.deny_rel_path
            ),
            Some(input.deny_rel_path.clone()),
            None,
        ));
    }

    if !allow_names.is_empty() {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "bans allow-list present".to_owned(),
            format!(
                "`{}` has non-empty `[bans].allow`: {}.",
                input.deny_rel_path,
                join_set(&allow_names)
            ),
            Some(input.deny_rel_path.clone()),
            None,
        ));
    }

    let expected = expected_bans(input.profile_name.as_deref());
    let actual_deny = bans.deny.iter().filter_map(crate::support::ban_name).collect::<BTreeSet<_>>();
    for name in allow_names {
        if expected.contains_key(&name) || actual_deny.contains(&name) {
            results.push(G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Error,
                "allow-list overrides deny-list".to_owned(),
                format!(
                    "`{}` allows `{name}` even though it is banned.",
                    input.deny_rel_path
                ),
                Some(input.deny_rel_path.clone()),
                None,
            ));
        }
    }
}

#[cfg(test)]
#[path = "rs_deny_config_25_allow_override_channel_tests/mod.rs"]
mod tests;
