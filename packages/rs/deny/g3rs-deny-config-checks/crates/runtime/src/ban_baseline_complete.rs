use g3rs_deny_types::G3RsDenyConfigChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::support::expectations::expected_bans;
use crate::support::identities::ban_name;
use crate::support::policy::{managed_profile_name, rust_policy_valid};

/// Rule identifier emitted by this check.
const ID: &str = "g3rs-deny/ban-baseline-complete";

/// Runs the rule and appends any findings to `results`.
pub(crate) fn check(input: &G3RsDenyConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    if !rust_policy_valid(input) {
        return;
    }

    let Some(bans) = input.deny.bans.as_ref() else {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "[bans] section missing".to_owned(),
            format!("`{}` has no `[bans]` section.", input.deny_rel_path),
            Some(input.deny_rel_path.clone()),
            None,
        ));
        return;
    };

    if input
        .deny
        .bans
        .as_ref()
        .is_some_and(|section| section.deny.is_empty())
    {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "[bans].deny missing".to_owned(),
            format!("`{}` must contain `[bans].deny`.", input.deny_rel_path),
            Some(input.deny_rel_path.clone()),
            None,
        ));
        return;
    }

    let expected = expected_bans(managed_profile_name(input));
    let actual_names = bans
        .deny
        .iter()
        .filter_map(ban_name)
        .collect::<std::collections::BTreeSet<_>>();

    for name in expected.keys() {
        if !actual_names.contains(name) {
            results.push(G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Error,
                "missing canonical ban".to_owned(),
                format!("`{}` is missing deny ban `{name}`.", input.deny_rel_path),
                Some(input.deny_rel_path.clone()),
                None,
            ));
        }
    }
}
