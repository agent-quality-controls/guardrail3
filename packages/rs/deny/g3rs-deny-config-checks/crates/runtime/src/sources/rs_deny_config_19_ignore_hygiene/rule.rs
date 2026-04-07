use deny_toml_parser::{AdvisoryIgnoreEntry, DenyToml};
use guardrail3_check_types::G3CheckResult;
use guardrail3_reason_policy::validate_reason_text;

use crate::support::{advisory_ignore_identity, advisory_ignore_reason, error, warn};

const ID: &str = "RS-DENY-CONFIG-19";

pub(crate) fn check(deny_rel_path: &str, deny: &DenyToml, results: &mut Vec<G3CheckResult>) {
    let Some(advisories) = deny.advisories.as_ref() else {
        return;
    };
    if advisories.ignore.is_empty() {
        return;
    }

    let mut documented_count = 0usize;
    let mut missing_reason_count = 0usize;
    let mut weak_reason_count = 0usize;

    for entry in &advisories.ignore {
        match entry {
            AdvisoryIgnoreEntry::Simple(id) => {
                missing_reason_count += 1;
                results.push(error(
                    ID,
                    "advisory ignore must use table form",
                    format!(
                        "`{deny_rel_path}` has `[advisories].ignore` string entry `{id}`; use table form with a `reason`."
                    ),
                    deny_rel_path,
                ));
                continue;
            }
            AdvisoryIgnoreEntry::Detailed(_) => {}
        }

        let Some(identity) = advisory_ignore_identity(entry) else {
            missing_reason_count += 1;
            results.push(error(
                ID,
                "malformed advisory ignore entry",
                format!(
                    "`{deny_rel_path}` has an `[advisories].ignore` entry without a valid advisory id or package selector."
                ),
                deny_rel_path,
            ));
            continue;
        };

        let Some(reason) = advisory_ignore_reason(entry) else {
            missing_reason_count += 1;
            results.push(error(
                ID,
                "advisory ignore missing reason",
                format!("`{deny_rel_path}` ignores `{identity}` without a `reason`."),
                deny_rel_path,
            ));
            continue;
        };

        match validate_reason_text(reason) {
            Ok(()) => {
                documented_count += 1;
                results.push(warn(
                    ID,
                    "advisory ignore entry",
                    format!("`{deny_rel_path}` has documented advisory ignore `{identity}`."),
                    deny_rel_path,
                ));
            }
            Err(issue) => {
                weak_reason_count += 1;
                results.push(error(
                    ID,
                    "advisory ignore reason too weak",
                    format!(
                        "`{deny_rel_path}` ignores `{identity}` with a weak `reason`: {}.",
                        issue.message()
                    ),
                    deny_rel_path,
                ));
            }
        }
    }

    let total = documented_count + missing_reason_count + weak_reason_count;
    if total > 0 {
        results.push(warn(
            ID,
            "advisory ignore count",
            format!(
                "`{deny_rel_path}` has {total} advisory ignores ({documented_count} documented, {missing_reason_count} missing reasons, {weak_reason_count} weak reasons)."
            ),
            deny_rel_path,
        ));
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod tests;
