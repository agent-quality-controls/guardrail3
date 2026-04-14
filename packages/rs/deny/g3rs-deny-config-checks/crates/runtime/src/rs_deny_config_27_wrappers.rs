use g3rs_deny_config_checks_types::G3RsDenyConfigChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::support::{ban_name, expected_bans, join_set, managed_profile_name, rust_policy_valid, wrappers};

const ID: &str = "RS-DENY-CONFIG-27";

pub(crate) fn check(input: &G3RsDenyConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    if !rust_policy_valid(input) {
        return;
    }
    let Some(bans) = input.deny.bans.as_ref() else {
        return;
    };
    let expected = expected_bans(managed_profile_name(input));

    for entry in &bans.deny {
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
            continue;
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
            continue;
        };

        if actual_wrappers != expected_ban.wrappers {
            let message = if expected_ban.wrappers.is_empty() {
                format!(
                    "`{}` ban `{name}` must not add wrappers.",
                    input.deny_rel_path
                )
            } else {
                format!(
                    "`{}` ban `{name}` must keep wrappers `{}`.",
                    input.deny_rel_path,
                    join_set(&expected_ban.wrappers)
                )
            };
            results.push(G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Error,
                "managed ban wrappers changed".to_owned(),
                message,
                Some(input.deny_rel_path.clone()),
                None,
            ));
        }
    }
}

#[cfg(test)]
#[path = "rs_deny_config_27_wrappers_tests/mod.rs"]
mod tests;
