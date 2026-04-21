use g3ts_npmrc_types::{G3TsNpmrcChecksInput, G3TsNpmrcRootSnapshot, G3TsNpmrcRootState};
use guardrail3_check_types::{G3CheckResult, G3Severity};

const REQUIRED_SETTINGS: &[(&str, &str)] = &[
    ("strict-peer-dependencies", "true"),
    ("disallow-workspace-cycles", "true"),
    ("engine-strict", "true"),
    ("minimum-release-age", "1440"),
    ("block-exotic-subdeps", "true"),
    ("trust-policy", "warn"),
];

#[must_use]
pub(crate) fn root_rel_path(input: &G3TsNpmrcChecksInput) -> Option<&str> {
    match &input.root {
        G3TsNpmrcRootState::NotPackageManagerRoot | G3TsNpmrcRootState::Missing => None,
        G3TsNpmrcRootState::Unreadable { rel_path, .. }
        | G3TsNpmrcRootState::ParseError { rel_path, .. } => Some(rel_path),
        G3TsNpmrcRootState::Parsed { snapshot } => Some(&snapshot.rel_path),
    }
}

#[must_use]
pub(crate) fn parsed_root(input: &G3TsNpmrcChecksInput) -> Option<&G3TsNpmrcRootSnapshot> {
    match &input.root {
        G3TsNpmrcRootState::Parsed { snapshot } => Some(snapshot),
        G3TsNpmrcRootState::NotPackageManagerRoot
        | G3TsNpmrcRootState::Missing
        | G3TsNpmrcRootState::Unreadable { .. }
        | G3TsNpmrcRootState::ParseError { .. } => None,
    }
}

#[must_use]
pub(crate) fn duplicate_keys(snapshot: &G3TsNpmrcRootSnapshot) -> &[String] {
    &snapshot.duplicate_keys
}

#[must_use]
pub(crate) fn missing_required_settings(snapshot: &G3TsNpmrcRootSnapshot) -> Vec<&'static str> {
    REQUIRED_SETTINGS
        .iter()
        .filter_map(|(key, _)| effective_value(snapshot, key).is_none().then_some(*key))
        .collect()
}

#[must_use]
pub(crate) fn weakened_required_settings(
    snapshot: &G3TsNpmrcRootSnapshot,
) -> Vec<(&'static str, String, &'static str)> {
    REQUIRED_SETTINGS
        .iter()
        .filter_map(|(key, expected)| {
            let actual = effective_value(snapshot, key)?;
            (actual != *expected).then_some((*key, actual.to_owned(), *expected))
        })
        .collect()
}

#[must_use]
pub(crate) fn extra_settings(snapshot: &G3TsNpmrcRootSnapshot) -> Vec<(String, String)> {
    snapshot
        .settings
        .iter()
        .filter(|setting| !REQUIRED_SETTINGS.iter().any(|(key, _)| *key == setting.key))
        .map(|setting| (setting.key.clone(), setting.value.clone()))
        .collect()
}

fn effective_value<'a>(snapshot: &'a G3TsNpmrcRootSnapshot, key: &str) -> Option<&'a str> {
    snapshot
        .settings
        .iter()
        .rev()
        .find(|setting| setting.key == key)
        .map(|setting| setting.value.as_str())
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
