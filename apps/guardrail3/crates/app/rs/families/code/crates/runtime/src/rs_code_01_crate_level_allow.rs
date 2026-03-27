use guardrail3_domain_report::{CheckResult, Severity};

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
pub(crate) fn run_family(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_root(root)
}

#[cfg(test)]
pub(crate) fn copy_fixture() -> tempfile::TempDir {
    crate::copy_test_fixture()
}

#[cfg(test)]
#[allow(dead_code)]
pub(crate) fn check_source(rel_path: &str, content: &str, is_test: bool) -> Vec<CheckResult> {
    let ast = super::parse::parse_rust_file(content).expect("valid rust");
    let input = super::inputs::RustCodeFileInput {
        rel_path,
        content,
        ast: &ast,
        is_test,
        profile_name: None,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}

#[cfg(test)]
#[path = "rs_code_01_crate_level_allow_tests/mod.rs"]
mod rs_code_01_crate_level_allow_tests;
