use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::ExceptionCommentInput;

const ID: &str = "RS-CODE-07";

pub fn check(input: &ExceptionCommentInput<'_>, results: &mut Vec<CheckResult>) {
    results.push(
        CheckResult::from_parts(
            ID.to_owned(),
            Severity::Warn,
            "EXCEPTION comment inventory".to_owned(),
            format!("Config exception comment: {}", input.line_text),
            Some(input.rel_path.to_owned()),
            Some(input.line),
            false,
        ),
    );
}





// reason: test-only sidecar module wiring
