use guardrail3_check_types::{G3CheckResult, G3Severity};

pub(crate) fn error(
    id: &str,
    title: impl Into<String>,
    message: impl Into<String>,
    file: &str,
) -> G3CheckResult {
    G3CheckResult::new(
        id.to_owned(),
        G3Severity::Error,
        title.into(),
        message.into(),
        Some(file.to_owned()),
        None,
    )
}

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

pub(crate) fn message_covers_prefix(message: &str, prefix: &str) -> bool {
    if message == prefix {
        return true;
    }

    let Some(prefix_body) = prefix.strip_prefix('^') else {
        return false;
    };
    let Some(message_body) = message.strip_prefix('^') else {
        return false;
    };

    if let Some(stripped) = message_body.strip_prefix(prefix_body) {
        return has_valid_commit_suffix(stripped);
    }

    let Some(grouped) = message_body.strip_prefix('(') else {
        return false;
    };
    let Some(group_end) = grouped.find(')') else {
        return false;
    };
    let heads = &grouped[..group_end];
    let suffix = &grouped[(group_end + 1)..];

    heads.split('|').any(|head| head == prefix_body) && has_valid_commit_suffix(suffix)
}

fn has_valid_commit_suffix(suffix: &str) -> bool {
    suffix.starts_with(':') || (suffix.starts_with('(') && suffix.ends_with(':'))
}
