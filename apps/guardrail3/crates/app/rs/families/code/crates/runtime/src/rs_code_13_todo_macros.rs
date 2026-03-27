use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::RustCodeFileInput;
use super::parse::{find_forbidden_macros, line_text};

const ID: &str = "RS-CODE-13";

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    for (line, macro_name) in find_forbidden_macros(input.ast) {
        let base_name = macro_name.rsplit("::").next().unwrap_or(&macro_name);
        match base_name {
            "todo" | "unimplemented" => results.push(CheckResult {
                id: ID.to_owned(),
                severity: Severity::Warn,
                title: format!("{macro_name}! macro"),
                message: format!(
                    "`{macro_name}!()` macro found: {}.",
                    line_text(input.content, line)
                ),
                file: Some(input.rel_path.to_owned()),
                line: Some(line),
                inventory: false,
            }),
            "unreachable" if !input.is_test => results.push(CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "unreachable! macro".to_owned(),
                message: format!(
                    "`unreachable!()` macro found: {}.",
                    line_text(input.content, line)
                ),
                file: Some(input.rel_path.to_owned()),
                line: Some(line),
                inventory: false,
            }),
            _ => {}
        }
    }
}


#[cfg(test)]
pub(crate) fn run_family(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_root(root)
}

#[cfg(test)]
pub(crate) fn copy_fixture() -> test_support::TempDir {
    crate::copy_test_fixture()
}

#[cfg(test)]
pub(crate) fn check_source(rel_path: &str, content: &str, is_test: bool) -> Vec<CheckResult> {
    let ast = super::parse::parse_rust_file(content).unwrap_or_else(|error| std::panic::panic_any(format!("valid rust: {error}")));
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
#[path = "rs_code_13_todo_macros_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_code_13_todo_macros_tests;
