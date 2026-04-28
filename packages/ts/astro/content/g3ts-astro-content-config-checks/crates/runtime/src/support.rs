use g3ts_astro_content_types::{
    G3TsAstroContentPolicySnapshot, G3TsAstroContentPolicySurfaceState,
    G3TsAstroPackageSurfaceSnapshot, G3TsAstroPackageSurfaceState,
};
use guardrail3_check_types::{G3CheckResult, G3Severity};

pub(crate) fn parsed_package(
    package: &G3TsAstroPackageSurfaceState,
) -> Option<&G3TsAstroPackageSurfaceSnapshot> {
    match package {
        G3TsAstroPackageSurfaceState::Parsed { snapshot } => Some(snapshot),
        G3TsAstroPackageSurfaceState::Missing { .. }
        | G3TsAstroPackageSurfaceState::Unreadable { .. }
        | G3TsAstroPackageSurfaceState::ParseError { .. } => None,
    }
}

pub(crate) fn package_rel_path(package: &G3TsAstroPackageSurfaceState) -> Option<&str> {
    match package {
        G3TsAstroPackageSurfaceState::Missing { rel_path }
        | G3TsAstroPackageSurfaceState::Unreadable { rel_path, .. }
        | G3TsAstroPackageSurfaceState::ParseError { rel_path, .. } => Some(rel_path),
        G3TsAstroPackageSurfaceState::Parsed { snapshot } => Some(&snapshot.rel_path),
    }
}

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

pub(crate) fn parsed_content_policy(
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

pub(crate) fn content_policy_rel_path(policy: &G3TsAstroContentPolicySurfaceState) -> Option<&str> {
    match policy {
        G3TsAstroContentPolicySurfaceState::Missing { rel_path }
        | G3TsAstroContentPolicySurfaceState::Unreadable { rel_path, .. }
        | G3TsAstroContentPolicySurfaceState::ParseError { rel_path, .. }
        | G3TsAstroContentPolicySurfaceState::MissingAstroPolicy { rel_path } => Some(rel_path),
        G3TsAstroContentPolicySurfaceState::Parsed { snapshot } => Some(&snapshot.rel_path),
    }
}

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
