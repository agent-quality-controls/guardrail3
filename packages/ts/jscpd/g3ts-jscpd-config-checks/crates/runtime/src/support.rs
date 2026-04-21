use g3ts_jscpd_types::{G3TsJscpdChecksInput, G3TsJscpdRootSnapshot, G3TsJscpdRootState};
use guardrail3_check_types::{G3CheckResult, G3Severity};

pub(crate) const REQUIRED_IGNORES: &[&str] = &[
    "**/node_modules/**",
    "**/.next/**",
    "**/dist/**",
    "**/target/**",
    "**/components/ui/**",
];

pub(crate) fn root_rel_path(input: &G3TsJscpdChecksInput) -> Option<&str> {
    match &input.root {
        G3TsJscpdRootState::Missing => None,
        G3TsJscpdRootState::Unreadable { rel_path, .. }
        | G3TsJscpdRootState::ParseError { rel_path, .. } => Some(rel_path),
        G3TsJscpdRootState::Parsed { snapshot } => Some(&snapshot.rel_path),
    }
}

pub(crate) fn parsed_root(input: &G3TsJscpdChecksInput) -> Option<&G3TsJscpdRootSnapshot> {
    match &input.root {
        G3TsJscpdRootState::Parsed { snapshot } => Some(snapshot),
        G3TsJscpdRootState::Missing
        | G3TsJscpdRootState::Unreadable { .. }
        | G3TsJscpdRootState::ParseError { .. } => None,
    }
}

pub(crate) fn missing_required_ignores(snapshot: &G3TsJscpdRootSnapshot) -> Vec<&'static str> {
    REQUIRED_IGNORES
        .iter()
        .filter(|pattern| !snapshot.ignore.iter().any(|item| item == **pattern))
        .copied()
        .collect()
}

pub(crate) fn has_typescript_format(snapshot: &G3TsJscpdRootSnapshot) -> bool {
    snapshot.format.iter().any(|item| item == "typescript")
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
