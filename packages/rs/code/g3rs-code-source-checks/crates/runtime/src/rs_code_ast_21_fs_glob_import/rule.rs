use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::parse::find_std_fs_glob_import_lines;
use crate::support::CodeSourceRuleInput;

const ID: &str = "RS-CODE-SOURCE-21";

fn is_filesystem_boundary_module(rel_path: &str) -> bool {
    rel_path.ends_with("src/fs.rs")
        || rel_path.ends_with("src/fs/mod.rs")
        || rel_path.ends_with("fs/src/lib.rs")
}

pub(crate) fn check(input: &CodeSourceRuleInput<'_>, results: &mut Vec<G3CheckResult>) {
    if input.is_test || is_filesystem_boundary_module(input.rel_path) {
        return;
    }

    for line in find_std_fs_glob_import_lines(input.source) {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "std::fs glob import".to_owned(),
            "Direct `use std::fs::*` glob import bypasses clippy method bans.".to_owned(),
            Some(input.rel_path.to_owned()),
            Some(line),
        ));
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod rule_tests;
