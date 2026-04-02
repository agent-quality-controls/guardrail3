use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::RustCodeFileInput;
use crate::parse::find_std_fs_glob_import_lines;

const ID: &str = "RS-CODE-21";

fn is_filesystem_boundary_module(rel_path: &str) -> bool {
    rel_path.ends_with("src/fs.rs")
        || rel_path.ends_with("src/fs/mod.rs")
        || rel_path.ends_with("fs/src/lib.rs")
}

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    if input.is_test_root || is_filesystem_boundary_module(input.rel_path) {
        return;
    }

    for line in find_std_fs_glob_import_lines(input.ast) {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "std::fs glob import".to_owned(),
            "Direct `use std::fs::*` glob import bypasses clippy method bans.".to_owned(),
            Some(input.rel_path.to_owned()),
            Some(line),
            false,
        ));
    }
}

