use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "RS-CODE-07";

pub(crate) fn check(
    rel_path: &str,
    line: usize,
    line_text: &str,
    results: &mut Vec<G3CheckResult>,
) {
    results.push(G3CheckResult::new(
        ID.to_owned(),
        G3Severity::Warn,
        "EXCEPTION comment inventory".to_owned(),
        format!("Config exception comment: {line_text}"),
        Some(rel_path.to_owned()),
        Some(line),
    ));
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod rule_tests;
