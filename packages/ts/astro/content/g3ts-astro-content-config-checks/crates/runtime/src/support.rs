use g3ts_astro_content_types::{
    G3TsAstroContentPolicySnapshot, G3TsAstroContentPolicySurfaceState,
    G3TsAstroPackageSurfaceSnapshot, G3TsAstroPackageSurfaceState,
};
use guardrail3_check_types::{G3CheckResult, G3Severity};

/// Internal function `fn`.
pub(crate) const fn parsed_package(
    package: &G3TsAstroPackageSurfaceState,
) -> Option<&G3TsAstroPackageSurfaceSnapshot> {
    match package {
        G3TsAstroPackageSurfaceState::Parsed { snapshot } => Some(snapshot),
        G3TsAstroPackageSurfaceState::Missing { .. }
        | G3TsAstroPackageSurfaceState::Unreadable { .. }
        | G3TsAstroPackageSurfaceState::ParseError { .. } => None,
    }
}

/// Returns the relative path of the package surface across all parse states.
pub(crate) fn package_rel_path(package: &G3TsAstroPackageSurfaceState) -> &str {
    match package {
        G3TsAstroPackageSurfaceState::Missing { rel_path }
        | G3TsAstroPackageSurfaceState::Unreadable { rel_path, .. }
        | G3TsAstroPackageSurfaceState::ParseError { rel_path, .. } => rel_path,
        G3TsAstroPackageSurfaceState::Parsed { snapshot } => &snapshot.rel_path,
    }
}

/// Internal function `package_has_dependency`.
pub(crate) fn package_has_dependency(
    package: &G3TsAstroPackageSurfaceState,
    dependency_name: &str,
) -> bool {
    parsed_package(package).is_some_and(|snapshot| {
        snapshot
            .dependencies
            .iter()
            .chain(snapshot.dev_dependencies.iter())
            .any(|dependency| dependency == dependency_name)
    })
}

/// Internal function `fn`.
pub(crate) const fn parsed_content_policy(
    policy: &G3TsAstroContentPolicySurfaceState,
) -> Option<&G3TsAstroContentPolicySnapshot> {
    match policy {
        G3TsAstroContentPolicySurfaceState::Parsed { snapshot } => Some(snapshot),
        G3TsAstroContentPolicySurfaceState::Missing { .. }
        | G3TsAstroContentPolicySurfaceState::Unreadable { .. }
        | G3TsAstroContentPolicySurfaceState::ParseError { .. }
        | G3TsAstroContentPolicySurfaceState::MissingAstroPolicy { .. } => None,
    }
}

/// Returns the relative path of the content policy across all parse states.
pub(crate) fn content_policy_rel_path(policy: &G3TsAstroContentPolicySurfaceState) -> &str {
    match policy {
        G3TsAstroContentPolicySurfaceState::Missing { rel_path }
        | G3TsAstroContentPolicySurfaceState::Unreadable { rel_path, .. }
        | G3TsAstroContentPolicySurfaceState::ParseError { rel_path, .. }
        | G3TsAstroContentPolicySurfaceState::MissingAstroPolicy { rel_path } => rel_path,
        G3TsAstroContentPolicySurfaceState::Parsed { snapshot } => &snapshot.rel_path,
    }
}

/// Internal function `info`.
pub(crate) fn info(id: &str, title: &str, message: String, file: &str) -> G3CheckResult {
    G3CheckResult::new(
        id.to_owned(),
        G3Severity::Info,
        title.to_owned(),
        message,
        Some(file.to_owned()),
        None,
    )
    .into_inventory()
}

/// Internal function `error`.
pub(crate) fn error(id: &str, title: &str, message: String, file: Option<&str>) -> G3CheckResult {
    G3CheckResult::new(
        id.to_owned(),
        G3Severity::Error,
        title.to_owned(),
        message,
        file.map(str::to_owned),
        None,
    )
}

/// Internal function `warning`.
pub(crate) fn warning(id: &str, title: &str, message: String, file: Option<&str>) -> G3CheckResult {
    G3CheckResult::new(
        id.to_owned(),
        G3Severity::Warn,
        title.to_owned(),
        message,
        file.map(str::to_owned),
        None,
    )
    .into_inventory()
}
