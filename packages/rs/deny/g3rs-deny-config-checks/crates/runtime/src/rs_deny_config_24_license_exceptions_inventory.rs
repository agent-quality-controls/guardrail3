use g3rs_deny_config_checks_types::G3RsDenyConfigChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};
use guardrail3_reason_policy::validate_reason_text;

const ID: &str = "RS-DENY-CONFIG-24";

pub(crate) fn check(input: &G3RsDenyConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    let Some(licenses) = input.deny.licenses.as_ref() else {
        return;
    };
    if licenses.exceptions.is_empty() {
        return;
    }

    let mut documented_count = 0usize;
    let mut missing_or_invalid_reason_count = 0usize;
    let mut weak_reason_count = 0usize;

    for entry in &licenses.exceptions {
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
            continue;
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
            continue;
        }

        let Some(reason) = entry.reason.as_deref() else {
            missing_or_invalid_reason_count += 1;
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
            continue;
        };

        match validate_reason_text(reason) {
            Ok(()) => {
                documented_count += 1;
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
                weak_reason_count += 1;
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

    let total = documented_count + missing_or_invalid_reason_count + weak_reason_count;
    if total > 0 {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Warn,
            "license exception count".to_owned(),
            format!(
                "`{}` has {total} license exceptions ({documented_count} documented, {missing_or_invalid_reason_count} missing or invalid reasons, {weak_reason_count} weak reasons).",
                input.deny_rel_path
            ),
            None,
            None,
        ));
    }
}

#[cfg(test)]
#[path = "rs_deny_config_24_license_exceptions_inventory_tests/mod.rs"]
mod tests;
