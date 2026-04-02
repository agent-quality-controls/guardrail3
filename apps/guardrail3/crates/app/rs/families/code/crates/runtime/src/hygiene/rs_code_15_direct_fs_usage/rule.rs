use std::collections::BTreeSet;

use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::RustCodeFileInput;
use crate::parse::{find_inline_std_fs_call_lines, find_std_fs_import_lines, line_text};

const ID: &str = "RS-CODE-15";

fn is_filesystem_boundary_module(rel_path: &str) -> bool {
    rel_path.ends_with("src/fs.rs")
        || rel_path.ends_with("src/fs/mod.rs")
        || rel_path.ends_with("fs/src/lib.rs")
}

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    if input.is_test_root || is_filesystem_boundary_module(input.rel_path) {
        return;
    }

    let mut import_lines: BTreeSet<usize> =
        find_std_fs_import_lines(input.ast).into_iter().collect();
    let call_lines: BTreeSet<usize> = find_inline_std_fs_call_lines(input.ast)
        .into_iter()
        .collect();

    for line in import_lines.iter().copied() {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "direct std::fs import".to_owned(),
            format!(
                "Direct `use std::fs` import found: `{}`.",
                line_text(input.content, line)
            ),
            Some(input.rel_path.to_owned()),
            Some(line),
            false,
        ));
    }

    for line in call_lines {
        if import_lines.remove(&line) {
            continue;
        }
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "direct std::fs call".to_owned(),
            format!(
                "Direct `std::fs::*` call found: `{}`.",
                line_text(input.content, line)
            ),
            Some(input.rel_path.to_owned()),
            Some(line),
            false,
        ));
    }
}

