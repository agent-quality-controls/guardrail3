use crate::domain::report::{CheckResult, Severity};

use super::inputs::RustCodeFileInput;
use super::parse::{find_crate_level_allows, find_inline_mod_allows};

const ID: &str = "RS-CODE-01";

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    for (line, lint) in find_crate_level_allows(input.ast) {
        if lint == "unused_crate_dependencies" {
            continue;
        }
        push_result(input, results, line, &lint, None);
    }

    for info in find_inline_mod_allows(input.ast) {
        if info.lint == "unused_crate_dependencies" {
            continue;
        }
        push_result(
            input,
            results,
            info.line,
            &info.lint,
            Some(info.module_path.as_str()),
        );
    }
}

fn push_result(
    input: &RustCodeFileInput<'_>,
    results: &mut Vec<CheckResult>,
    line: usize,
    lint: &str,
    module_path: Option<&str>,
) {
    let severity = if input.is_test {
        Severity::Info
    } else {
        Severity::Error
    };
    let title = module_path.map_or_else(
        || "crate-level allow".to_owned(),
        |module_path| format!("module-level allow in {module_path}"),
    );
    let message = if input.is_test {
        format!("Crate/module-wide allow for `{lint}` is test-file exempt.")
    } else {
        format!("Crate/module-wide `allow({lint})` suppresses the lint too broadly.")
    };
    results.push(CheckResult {
        id: ID.to_owned(),
        severity,
        title,
        message,
        file: Some(input.rel_path.to_owned()),
        line: Some(line),
        inventory: false,
    });
}

#[cfg(test)]
#[path = "rs_code_01_crate_level_allow_tests/mod.rs"]
mod tests;
