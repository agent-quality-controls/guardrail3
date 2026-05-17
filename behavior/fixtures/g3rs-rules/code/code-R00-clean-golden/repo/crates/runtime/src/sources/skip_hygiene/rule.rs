use deny_toml_parser::types::{BanSkipEntry, DenyToml};
use guardrail3_check_types::G3CheckResult;
use guardrail3_reason_policy::validate_reason_text;

use crate::support::findings::{error, warn};
use crate::support::identities::{skip_entry_name, skip_entry_reason};

/// Rule identifier emitted by this check.
const ID: &str = "g3rs-deny/skip-hygiene";

/// Counter triple updated as `[bans.skip]` entries are classified.
#[derive(Default)]
struct SkipCounts {
    /// Entries with reasons accepted by `validate_reason_text`.
    documented: usize,
    /// Entries missing reasons or otherwise malformed.
    missing_reason: usize,
    /// Entries whose reasons failed validation.
    weak_reason: usize,
}

/// Runs the rule and appends any findings to `results`.
pub(crate) fn check(deny_rel_path: &str, deny: &DenyToml, results: &mut Vec<G3CheckResult>) {
    let Some(bans) = deny.bans.as_ref() else {
        return;
    };
    if bans.skip.is_empty() {
        return;
    }

    let mut counts = SkipCounts::default();
    for entry in &bans.skip {
        classify_skip_entry(deny_rel_path, entry, &mut counts, results);
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
            "skip entry count",
            format!(
                "`{deny_rel_path}` has {total} skip entries ({documented} documented, {missing} missing reasons, {weak} weak reasons)."
            ),
            deny_rel_path,
        ));
    }
}

/// Classifies a single `[bans.skip]` entry, updating `counts` and pushing findings.
fn classify_skip_entry(
    deny_rel_path: &str,
    entry: &BanSkipEntry,
    counts: &mut SkipCounts,
    results: &mut Vec<G3CheckResult>,
) {
    match entry {
        BanSkipEntry::Simple(name) => {
            counts.missing_reason = counts.missing_reason.saturating_add(1);
            results.push(error(
                ID,
                "skip entry must use table form",
                format!(
                    "`{deny_rel_path}` has `[bans.skip]` string entry `{name}`; use table form with a `reason`."
                ),
                deny_rel_path,
            ));
            return;
        }
        BanSkipEntry::Detailed(_) => {}
    }

    let Some(name) = skip_entry_name(entry) else {
        counts.missing_reason = counts.missing_reason.saturating_add(1);
        results.push(error(
            ID,
            "malformed skip entry",
            format!("`{deny_rel_path}` has `[bans.skip]` entry without a valid crate identifier."),
            deny_rel_path,
        ));
        return;
    };

    let Some(reason) = skip_entry_reason(entry) else {
        counts.missing_reason = counts.missing_reason.saturating_add(1);
        results.push(error(
            ID,
            "skip entry missing reason",
            format!("`{deny_rel_path}` skips `{name}` without a `reason`."),
            deny_rel_path,
        ));
        return;
    };

    match validate_reason_text(reason) {
        Ok(()) => {
            counts.documented = counts.documented.saturating_add(1);
            results.push(warn(
                ID,
                "skip entry",
                format!("`{deny_rel_path}` has documented skip entry `{name}`."),
                deny_rel_path,
            ));
        }
        Err(issue) => {
            counts.weak_reason = counts.weak_reason.saturating_add(1);
            results.push(error(
                ID,
                "skip entry reason too weak",
                format!(
                    "`{deny_rel_path}` skips `{name}` with a weak `reason`: {}.",
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
