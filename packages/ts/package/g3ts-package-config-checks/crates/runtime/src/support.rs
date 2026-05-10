use g3ts_package_types::{G3TsPackageChecksInput, G3TsPackageRootSnapshot, G3TsPackageRootState};
use guardrail3_check_types::{G3CheckResult, G3Severity};

#[must_use]
/// `root_rel_path`: root rel path.
pub(crate) fn root_rel_path(input: &G3TsPackageChecksInput) -> Option<&str> {
    match &input.root {
        G3TsPackageRootState::NotPackageManagerRoot | G3TsPackageRootState::Missing => None,
        G3TsPackageRootState::Unreadable { rel_path, .. }
        | G3TsPackageRootState::ParseError { rel_path, .. } => Some(rel_path),
        G3TsPackageRootState::Parsed { snapshot } => Some(&snapshot.rel_path),
    }
}

#[must_use]
/// `parsed_root`: parsed root.
pub(crate) const fn parsed_root(
    input: &G3TsPackageChecksInput,
) -> Option<&G3TsPackageRootSnapshot> {
    match &input.root {
        G3TsPackageRootState::Parsed { snapshot } => Some(snapshot),
        G3TsPackageRootState::NotPackageManagerRoot
        | G3TsPackageRootState::Missing
        | G3TsPackageRootState::Unreadable { .. }
        | G3TsPackageRootState::ParseError { .. } => None,
    }
}

#[must_use]
/// `root_has_dependency`: root has dependency.
pub(crate) fn root_has_dependency(snapshot: &G3TsPackageRootSnapshot, dependency: &str) -> bool {
    snapshot
        .dependencies
        .iter()
        .chain(snapshot.dev_dependencies.iter())
        .any(|declared| declared == dependency)
}

#[must_use]
/// `root_invokes_tool`: root invokes tool.
pub(crate) fn root_invokes_tool(
    snapshot: &G3TsPackageRootSnapshot,
    executable: &str,
    first_arg: &str,
) -> bool {
    snapshot.script_tool_invocations.iter().any(|invocation| {
        invocation.executable == executable
            && invocation.args.first().is_some_and(|arg| arg == first_arg)
    })
}

#[must_use]
/// `forbidden_syncpack_deps_message`: forbidden syncpack deps message.
pub(crate) fn forbidden_syncpack_deps_message(input: &G3TsPackageChecksInput) -> String {
    input
        .forbidden_syncpack_deps
        .iter()
        .map(|dependency| format!("`{dependency}`"))
        .collect::<Vec<_>>()
        .join(", ")
}

#[must_use]
/// `is_pinned_pnpm_package_manager`: is pinned pnpm package manager.
pub(crate) fn is_pinned_pnpm_package_manager(value: Option<&str>) -> bool {
    let Some(value) = value else {
        return false;
    };
    let Some(version) = value.strip_prefix("pnpm@") else {
        return false;
    };
    !version.is_empty()
        && !version.contains(['^', '~', '*', '>', '<', ' ', '\t'])
        && version != "latest"
}

#[must_use]
/// `info`: info.
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

#[must_use]
/// `error`: error.
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
