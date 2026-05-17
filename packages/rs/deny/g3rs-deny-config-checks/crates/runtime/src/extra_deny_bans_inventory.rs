use g3rs_deny_types::G3RsDenyConfigChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::support::expectations::expected_ban_names;
use crate::support::identities::ban_name;
use crate::support::policy::{managed_profile_name, rust_policy_valid};

/// Rule identifier emitted by this check.
const ID: &str = "g3rs-deny/extra-deny-bans-inventory";

/// Runs the rule and appends any findings to `results`.
pub(crate) fn check(input: &G3RsDenyConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    if !rust_policy_valid(input) {
        return;
    }
    let Some(bans) = input.deny.bans.as_ref() else {
        return;
    };

    let expected_names = expected_ban_names(managed_profile_name(input));
    let mut extra_count = 0usize;
    for entry in &bans.deny {
        let Some(name) = ban_name(entry) else {
            results.push(G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Error,
                "deny ban entry malformed".to_owned(),
                format!(
                    "`{}` has `[bans].deny` entry without a usable `name` or `crate`.",
                    input.deny_rel_path
                ),
                Some(input.deny_rel_path.clone()),
                None,
            ));
            continue;
        };
        if expected_names.contains(&name) {
            continue;
        }
        extra_count = extra_count.saturating_add(1);
        results.push(
            G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Info,
                "extra deny ban".to_owned(),
                format!(
                    "`{}` adds deny ban `{name}` beyond the managed baseline.",
                    input.deny_rel_path
                ),
                Some(input.deny_rel_path.clone()),
                None,
            )
            .into_inventory(),
        );
    }

    results.push(
        G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Info,
            if extra_count == 0 {
                "no extra deny bans".to_owned()
            } else {
                "extra deny ban count".to_owned()
            },
            format!(
                "`{}` has {extra_count} deny bans beyond the managed baseline.",
                input.deny_rel_path
            ),
            Some(input.deny_rel_path.clone()),
            None,
        )
        .into_inventory(),
    );
}
