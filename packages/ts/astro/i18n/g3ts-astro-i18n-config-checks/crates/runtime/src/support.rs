use g3ts_astro_i18n_types::{
    G3TsAstroI18nEslintSurfaceState, G3TsAstroI18nPolicySnapshot, G3TsAstroI18nPolicySurfaceState,
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

pub(crate) fn parsed_i18n_policy(
    policy: &G3TsAstroI18nPolicySurfaceState,
) -> Option<&G3TsAstroI18nPolicySnapshot> {
    match policy {
        G3TsAstroI18nPolicySurfaceState::Parsed { snapshot } => Some(snapshot),
        G3TsAstroI18nPolicySurfaceState::Missing { .. }
        | G3TsAstroI18nPolicySurfaceState::Unreadable { .. }
        | G3TsAstroI18nPolicySurfaceState::ParseError { .. }
        | G3TsAstroI18nPolicySurfaceState::MissingAstroPolicy { .. }
        | G3TsAstroI18nPolicySurfaceState::MissingI18nPolicy { .. } => None,
    }
}

pub(crate) fn i18n_policy_rel_path(policy: &G3TsAstroI18nPolicySurfaceState) -> Option<&str> {
    match policy {
        G3TsAstroI18nPolicySurfaceState::Missing { rel_path }
        | G3TsAstroI18nPolicySurfaceState::Unreadable { rel_path, .. }
        | G3TsAstroI18nPolicySurfaceState::ParseError { rel_path, .. }
        | G3TsAstroI18nPolicySurfaceState::MissingAstroPolicy { rel_path }
        | G3TsAstroI18nPolicySurfaceState::MissingI18nPolicy { rel_path } => Some(rel_path),
        G3TsAstroI18nPolicySurfaceState::Parsed { snapshot } => Some(&snapshot.rel_path),
    }
}

pub(crate) fn eslint_rel_path(config: &G3TsAstroI18nEslintSurfaceState) -> Option<&str> {
    match config {
        G3TsAstroI18nEslintSurfaceState::Missing { rel_path }
        | G3TsAstroI18nEslintSurfaceState::Unreadable { rel_path, .. }
        | G3TsAstroI18nEslintSurfaceState::ParseError { rel_path, .. } => Some(rel_path),
        G3TsAstroI18nEslintSurfaceState::Parsed { snapshot } => Some(&snapshot.rel_path),
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
