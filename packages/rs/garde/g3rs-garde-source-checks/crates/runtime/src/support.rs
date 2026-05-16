use guardrail3_check_types::{G3CheckResult, G3Severity};

pub(crate) use g3rs_garde_types::{
    G3RsGardeBoundaryFieldSite as BoundaryFieldSite,
    G3RsGardeDerivedBoundaryTypeSite as DerivedBoundaryTypeSite,
    G3RsGardeInputFailureSite as InputFailureSite,
    G3RsGardeManualDeserializeImplSite as ManualDeserializeImplSite,
    G3RsGardeQueryAsMacroSite as QueryAsMacroSite,
};

/// Implements `error`.
pub(crate) fn error(
    id: &str,
    title: impl Into<String>,
    message: impl Into<String>,
    file: &str,
    line: Option<usize>,
) -> G3CheckResult {
    G3CheckResult::new(
        id.to_owned(),
        G3Severity::Error,
        title.into(),
        message.into(),
        Some(file.to_owned()),
        line,
    )
}

/// Implements `warn`.
pub(crate) fn warn(
    id: &str,
    title: impl Into<String>,
    message: impl Into<String>,
    file: Option<&str>,
    line: Option<usize>,
) -> G3CheckResult {
    G3CheckResult::new(
        id.to_owned(),
        G3Severity::Warn,
        title.into(),
        message.into(),
        file.map(str::to_owned),
        line,
    )
}
