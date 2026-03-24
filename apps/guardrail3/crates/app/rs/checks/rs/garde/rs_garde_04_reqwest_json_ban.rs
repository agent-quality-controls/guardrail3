use crate::domain::report::{CheckResult, Severity};

use super::garde_support::{REQWEST_JSON_BAN, extract_ban_paths};
use super::inputs::GardeRootInput;

const ID: &str = "RS-GARDE-04";

pub fn check(input: &GardeRootInput<'_>, results: &mut Vec<CheckResult>) {
    let Some(parsed) = input.root.clippy_parsed.as_ref() else {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: "cannot verify reqwest garde ban".to_owned(),
            message: input.root.clippy_parse_error.clone().unwrap_or_else(|| {
                "No covering clippy configuration found for reqwest garde-ban validation."
                    .to_owned()
            }),
            file: input.root.clippy_rel_path.clone(),
            line: None,
            inventory: false,
        });
        return;
    };

    let found = extract_ban_paths(parsed, "disallowed-methods");
    if found.contains(REQWEST_JSON_BAN) {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "reqwest garde ban present".to_owned(),
                message:
                    "`reqwest::Response::json` is banned in the covering clippy configuration."
                        .to_owned(),
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
            title: "missing reqwest garde ban".to_owned(),
            message: "Missing `reqwest::Response::json` from `disallowed-methods`.".to_owned(),
            file: input.root.clippy_rel_path.clone(),
            line: None,
            inventory: false,
        });
    }
}

#[cfg(test)]
#[path = "rs_garde_04_reqwest_json_ban_tests/mod.rs"]
mod tests;
