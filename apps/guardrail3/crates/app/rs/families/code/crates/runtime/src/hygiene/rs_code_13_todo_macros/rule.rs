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
                Severity::Warn,
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

