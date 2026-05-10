use g3ts_npmrc_types::{G3TsNpmrcChecksInput, G3TsNpmrcRootSnapshot, G3TsNpmrcRootState};
use guardrail3_check_types::{G3CheckResult, G3Severity};

/// `(key, expected_value)` pair describing one required `.npmrc` setting.
type RequiredSetting = (&'static str, &'static str);

/// Required `.npmrc` settings together with their canonical values, in the
/// order rules report them.
const REQUIRED_SETTINGS: &[RequiredSetting] = &[
    ("strict-peer-dependencies", "true"),
    ("disallow-workspace-cycles", "true"),
    ("engine-strict", "true"),
    ("minimum-release-age", "1440"),
    ("block-exotic-subdeps", "true"),
    ("trust-policy", "warn"),
];

/// Borrow the rel-path of the root `.npmrc` if any non-`Missing` state
/// records it.
#[must_use]
pub(crate) fn root_rel_path(input: &G3TsNpmrcChecksInput) -> Option<&str> {
    match &input.root {
        G3TsNpmrcRootState::NotPackageManagerRoot | G3TsNpmrcRootState::Missing => None,
        G3TsNpmrcRootState::Unreadable { rel_path, .. }
        | G3TsNpmrcRootState::ParseError { rel_path, .. } => Some(rel_path),
        G3TsNpmrcRootState::Parsed { snapshot } => Some(&snapshot.rel_path),
    }
}

/// Borrow the parsed root snapshot when `input.root` is in the `Parsed`
/// state, returning `None` otherwise.
#[must_use]
pub(crate) const fn parsed_root(input: &G3TsNpmrcChecksInput) -> Option<&G3TsNpmrcRootSnapshot> {
    match &input.root {
        G3TsNpmrcRootState::Parsed { snapshot } => Some(snapshot),
        G3TsNpmrcRootState::NotPackageManagerRoot
        | G3TsNpmrcRootState::Missing
        | G3TsNpmrcRootState::Unreadable { .. }
        | G3TsNpmrcRootState::ParseError { .. } => None,
    }
}

/// Return the duplicate setting keys reported by the parser.
#[must_use]
pub(crate) fn duplicate_keys(snapshot: &G3TsNpmrcRootSnapshot) -> &[String] {
    &snapshot.duplicate_keys
}

/// Return the [`REQUIRED_SETTINGS`] keys that are not declared in `snapshot`.
#[must_use]
pub(crate) fn missing_required_settings(snapshot: &G3TsNpmrcRootSnapshot) -> Vec<&'static str> {
    REQUIRED_SETTINGS
        .iter()
        .filter_map(|(key, _)| effective_value(snapshot, key).is_none().then_some(*key))
        .collect()
}

/// `(key, actual_value, expected_value)` triple reporting one weakened
/// `.npmrc` setting.
pub(crate) type WeakenedSetting = (&'static str, String, &'static str);

/// Return required settings whose effective value differs from the canonical
/// value in [`REQUIRED_SETTINGS`].
#[must_use]
pub(crate) fn weakened_required_settings(snapshot: &G3TsNpmrcRootSnapshot) -> Vec<WeakenedSetting> {
    REQUIRED_SETTINGS
        .iter()
        .filter_map(|(key, expected)| {
            let actual = effective_value(snapshot, key)?;
            (actual != *expected).then_some((*key, actual.to_owned(), *expected))
        })
        .collect()
}

/// `(key, value)` pair describing one extra (non-required) setting present in
/// the `.npmrc`.
pub(crate) type ExtraSetting = (String, String);

/// Return setting `(key, value)` pairs that are not part of
/// [`REQUIRED_SETTINGS`].
#[must_use]
pub(crate) fn extra_settings(snapshot: &G3TsNpmrcRootSnapshot) -> Vec<ExtraSetting> {
    snapshot
        .settings
        .iter()
        .filter(|setting| !REQUIRED_SETTINGS.iter().any(|(key, _)| *key == setting.key))
        .map(|setting| (setting.key.clone(), setting.value.clone()))
        .collect()
}

/// Return the last-declared value for `key` in the `.npmrc`, honoring
/// later-overrides-earlier semantics.
fn effective_value<'a>(snapshot: &'a G3TsNpmrcRootSnapshot, key: &str) -> Option<&'a str> {
    snapshot
        .settings
        .iter()
        .rev()
        .find(|setting| setting.key == key)
        .map(|setting| setting.value.as_str())
}

/// Build an inventory-tagged `Info` check result for `file`.
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

/// Build an `Error`-severity check result for `file`.
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
