use g3rs_deny_config_checks_types::G3RsDenyConfigChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::support::{ban_name, expected_ban_names};

const ID: &str = "RS-DENY-CONFIG-26";

pub(crate) fn check(input: &G3RsDenyConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    if !input.policy_context_valid {
        return;
    }
    let Some(bans) = input.deny.bans.as_ref() else {
        return;
    };

    let expected_names = expected_ban_names(input.profile_name.as_deref());
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
        extra_count += 1;
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

#[cfg(test)]
#[path = "rs_deny_config_26_extra_deny_bans_inventory_tests/mod.rs"]
mod tests;
