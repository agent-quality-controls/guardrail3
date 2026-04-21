use g3ts_package_types::{
    G3TsPackageChecksInput, G3TsPackageLocalSnapshot, G3TsPackageLocalState,
    G3TsPackageRootSnapshot, G3TsPackageRootState,
};
use guardrail3_check_types::{G3CheckResult, G3Severity};

const BANNED_DEPENDENCIES: &[&str] = &[
    "axios",
    "lodash",
    "moment",
    "uuid",
    "nanoid",
    "express",
    "classnames",
    "winston",
    "pino",
    "request",
    "got",
    "superagent",
    "node-fetch",
    "isomorphic-fetch",
    "underscore",
    "request-promise",
    "cross-fetch",
    "xregexp",
    "regexp-tree",
];

const BANNED_PREFIXES: &[&str] = &["embla-carousel"];

#[must_use]
pub(crate) fn root_rel_path(input: &G3TsPackageChecksInput) -> Option<&str> {
    match &input.root {
        G3TsPackageRootState::NotPackageManagerRoot => None,
        G3TsPackageRootState::Missing => None,
        G3TsPackageRootState::Unreadable { rel_path, .. }
        | G3TsPackageRootState::ParseError { rel_path, .. } => Some(rel_path),
        G3TsPackageRootState::Parsed { snapshot } => Some(&snapshot.rel_path),
    }
}

#[must_use]
pub(crate) fn parsed_root(input: &G3TsPackageChecksInput) -> Option<&G3TsPackageRootSnapshot> {
    match &input.root {
        G3TsPackageRootState::NotPackageManagerRoot => None,
        G3TsPackageRootState::Parsed { snapshot } => Some(snapshot),
        G3TsPackageRootState::Missing
        | G3TsPackageRootState::Unreadable { .. }
        | G3TsPackageRootState::ParseError { .. } => None,
    }
}

#[must_use]
pub(crate) fn local_parse_blockers(input: &G3TsPackageChecksInput) -> Vec<(&str, String)> {
    input
        .locals
        .iter()
        .filter_map(|state| match state {
            G3TsPackageLocalState::Unreadable { rel_path, reason }
            | G3TsPackageLocalState::ParseError { rel_path, reason } => {
                Some((rel_path.as_str(), reason.clone()))
            }
            G3TsPackageLocalState::Parsed { .. } => None,
        })
        .collect()
}

#[must_use]
pub(crate) fn local_banned_dependencies(
    input: &G3TsPackageChecksInput,
) -> Vec<(&G3TsPackageLocalSnapshot, String)> {
    input
        .locals
        .iter()
        .filter_map(|state| match state {
            G3TsPackageLocalState::Parsed { snapshot } => Some(snapshot),
            G3TsPackageLocalState::Unreadable { .. } | G3TsPackageLocalState::ParseError { .. } => {
                None
            }
        })
        .flat_map(|snapshot| {
            snapshot
                .dependencies
                .iter()
                .chain(snapshot.dev_dependencies.iter())
                .filter(|dependency| is_banned_dependency(dependency))
                .cloned()
                .map(|dependency| (snapshot, dependency))
                .collect::<Vec<_>>()
        })
        .collect()
}

#[must_use]
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

fn is_banned_dependency(dependency: &str) -> bool {
    BANNED_DEPENDENCIES.contains(&dependency)
        || BANNED_PREFIXES
            .iter()
            .any(|prefix| dependency.starts_with(prefix))
}
