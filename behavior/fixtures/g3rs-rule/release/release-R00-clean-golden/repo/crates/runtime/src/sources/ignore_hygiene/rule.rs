use deny_toml_parser::types::{AdvisoryIgnoreEntry, DenyToml};
use guardrail3_check_types::G3CheckResult;
use guardrail3_reason_policy::validate_reason_text;

use crate::support::findings::{error, warn};
use crate::support::identities::{advisory_ignore_identity, advisory_ignore_reason};

/// Rule identifier emitted by this check.
const ID: &str = "g3rs-deny/ignore-hygiene";

/// Counter triple updated as advisory-ignore entries are classified.
#[derive(Default)]
struct IgnoreCounts {
    /// Entries with reasons accepted by `validate_reason_text`.
    documented: usize,
    /// Entries missing reasons or otherwise malformed.
    missing_reason: usize,
    /// Entries whose reasons failed validation.
    weak_reason: usize,
}

/// Runs the rule and appends any findings to `results`.
pub(crate) fn check(deny_rel_path: &str, deny: &DenyToml, results: &mut Vec<G3CheckResult>) {
    let Some(advisories) = deny.advisories.as_ref() else {
        return;
    };
    if advisories.ignore.is_empty() {
        return;
    }

    let mut counts = IgnoreCounts::default();
    for entry in &advisories.ignore {
        classify_ignore_entry(deny_rel_path, entry, &mut counts, results);
    }

    let total = counts
        .documented
        .saturating_add(counts.missing_reason)
        .saturating_add(counts.weak_reason);
    if total > 0 {
        let documented = counts.documented;
        let missing = counts.missing_reason;
        let weak = counts.weak_reason;
        results.push(warn(
            ID,
            "advisory ignore count",
            format!(
                "`{deny_rel_path}` has {total} advisory ignores ({documented} documented, {missing} missing reasons, {weak} weak reasons)."
            ),
            deny_rel_path,
        ));
    }
}

/// Classifies a single `[advisories].ignore` entry, updating `counts` and pushing findings.
fn classify_ignore_entry(
    deny_rel_path: &str,
    entry: &AdvisoryIgnoreEntry,
    counts: &mut IgnoreCounts,
    results: &mut Vec<G3CheckResult>,
) {
    match entry {
        AdvisoryIgnoreEntry::Simple(id) => {
            counts.missing_reason = counts.missing_reason.saturating_add(1);
            results.push(error(
                ID,
                "advisory ignore must use table form",
                format!(
                    "`{deny_rel_path}` has `[advisories].ignore` string entry `{id}`; use table form with a `reason`."
                ),
                deny_rel_path,
            ));
            return;
        }
        AdvisoryIgnoreEntry::Detailed(_) => {}
    }

    let Some(identity) = advisory_ignore_identity(entry) else {
        counts.missing_reason = counts.missing_reason.saturating_add(1);
        results.push(error(
            ID,
            "malformed advisory ignore entry",
            format!(
                "`{deny_rel_path}` has an `[advisories].ignore` entry without a valid advisory id or package selector."
            ),
            deny_rel_path,
        ));
        return;
    };

    let Some(reason) = advisory_ignore_reason(entry) else {
        counts.missing_reason = counts.missing_reason.saturating_add(1);
        results.push(error(
            ID,
            "advisory ignore missing reason",
            format!("`{deny_rel_path}` ignores `{identity}` without a `reason`."),
            deny_rel_path,
        ));
        return;
    };

    match validate_reason_text(reason) {
        Ok(()) => {
            counts.documented = counts.documented.saturating_add(1);
            results.push(warn(
                ID,
                "advisory ignore entry",
                format!("`{deny_rel_path}` has documented advisory ignore `{identity}`."),
                deny_rel_path,
            ));
        }
        Err(issue) => {
            counts.weak_reason = counts.weak_reason.saturating_add(1);
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

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod rule_tests;
