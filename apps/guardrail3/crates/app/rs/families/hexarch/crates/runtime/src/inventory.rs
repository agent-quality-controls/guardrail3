use guardrail3_domain_report::{CheckResult, Severity};

pub(super) fn push_success(
    results: &mut Vec<CheckResult>,
    id: &str,
    title: impl Into<String>,
    message: impl Into<String>,
    file: Option<String>,
) {
    results.push(CheckResult {
        id: id.to_owned(),
        severity: Severity::Info,
        title: title.into(),
        message: message.into(),
        file,
        line: None,
        inventory: true,
    });
}
