use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::RustCodeFileInput;
use super::parse::count_top_level_use_statements;

const ID: &str = "RS-CODE-11";

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    if input.is_test {
        return;
    }

    let use_count = count_top_level_use_statements(input.ast);
    if !(16..=20).contains(&use_count) {
        return;
    }

    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Warn,
        title: "many use statements".to_owned(),
        message: format!("{use_count} top-level use statements (warn at 16, max 20)."),
        file: Some(input.rel_path.to_owned()),
        line: None,
        inventory: false,
    });
}

#[cfg(test)]
#[path = "rs_code_11_use_count_warn_tests/mod.rs"]
mod tests;
