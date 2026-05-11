use std::collections::BTreeSet;

use g3rs_deps_types::G3RsDepsConfigChecksInput;
use g3rs_deps_types::{
    G3RsDepsConfigInputScope, G3RsDepsDependencySection, G3RsDepsResolvedDependency,
};
use g3rs_toml_parser::types::RustProfile;
use guardrail3_check_types::{G3CheckResult, G3Severity};

/// Implements `info`.
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

/// Implements `warn`.
pub(crate) fn warn(id: &str, title: &str, message: String, file: &str) -> G3CheckResult {
    G3CheckResult::new(
        id.to_owned(),
        G3Severity::Warn,
        title.to_owned(),
        message,
        Some(file.to_owned()),
        None,
    )
}

/// Implements `error`.
pub(crate) fn error(id: &str, title: &str, message: String, file: &str) -> G3CheckResult {
    G3CheckResult::new(
        id.to_owned(),
        G3Severity::Error,
        title.to_owned(),
        message,
        Some(file.to_owned()),
        None,
    )
}

/// Implements `allowlist present`.
pub(crate) const fn allowlist_present(input: &G3RsDepsConfigChecksInput) -> bool {
    input.allowlist_present
}

/// Implements `is workspace tooling`.
pub(crate) fn is_workspace_tooling(input: &G3RsDepsConfigChecksInput) -> bool {
    input.scope == G3RsDepsConfigInputScope::WorkspaceTooling
}

/// Implements `tool installed`.
pub(crate) fn tool_installed(input: &G3RsDepsConfigChecksInput, tool: &str) -> bool {
    input
        .installed_tools
        .iter()
        .any(|installed| installed == tool)
}

/// Implements `allowlisted`.
pub(crate) fn allowlisted(input: &G3RsDepsConfigChecksInput, dep_package_name: &str) -> bool {
    input.allowed_deps.iter().any(|dep| dep == dep_package_name)
}

/// Implements `workspace is library`.
pub(crate) fn workspace_is_library(input: &G3RsDepsConfigChecksInput) -> bool {
    input.profile == Some(RustProfile::Library)
}

/// Implements `unique direct dependency names`.
pub(crate) fn unique_direct_dependency_names(input: &G3RsDepsConfigChecksInput) -> BTreeSet<&str> {
    input
        .dependencies
        .iter()
        .map(|entry| entry.package_name.as_str())
        .collect()
}

/// Implements `dependencies in section`.
pub(crate) fn dependencies_in_section(
    input: &G3RsDepsConfigChecksInput,
    section: G3RsDepsDependencySection,
) -> impl Iterator<Item = &G3RsDepsResolvedDependency> {
    input
        .dependencies
        .iter()
        .filter(move |entry| entry.section == section)
}
