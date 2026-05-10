use g3ts_npmrc_types::{G3TsNpmrcChecksInput, G3TsNpmrcRootState};

/// Discriminator describing which `G3TsNpmrcRootState` variant the input
/// holds, used to produce assertion failure messages without panicking.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RootKind {
    /// The `Missing` variant.
    Missing,
    /// The `NotPackageManagerRoot` variant.
    NotPackageManagerRoot,
    /// The `Unreadable` variant.
    Unreadable,
    /// The `ParseError` variant.
    ParseError,
    /// The `Parsed` variant.
    Parsed,
}

impl RootKind {
    /// Inspect a `G3TsNpmrcRootState` value and return its variant kind.
    const fn of(state: &G3TsNpmrcRootState) -> Self {
        match state {
            G3TsNpmrcRootState::Missing => Self::Missing,
            G3TsNpmrcRootState::NotPackageManagerRoot => Self::NotPackageManagerRoot,
            G3TsNpmrcRootState::Unreadable { .. } => Self::Unreadable,
            G3TsNpmrcRootState::ParseError { .. } => Self::ParseError,
            G3TsNpmrcRootState::Parsed { .. } => Self::Parsed,
        }
    }
}

/// Compare the actual `RootKind` of `input.root` against `expected_kind`,
/// using `label` in the failure message.
fn assert_root_kind(input: &G3TsNpmrcChecksInput, expected_kind: RootKind, label: &str) {
    let actual_kind = RootKind::of(&input.root);
    assert_eq!(
        actual_kind, expected_kind,
        "expected {label} root .npmrc state, got: {:?}",
        input.root
    );
}

/// Assert that the ingested root state is `Missing`.
///
/// # Panics
///
/// Panics if `input.root` is any variant other than
/// [`G3TsNpmrcRootState::Missing`].
pub fn assert_root_missing(input: &G3TsNpmrcChecksInput) {
    assert_root_kind(input, RootKind::Missing, "missing");
}

/// Assert that the ingested root state is `NotPackageManagerRoot`.
///
/// # Panics
///
/// Panics if `input.root` is any variant other than
/// [`G3TsNpmrcRootState::NotPackageManagerRoot`].
pub fn assert_root_not_package_manager_root(input: &G3TsNpmrcChecksInput) {
    assert_root_kind(
        input,
        RootKind::NotPackageManagerRoot,
        "non-package-manager",
    );
}

/// Assert that the ingested root state is `ParseError` for `expected_rel_path`.
///
/// # Panics
///
/// Panics if `input.root` is not a `ParseError` variant or the recorded
/// `rel_path` does not match `expected_rel_path`.
pub fn assert_root_parse_error(input: &G3TsNpmrcChecksInput, expected_rel_path: &str) {
    assert_root_kind(input, RootKind::ParseError, "parse-error");
    if let G3TsNpmrcRootState::ParseError { rel_path, .. } = &input.root {
        assert_eq!(
            rel_path, expected_rel_path,
            "root parse error path mismatch"
        );
    }
}

/// Assert that the ingested root state is `Parsed` for `expected_rel_path`.
///
/// # Panics
///
/// Panics if `input.root` is not a `Parsed` variant or the parsed snapshot
/// `rel_path` does not match `expected_rel_path`.
pub fn assert_root_parsed(input: &G3TsNpmrcChecksInput, expected_rel_path: &str) {
    assert_root_kind(input, RootKind::Parsed, "parsed");
    if let G3TsNpmrcRootState::Parsed { snapshot } = &input.root {
        assert_eq!(
            snapshot.rel_path, expected_rel_path,
            "parsed root path mismatch"
        );
    }
}
