use crate::domain::report::{CheckResult, Severity};

use super::inputs::HexRootInput;

const ID: &str = "RS-HEXARCH-02";
const EXPECTED: [&str; 4] = ["adapters", "app", "domain", "ports"];

pub fn check(input: &HexRootInput<'_>, results: &mut Vec<CheckResult>) {
    for expected in EXPECTED {
        if input.dirs.iter().any(|dir| dir == expected) {
            continue;
        }
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: format!(
                "Service `{}` missing {}/{}/ directory",
                input.app_name,
                label(input),
                expected
            ),
            message: format!(
                "Service `{}` is missing `{}/{}/`. Create it and add a `.gitkeep` if not needed yet.",
                input.app_name,
                label(input),
                expected
            ),
            file: Some(input.crates_rel_dir.to_owned()),
            line: None,
            inventory: false,
        });
    }

    for dir in input.dirs {
        if EXPECTED.contains(&dir.as_str()) {
            continue;
        }
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: format!(
                "Service `{}` has unexpected directory {}/{}/",
                input.app_name,
                label(input),
                dir
            ),
            message: format!(
                "Service `{}` has `{}/{}/` which is not part of the hex arch template. Only `{{adapters, app, domain, ports}}` directories are allowed in `{}/`.",
                input.app_name,
                label(input),
                dir,
                label(input)
            ),
            file: Some(format!("{}/{}", input.crates_rel_dir, dir)),
            line: None,
            inventory: false,
        });
    }
}

fn label<'a>(input: &'a HexRootInput<'a>) -> &'a str {
    input
        .crates_rel_dir
        .strip_prefix(input.app_rel_dir)
        .unwrap_or(input.crates_rel_dir)
        .trim_start_matches('/')
}

#[cfg(test)]
#[path = "rs_hexarch_02_exact_contents_tests.rs"]
mod tests;
