use crate::domain::report::{CheckResult, Severity};

use super::inputs::RustCodeFileInput;
use super::parse::find_std_fs_glob_import_lines;

const ID: &str = "RS-CODE-21";

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    if input.is_test || input.rel_path.ends_with("src/fs.rs") {
        return;
    }

    for line in find_std_fs_glob_import_lines(input.ast) {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: "std::fs glob import".to_owned(),
            message: "Direct `use std::fs::*` glob import bypasses clippy method bans.".to_owned(),
            file: Some(input.rel_path.to_owned()),
            line: Some(line),
            inventory: false,
        });
    }
}

#[cfg(test)]
#[path = "rs_code_21_fs_glob_import_tests/mod.rs"]
mod tests;
