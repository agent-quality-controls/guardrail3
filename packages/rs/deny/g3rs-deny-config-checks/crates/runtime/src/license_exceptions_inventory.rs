#![expect(
    clippy::too_many_lines,
    reason = "classify_exception_entry walks the full schema (name resolution, blank-allow detection, missing-reason detection, weak-reason detection) for a single [[licenses.exceptions]] entry. Each branch builds a verbose G3CheckResult inline because the rule id, severity, title, message, and file path are all part of the contract under test; extracting per-branch helpers would multiply the surface and obscure that contract"
)]

use g3rs_deny_types::G3RsDenyConfigChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};
use guardrail3_reason_policy::validate_reason_text;

/// Rule identifier emitted by this check.
const ID: &str = "g3rs-deny/license-exceptions-inventory";

/// Counter triple updated as `[[licenses.exceptions]]` entries are classified.
#[derive(Default)]
struct ExceptionCounts {
    /// Entries with reasons accepted by `validate_reason_text`.
    documented: usize,
    /// Entries missing reasons or otherwise malformed.
    missing_or_invalid_reason: usize,
    /// Entries whose reasons failed validation.
    weak_reason: usize,
}

/// Runs the rule and appends any findings to `results`.
pub(crate) fn check(input: &G3RsDenyConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    let Some(licenses) = input.deny.licenses.as_ref() else {
        return;
    };
    if licenses.exceptions.is_empty() {
        return;
    }

    let mut counts = ExceptionCounts::default();
    for entry in &licenses.exceptions {
        classify_exception_entry(input, entry, &mut counts, results);
    }

    let total = counts
        .documented
        .saturating_add(counts.missing_or_invalid_reason)
        .saturating_add(counts.weak_reason);
    if total > 0 {
        let documented = counts.documented;
        let missing = counts.missing_or_invalid_reason;
        let weak = counts.weak_reason;
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Warn,
            "license exception count".to_owned(),
            format!(
                "`{}` has {total} license exceptions ({documented} documented, {missing} missing or invalid reasons, {weak} weak reasons).",
                input.deny_rel_path
            ),
            None,
            None,
        ));
    }
}

/// Classifies a single `[[licenses.exceptions]]` entry, updating `counts` and pushing findings.
fn classify_exception_entry(
    input: &G3RsDenyConfigChecksInput,
    entry: &deny_toml_parser::types::LicenseException,
    counts: &mut ExceptionCounts,
    results: &mut Vec<G3CheckResult>,
) {
    let Some(name) = entry
        .name
        .as_deref()
        .or(entry.crate_name.as_deref())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_owned)
    else {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "malformed license exception entry".to_owned(),
            format!(
                "`{}` has `[[licenses.exceptions]]` entry without a valid crate identifier.",
                input.deny_rel_path
            ),
            Some(input.deny_rel_path.clone()),
            None,
        ));
        return;
    };

    if entry.allow.iter().any(|license| license.trim().is_empty()) {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "malformed license exception entry".to_owned(),
            format!(
                "`{}` has `[[licenses.exceptions]]` entry `{name}` with blank allowed license name.",
                input.deny_rel_path
            ),
            Some(input.deny_rel_path.clone()),
            None,
        ));
        return;
    }

    let Some(reason) = entry.reason.as_deref() else {
        counts.missing_or_invalid_reason = counts.missing_or_invalid_reason.saturating_add(1);
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "license exception missing reason".to_owned(),
            format!(
                "`{}` has license exception `{name}` without a `reason`.",
                input.deny_rel_path
            ),
            Some(input.deny_rel_path.clone()),
            None,
        ));
        return;
    };

    match validate_reason_text(reason) {
        Ok(()) => {
            counts.documented = counts.documented.saturating_add(1);
            results.push(G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Warn,
                "license exception entry".to_owned(),
                format!(
                    "`{}` has documented license exception for `{name}`.",
                    input.deny_rel_path
                ),
                Some(input.deny_rel_path.clone()),
                None,
            ));
        }
        Err(issue) => {
            counts.weak_reason = counts.weak_reason.saturating_add(1);
            results.push(G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Error,
                "license exception reason too weak".to_owned(),
                format!(
                    "`{}` has license exception `{name}` with a weak `reason`: {}.",
                    input.deny_rel_path,
                    issue.message()
                ),
                Some(input.deny_rel_path.clone()),
                None,
            ));
        }
    }
}

#[cfg(test)]
#[path = "license_exceptions_inventory_tests/mod.rs"]
mod license_exceptions_inventory_tests;
