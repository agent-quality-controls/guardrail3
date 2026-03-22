use std::collections::BTreeSet;

use crate::domain::report::{CheckResult, Severity};

use super::inputs::RustCodeFileInput;
use super::parse::{find_inline_std_fs_call_lines, find_std_fs_import_lines, line_text};

const ID: &str = "RS-CODE-15";

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    if input.is_test || input.rel_path.ends_with("src/fs.rs") {
        return;
    }

    let mut import_lines: BTreeSet<usize> = find_std_fs_import_lines(input.ast).into_iter().collect();
    let call_lines: BTreeSet<usize> = find_inline_std_fs_call_lines(input.ast).into_iter().collect();

    for line in import_lines.iter().copied() {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: "direct std::fs import".to_owned(),
            message: format!("Direct `use std::fs` import found: `{}`.", line_text(input.content, line)),
            file: Some(input.rel_path.to_owned()),
            line: Some(line),
            inventory: false,
        });
    }

    for line in call_lines {
        if import_lines.remove(&line) {
            continue;
        }
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: "direct std::fs call".to_owned(),
            message: format!("Direct `std::fs::*` call found: `{}`.", line_text(input.content, line)),
            file: Some(input.rel_path.to_owned()),
            line: Some(line),
            inventory: false,
        });
    }
}

#[cfg(test)]
#[path = "rs_code_15_direct_fs_usage_tests.rs"]
mod tests;
