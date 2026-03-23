use crate::domain::report::{CheckResult, Severity};

use super::inputs::RustCodeFileInput;
use super::parse::{find_forbidden_macros, line_text};

const ID: &str = "RS-CODE-16";

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    if input.is_test {
        return;
    }

    for (line, macro_name) in find_forbidden_macros(input.ast) {
        let base_name = macro_name.rsplit("::").next().unwrap_or(&macro_name);
        if base_name != "panic" {
            continue;
        }

        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: "panic! macro".to_owned(),
            message: format!(
                "`panic!()` macro found: {}.",
                line_text(input.content, line)
            ),
            file: Some(input.rel_path.to_owned()),
            line: Some(line),
            inventory: false,
        });
    }
}

#[cfg(test)]
#[path = "rs_code_16_panic_macro_tests/mod.rs"]
mod tests;
