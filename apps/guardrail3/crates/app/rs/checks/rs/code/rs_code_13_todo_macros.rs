use crate::domain::report::{CheckResult, Severity};

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
                message: format!("`{macro_name}!()` macro found: {}.", line_text(input.content, line)),
                file: Some(input.rel_path.to_owned()),
                line: Some(line),
                inventory: false,
            }),
            "unreachable" if !input.is_test => results.push(CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "unreachable! macro".to_owned(),
                message: format!("`unreachable!()` macro found: {}.", line_text(input.content, line)),
                file: Some(input.rel_path.to_owned()),
                line: Some(line),
                inventory: false,
            }),
            _ => {}
        }
    }
}

#[cfg(test)]
#[path = "rs_code_13_todo_macros_tests.rs"]
mod tests;
