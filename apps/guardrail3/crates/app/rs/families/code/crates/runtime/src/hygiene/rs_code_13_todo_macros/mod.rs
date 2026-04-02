use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::RustCodeFileInput;
use crate::parse::{find_forbidden_macros, line_text};

const ID: &str = "RS-CODE-13";

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    for info in find_forbidden_macros(input.ast, input.is_test_root) {
        let line = info.line;
        let macro_name = info.macro_name;
        let base_name = macro_name.rsplit("::").next().unwrap_or(&macro_name);
        match base_name {
            "todo" | "unimplemented" => results.push(CheckResult::from_parts(
                ID.to_owned(),
                Severity::Warn,
                format!("{macro_name}! macro"),
                format!(
                    "`{macro_name}!()` macro found: {}.",
                    line_text(input.content, line)
                ),
                Some(input.rel_path.to_owned()),
                Some(line),
                false,
            )),
            "unreachable" if !info.in_test_context => results.push(CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "unreachable! macro".to_owned(),
                format!(
                    "`unreachable!()` macro found: {}.",
                    line_text(input.content, line)
                ),
                Some(input.rel_path.to_owned()),
                Some(line),
                false,
            )),
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
pub(crate) fn check_source(rel_path: &str, content: &str, is_test_root: bool) -> Vec<CheckResult> {
    let ast = crate::parse::parse_rust_file(content)
        .unwrap_or_else(|error| std::panic::panic_any(format!("valid rust: {error}")));
    let input = crate::inputs::RustCodeFileInput {
        rel_path,
        content,
        ast: &ast,
        is_test_root,
        profile_name: None,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}

#[cfg(test)]

mod rs_code_13_todo_macros_tests;
