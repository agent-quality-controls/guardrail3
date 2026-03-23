use crate::domain::report::{CheckResult, Severity};

use super::inputs::DirectionalContainerHexarchInput;

const ID: &str = "RS-HEXARCH-03";

pub fn check(input: &DirectionalContainerHexarchInput<'_>, results: &mut Vec<CheckResult>) {
    for expected in ["inbound", "outbound"] {
        if input.dirs.iter().any(|dir| dir == expected) {
            continue;
        }
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: format!(
                "Service `{}` missing {}/{}/ directory",
                input.app_name, input.label, expected
            ),
            message: format!(
                "Service `{}` is missing `{}/{}/`. Create it and add a `.gitkeep` if not needed yet.",
                input.app_name, input.label, expected
            ),
            file: Some(input.rel_path.to_owned()),
            line: None,
            inventory: false,
        });
    }

    for dir in input.dirs {
        if ["inbound", "outbound"].contains(&dir.as_str()) {
            continue;
        }
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: format!(
                "Service `{}` has unexpected directory {}/{}/",
                input.app_name, input.label, dir
            ),
            message: format!(
                "Service `{}` has `{}/{}/` which is not part of the hex arch template. Only `{{inbound, outbound}}` directories are allowed in `{}/`.",
                input.app_name, input.label, dir, input.label
            ),
            file: Some(format!("{}/{}", input.rel_path, dir)),
            line: None,
            inventory: false,
        });
    }
}

#[cfg(test)]
#[path = "rs_hexarch_03_inbound_outbound_tests/mod.rs"]
mod tests;
