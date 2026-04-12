use std::collections::BTreeSet;

use g3rs_deps_config_checks_types::G3RsDepsConfigChecksInput;
use g3rs_deps_types::{
    G3RsDepsConfigInputScope, G3RsDepsDependencySection, G3RsDepsResolvedDependency,
};
use guardrail3_check_types::{G3CheckResult, G3Severity};
use guardrail3_rs_toml_parser::RustProfile;

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

pub(crate) fn allowlist_present(input: &G3RsDepsConfigChecksInput) -> bool {
    input.allowlist_present
}

pub(crate) fn is_workspace_tooling(input: &G3RsDepsConfigChecksInput) -> bool {
    input.scope == G3RsDepsConfigInputScope::WorkspaceTooling
}

pub(crate) fn tool_installed(input: &G3RsDepsConfigChecksInput, tool: &str) -> bool {
    input.installed_tools.iter().any(|installed| installed == tool)
}

pub(crate) fn allowlisted(input: &G3RsDepsConfigChecksInput, dep_package_name: &str) -> bool {
    input.allowed_deps.iter().any(|dep| dep == dep_package_name)
}

pub(crate) fn workspace_is_library(input: &G3RsDepsConfigChecksInput) -> bool {
    input.profile == Some(RustProfile::Library)
}

pub(crate) fn unique_direct_dependency_names(input: &G3RsDepsConfigChecksInput) -> BTreeSet<&str> {
    input
        .dependencies
        .iter()
        .map(|entry| entry.package_name.as_str())
        .collect()
}

pub(crate) fn dependencies_in_section<'a>(
    input: &'a G3RsDepsConfigChecksInput,
    section: G3RsDepsDependencySection,
) -> impl Iterator<Item = &'a G3RsDepsResolvedDependency> {
    input
        .dependencies
        .iter()
        .filter(move |entry| entry.section == section)
}
