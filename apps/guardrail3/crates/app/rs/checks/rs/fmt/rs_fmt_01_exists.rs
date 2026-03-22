use crate::domain::report::{CheckResult, Severity};

use super::inputs::RustfmtRootInput;

const ID: &str = "RS-FMT-01";

pub fn check(input: &RustfmtRootInput<'_>, results: &mut Vec<CheckResult>) {
    match input.config_rel {
        Some(rel) => results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "rustfmt config exists".to_owned(),
                message: "Found rustfmt config at workspace root".to_owned(),
                file: Some(rel.to_owned()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        ),
        None => results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: "rustfmt config missing".to_owned(),
            message: "Expected rustfmt.toml or .rustfmt.toml at workspace root".to_owned(),
            file: Some("".to_owned()),
            line: None,
            inventory: false,
        }),
    }
}

#[cfg(test)]
#[path = "rs_fmt_01_exists_tests.rs"]
mod tests;
