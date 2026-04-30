use guardrail3_check_types::{G3CheckResult, G3Severity};

pub(crate) fn error(
    id: &str,
    title: &str,
    message: &str,
    file: &str,
    line: Option<usize>,
) -> G3CheckResult {
    G3CheckResult::new(
        id.to_owned(),
        G3Severity::Error,
        title.to_owned(),
        message.to_owned(),
        Some(file.to_owned()),
        line,
    )
}

pub(crate) fn info(
    id: &str,
    title: &str,
    message: String,
    file: &str,
    line: Option<usize>,
) -> G3CheckResult {
    G3CheckResult::new(
        id.to_owned(),
        G3Severity::Info,
        title.to_owned(),
        message,
        Some(file.to_owned()),
        line,
    )
    .into_inventory()
}
