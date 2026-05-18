use g3rs_deny_types::G3RsDenyConfigChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

/// Rule identifier emitted by this check.
const ID: &str = "g3rs-deny/license-exceptions-inventory";

/// Counter triple updated as `[[licenses.exceptions]]` entries are classified.
#[derive(Default)]
struct ExceptionCounts {
    /// Entries that match cargo-deny's license exception schema.
    valid: usize,
    /// Entries with missing crate names or blank allowed licenses.
    malformed: usize,
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

    let total = counts.valid.saturating_add(counts.malformed);
    if total > 0 {
        let valid = counts.valid;
        let malformed = counts.malformed;
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Warn,
            "license exception count".to_owned(),
            format!(
                "`{}` has {total} license exceptions ({valid} valid, {malformed} malformed).",
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
        counts.malformed = counts.malformed.saturating_add(1);
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
        counts.malformed = counts.malformed.saturating_add(1);
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

    counts.valid = counts.valid.saturating_add(1);
    results.push(G3CheckResult::new(
        ID.to_owned(),
        G3Severity::Warn,
        "license exception entry".to_owned(),
        format!(
            "`{}` has license exception for `{name}`.",
            input.deny_rel_path
        ),
        Some(input.deny_rel_path.clone()),
        None,
    ));
}
