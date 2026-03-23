use crate::domain::report::{CheckResult, Severity};

use super::garde_support::{EXTRACTOR_TYPE_BANS, extract_ban_paths, missing_bans};
use super::inputs::GardeRootInput;

const ID: &str = "RS-GARDE-03";

pub fn check(input: &GardeRootInput<'_>, results: &mut Vec<CheckResult>) {
    let Some(parsed) = input.root.clippy_parsed.as_ref() else {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: "cannot verify garde extractor bans".to_owned(),
            message: input.root.clippy_parse_error.clone().unwrap_or_else(|| {
                "No covering clippy configuration found for garde extractor-ban validation."
                    .to_owned()
            }),
            file: input.root.clippy_rel_path.clone(),
            line: None,
            inventory: false,
        });
        return;
    };

    let found = extract_ban_paths(parsed, "disallowed-types");
    let missing = missing_bans(&found, EXTRACTOR_TYPE_BANS);
    if missing.is_empty() {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "garde extractor bans present".to_owned(),
                message: "All required Axum extractor bans are present in the covering clippy configuration.".to_owned(),
                file: input.root.clippy_rel_path.clone(),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    } else {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: "missing garde extractor bans".to_owned(),
            message: format!(
                "Missing garde extractor bans from `disallowed-types`: {}.",
                missing.join(", ")
            ),
            file: input.root.clippy_rel_path.clone(),
            line: None,
            inventory: false,
        });
    }
}

#[cfg(test)]
#[path = "rs_garde_03_extractor_type_bans_tests.rs"]
mod tests;
