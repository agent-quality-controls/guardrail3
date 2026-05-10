use g3ts_jscpd_types::{G3TsJscpdChecksInput, G3TsJscpdRootState};

/// Discriminator describing which `G3TsJscpdRootState` variant the input
/// holds, used to produce assertion failure messages without panicking.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RootStateKind {
    /// The `Missing` variant.
    Missing,
    /// The `Unreadable` variant.
    Unreadable,
    /// The `ParseError` variant.
    ParseError,
    /// The `Parsed` variant.
    Parsed,
}

impl RootStateKind {
    /// Inspect a `G3TsJscpdRootState` value and return its variant kind.
    const fn of(state: &G3TsJscpdRootState) -> Self {
        match state {
            G3TsJscpdRootState::Missing => Self::Missing,
            G3TsJscpdRootState::Unreadable { .. } => Self::Unreadable,
            G3TsJscpdRootState::ParseError { .. } => Self::ParseError,
            G3TsJscpdRootState::Parsed { .. } => Self::Parsed,
        }
    }

    /// Human-readable label for the kind, used in failure messages.
    const fn label(self) -> &'static str {
        match self {
            Self::Missing => "missing",
            Self::Unreadable => "unreadable",
            Self::ParseError => "parse-error",
            Self::Parsed => "parsed",
        }
    }
}

/// Borrow the `rel_path` carried by `state`, when applicable.
const fn rel_path_of(state: &G3TsJscpdRootState) -> Option<&str> {
    match state {
        G3TsJscpdRootState::Missing => None,
        G3TsJscpdRootState::Unreadable { rel_path, .. }
        | G3TsJscpdRootState::ParseError { rel_path, .. } => Some(rel_path.as_str()),
        G3TsJscpdRootState::Parsed { snapshot } => Some(snapshot.rel_path.as_str()),
    }
}

/// Assert that `input.root` matches `expected_kind`, optionally verifying the
/// associated `rel_path` when `expected_rel_path` is `Some`.
///
/// # Panics
///
/// Panics if the actual variant kind of `input.root` does not equal
/// `expected_kind`, or if `expected_rel_path` is provided and does not match
/// the recorded `rel_path` for the variant.
pub fn assert_root_kind(
    input: &G3TsJscpdChecksInput,
    expected_kind: RootStateKind,
    expected_rel_path: Option<&str>,
) {
    let actual_kind = RootStateKind::of(&input.root);
    assert_eq!(
        actual_kind,
        expected_kind,
        "expected {} root .jscpd.json state, got: {:?}",
        expected_kind.label(),
        input.root
    );
    if let Some(expected) = expected_rel_path {
        let actual = rel_path_of(&input.root).unwrap_or("");
        assert_eq!(
            actual,
            expected,
            "{} root path mismatch",
            expected_kind.label()
        );
    }
}

/// Assert that the ingested root state is `Missing`.
///
/// # Panics
///
/// Panics if `input.root` is any variant other than
/// [`G3TsJscpdRootState::Missing`].
pub fn assert_root_missing(input: &G3TsJscpdChecksInput) {
    assert_root_kind(input, RootStateKind::Missing, None);
}

/// Assert that the ingested root state is `ParseError` for `expected_rel_path`.
///
/// # Panics
///
/// Panics if `input.root` is not a `ParseError` variant or the recorded
/// `rel_path` does not match `expected_rel_path`.
pub fn assert_root_parse_error(input: &G3TsJscpdChecksInput, expected_rel_path: &str) {
    assert_root_kind(input, RootStateKind::ParseError, Some(expected_rel_path));
}

/// Assert that the ingested root state is `Unreadable` for `expected_rel_path`.
///
/// # Panics
///
/// Panics if `input.root` is not an `Unreadable` variant or the recorded
/// `rel_path` does not match `expected_rel_path`.
pub fn assert_root_unreadable(input: &G3TsJscpdChecksInput, expected_rel_path: &str) {
    assert_root_kind(input, RootStateKind::Unreadable, Some(expected_rel_path));
}

/// Assert that the ingested root state is `Parsed` for `expected_rel_path`.
///
/// # Panics
///
/// Panics if `input.root` is not a `Parsed` variant or the parsed snapshot
/// `rel_path` does not match `expected_rel_path`.
pub fn assert_root_parsed(input: &G3TsJscpdChecksInput, expected_rel_path: &str) {
    assert_root_kind(input, RootStateKind::Parsed, Some(expected_rel_path));
}
