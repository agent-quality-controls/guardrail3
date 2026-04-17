use std::collections::BTreeMap;

use deny_toml_parser::types::DenyToml;
use guardrail3_check_types::G3CheckResult;

use crate::support::findings::warn;
use crate::support::identities::{
    advisory_ignore_identity, deny_entry_name, feature_entry_name, normalized_skip_identity,
};

const ID: &str = "RS-DENY-CONFIG-20";

pub(crate) fn check(deny_rel_path: &str, deny: &DenyToml, results: &mut Vec<G3CheckResult>) {
    if let Some(bans) = deny.bans.as_ref() {
        let mut deny_counts = BTreeMap::<String, usize>::new();
        for entry in &bans.deny {
            if let Some(name) = deny_entry_name(entry) {
                *deny_counts.entry(name).or_default() += 1;
            }
        }
        for (name, count) in deny_counts {
            if count > 1 {
                results.push(warn(
                    ID,
                    "duplicate deny entry",
                    format!("`{deny_rel_path}` has duplicate deny entry `{name}`."),
                    deny_rel_path,
                ));
            }
        }

        let mut skip_counts = BTreeMap::<String, usize>::new();
        for entry in &bans.skip {
            if let Some(name) = normalized_skip_identity(entry) {
                *skip_counts.entry(name).or_default() += 1;
            }
        }
        for (name, count) in skip_counts {
            if count > 1 {
                results.push(warn(
                    ID,
                    "duplicate skip entry",
                    format!("`{deny_rel_path}` has duplicate skip entry `{name}`."),
                    deny_rel_path,
                ));
            }
        }

        let mut feature_counts = BTreeMap::<String, usize>::new();
        for entry in &bans.features {
            if let Some(name) = feature_entry_name(entry) {
                *feature_counts.entry(name).or_default() += 1;
            }
        }
        for (name, count) in feature_counts {
            if count > 1 {
                results.push(warn(
                    ID,
                    "duplicate feature-ban entry",
                    format!("`{deny_rel_path}` has duplicate `[[bans.features]]` for `{name}`."),
                    deny_rel_path,
                ));
            }
        }
    }

    if let Some(advisories) = deny.advisories.as_ref() {
        let mut ignore_counts = BTreeMap::<String, usize>::new();
        for entry in &advisories.ignore {
            if let Some(identity) = advisory_ignore_identity(entry) {
                *ignore_counts.entry(identity).or_default() += 1;
            }
        }
        for (identity, count) in ignore_counts {
            if count > 1 {
                results.push(warn(
                    ID,
                    "duplicate advisory ignore entry",
                    format!("`{deny_rel_path}` has duplicate advisory ignore `{identity}`."),
                    deny_rel_path,
                ));
            }
        }
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod rule_tests;
