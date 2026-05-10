use guardrail3_check_types::{G3CheckResult, G3Severity};

/// `warn` function.
pub(crate) fn warn(
    id: &str,
    title: impl Into<String>,
    message: impl Into<String>,
    file: &str,
) -> G3CheckResult {
    G3CheckResult::new(
        id.to_owned(),
        G3Severity::Warn,
        title.into(),
        message.into(),
        Some(file.to_owned()),
        None,
    )
}

/// `info` function.
pub(crate) fn info(
    id: &str,
    title: impl Into<String>,
    message: impl Into<String>,
    file: &str,
) -> G3CheckResult {
    G3CheckResult::new(
        id.to_owned(),
        G3Severity::Info,
        title.into(),
        message.into(),
        Some(file.to_owned()),
        None,
    )
    .into_inventory()
}
