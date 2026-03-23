use crate::domain::report::{CheckResult, Severity};

use super::garde_support::{CORE_METHOD_BANS, extract_ban_paths, missing_bans};
use super::inputs::GardeRootInput;

const ID: &str = "RS-GARDE-02";

pub fn check(input: &GardeRootInput<'_>, results: &mut Vec<CheckResult>) {
    let Some(parsed) = input.root.clippy_parsed.as_ref() else {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: "cannot verify core garde method bans".to_owned(),
            message: input.root.clippy_parse_error.clone().unwrap_or_else(|| {
                "No covering clippy configuration found for garde method-ban validation.".to_owned()
            }),
            file: input.root.clippy_rel_path.clone(),
            line: None,
            inventory: false,
        });
        return;
    };

    let found = extract_ban_paths(parsed, "disallowed-methods");
    let missing = missing_bans(&found, CORE_METHOD_BANS);
    if missing.is_empty() {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "core garde method bans present".to_owned(),
                message: "All core serde/toml/yaml deserialization bans are present in the covering clippy configuration.".to_owned(),
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
            title: "missing core garde method bans".to_owned(),
            message: format!(
                "Missing core garde deserialization bans from `disallowed-methods`: {}.",
                missing.join(", ")
            ),
            file: input.root.clippy_rel_path.clone(),
            line: None,
            inventory: false,
        });
    }
}

#[cfg(test)]
#[path = "rs_garde_02_core_method_bans_tests.rs"]
mod tests;
