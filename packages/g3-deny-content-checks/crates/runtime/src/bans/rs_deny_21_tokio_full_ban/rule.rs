use deny_toml_parser::DenyToml;
use guardrail3_check_types::G3CheckResult;

use crate::support::{expected_tokio_allowed_features, feature_entry_name, join_set, warn};

const ID: &str = "RS-DENY-21";

pub(crate) fn check(deny_rel_path: &str, deny: &DenyToml, results: &mut Vec<G3CheckResult>) {
    let Some(bans) = deny.bans.as_ref() else {
        results.push(warn(
            ID,
            "tokio full feature not banned",
            format!("`{deny_rel_path}` must ban `tokio` feature `full` under `[[bans.features]]`."),
            deny_rel_path,
        ));
        return;
    };

    let tokio_entries = bans
        .features
        .iter()
        .filter(|entry| feature_entry_name(entry).as_deref() == Some("tokio"))
        .collect::<Vec<_>>();

    if tokio_entries.is_empty() {
        results.push(warn(
            ID,
            "tokio full feature not banned",
            format!("`{deny_rel_path}` must ban `tokio` feature `full` under `[[bans.features]]`."),
            deny_rel_path,
        ));
        return;
    }

    if tokio_entries.iter().any(|entry| !entry.deny.iter().any(|feature| feature == "full")) {
        results.push(warn(
            ID,
            "tokio full feature not banned",
            format!("`{deny_rel_path}` must ban `tokio` feature `full` under `[[bans.features]]`."),
            deny_rel_path,
        ));
    }

    let expected_allow = expected_tokio_allowed_features();
    if tokio_entries.iter().any(|entry| entry.allow.iter().cloned().collect::<std::collections::BTreeSet<_>>() != expected_allow) {
        results.push(warn(
            ID,
            "tokio allowed features changed",
            format!(
                "`{deny_rel_path}` must keep `tokio` allowed features `{}`.",
                join_set(&expected_allow)
            ),
            deny_rel_path,
        ));
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod tests;
