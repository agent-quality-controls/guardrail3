use guardrail3_check_types::{G3CheckResult, G3Severity};

pub(crate) use g3rs_garde_types::{
    G3RsGardeBoundaryFieldSite as BoundaryFieldSite,
    G3RsGardeDerivedBoundaryTypeSite as DerivedBoundaryTypeSite,
    G3RsGardeInputFailureSite as InputFailureSite,
    G3RsGardeManualDeserializeImplSite as ManualDeserializeImplSite,
    G3RsGardeQueryAsMacroSite as QueryAsMacroSite,
};

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

#[cfg(test)]
pub(crate) fn active_source_input() -> g3rs_garde_types::G3RsGardeSourceChecksInput {
    g3rs_garde_types::G3RsGardeSourceChecksInput {
        applicability: g3rs_garde_types::G3RsGardeApplicability::Active,
        garde_dependency_present: true,
        rust_policy: g3rs_garde_types::G3RsGardeRustPolicyInput::Missing,
        input_failures: Vec::new(),
        struct_targets: Vec::new(),
        enum_targets: Vec::new(),
        manual_deserialize_impls: Vec::new(),
        boundary_fields: Vec::new(),
        query_as_macros: Vec::new(),
    }
}

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
