use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::RustCodeFileInput;
use crate::parse::{find_forbidden_macros, line_text};

const ID: &str = "RS-CODE-16";

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    if input.is_test_root {
        return;
    }

    for info in find_forbidden_macros(input.ast, input.is_test_root) {
        if info.in_test_context {
            continue;
        }
        let line = info.line;
        let macro_name = info.macro_name;
        let base_name = macro_name.rsplit("::").next().unwrap_or(&macro_name);
        if base_name != "panic" {
            continue;
        }

        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Warn,
            "panic! macro".to_owned(),
            format!(
                "`panic!()` macro found: {}.",
                line_text(input.content, line)
            ),
            Some(input.rel_path.to_owned()),
            Some(line),
            false,
        ));
    }
}

