use g3ts_package_types::{
    G3TsPackageChecksInput, G3TsPackageLocalState, G3TsPackageRootState,
    G3TsPackageSyncpackConfigState,
};

/// Discriminator describing which `G3TsPackageRootState` variant the input
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
    /// Inspect a `G3TsPackageRootState` value and return its variant kind.
    const fn of(state: &G3TsPackageRootState) -> Self {
        match state {
            G3TsPackageRootState::Missing => Self::Missing,
            G3TsPackageRootState::NotPackageManagerRoot => Self::NotPackageManagerRoot,
            G3TsPackageRootState::Unreadable { .. } => Self::Unreadable,
            G3TsPackageRootState::ParseError { .. } => Self::ParseError,
            G3TsPackageRootState::Parsed { .. } => Self::Parsed,
        }
    }
}

/// Discriminator describing which `G3TsPackageSyncpackConfigState` variant
/// the input holds, used to produce assertion failure messages without
/// panicking.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SyncpackKind {
    /// The `NotRequired` variant.
    NotRequired,
    /// The `Missing` variant.
    Missing,
    /// The `Unreadable` variant.
    Unreadable,
    /// The `ParseError` variant.
    ParseError,
    /// The `Parsed` variant.
    Parsed,
}

impl SyncpackKind {
    /// Inspect a `G3TsPackageSyncpackConfigState` value and return its
    /// variant kind.
    const fn of(state: &G3TsPackageSyncpackConfigState) -> Self {
        match state {
            G3TsPackageSyncpackConfigState::NotRequired => Self::NotRequired,
            G3TsPackageSyncpackConfigState::Missing { .. } => Self::Missing,
            G3TsPackageSyncpackConfigState::Unreadable { .. } => Self::Unreadable,
            G3TsPackageSyncpackConfigState::ParseError { .. } => Self::ParseError,
            G3TsPackageSyncpackConfigState::Parsed { .. } => Self::Parsed,
        }
    }
}

/// Borrow the rel-path recorded for any non-`Missing`/`NotPackageManagerRoot`
/// local manifest state.
fn local_rel_path(state: &G3TsPackageLocalState) -> &str {
    match state {
        G3TsPackageLocalState::Unreadable { rel_path, .. }
        | G3TsPackageLocalState::ParseError { rel_path, .. } => rel_path,
        G3TsPackageLocalState::Parsed { snapshot } => &snapshot.rel_path,
    }
}

/// Compare the actual `RootKind` of `input.root` against `expected_kind`,
/// using `label` in the failure message.
fn assert_root_kind(input: &G3TsPackageChecksInput, expected_kind: RootKind, label: &str) {
    let actual_kind = RootKind::of(&input.root);
    assert_eq!(
        actual_kind, expected_kind,
        "expected {label} root package state, got: {:?}",
        input.root
    );
}

/// Assert that the ingested root state is `Missing`.
///
/// # Panics
///
/// Panics if `input.root` is any variant other than
/// [`G3TsPackageRootState::Missing`].
pub fn assert_root_missing(input: &G3TsPackageChecksInput) {
    assert_root_kind(input, RootKind::Missing, "missing");
}

/// Assert that the ingested root state is `NotPackageManagerRoot`.
///
/// # Panics
///
/// Panics if `input.root` is any variant other than
/// [`G3TsPackageRootState::NotPackageManagerRoot`].
pub fn assert_root_not_package_manager_root(input: &G3TsPackageChecksInput) {
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
pub fn assert_root_parse_error(input: &G3TsPackageChecksInput, expected_rel_path: &str) {
    assert_root_kind(input, RootKind::ParseError, "parse-error");
    if let G3TsPackageRootState::ParseError { rel_path, .. } = &input.root {
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
pub fn assert_root_parsed(input: &G3TsPackageChecksInput, expected_rel_path: &str) {
    assert_root_kind(input, RootKind::Parsed, "parsed");
    if let G3TsPackageRootState::Parsed { snapshot } = &input.root {
        assert_eq!(
            snapshot.rel_path, expected_rel_path,
            "parsed root path mismatch"
        );
    }
}

/// Assert that the parsed root manifest declares the expected
/// `safely_runs_only_allow_pnpm` and `safely_runs_syncpack_lint` script
/// policies.
///
/// # Panics
///
/// Panics if `input.root` is not a `Parsed` variant or any of the script
/// policy flags differ from the expected values.
pub fn assert_root_script_policy(
    input: &G3TsPackageChecksInput,
    expected_only_allow_pnpm: bool,
    expected_syncpack_lint: bool,
) {
    assert_root_kind(input, RootKind::Parsed, "parsed");
    if let G3TsPackageRootState::Parsed { snapshot } = &input.root {
        assert_eq!(
            snapshot.safely_runs_only_allow_pnpm, expected_only_allow_pnpm,
            "root only-allow pnpm script policy mismatch"
        );
        assert_eq!(
            snapshot.safely_runs_syncpack_lint, expected_syncpack_lint,
            "root syncpack lint script policy mismatch"
        );
    }
}

/// Assert that the ingested local manifest paths match `expected` exactly,
/// in order.
///
/// # Panics
///
/// Panics if the projected list of local manifest paths differs from
/// `expected`.
pub fn assert_local_paths(input: &G3TsPackageChecksInput, expected: &[&str]) {
    let actual = input
        .locals
        .iter()
        .map(|state| local_rel_path(state).to_owned())
        .collect::<Vec<_>>();
    let expected = expected
        .iter()
        .map(|path| (*path).to_owned())
        .collect::<Vec<_>>();
    assert_eq!(actual, expected, "local manifest path mismatch");
}

/// Assert that the local manifest at `expected_rel_path` is in the
/// `ParseError` state.
///
/// # Panics
///
/// Panics when no local manifest with `expected_rel_path` is present, or
/// when the matching state is not a `ParseError`.
pub fn assert_local_parse_error(input: &G3TsPackageChecksInput, expected_rel_path: &str) {
    let matching = input
        .locals
        .iter()
        .find(|state| local_rel_path(state) == expected_rel_path);
    assert!(
        matching.is_some(),
        "missing local manifest state for `{expected_rel_path}`",
    );
    if let Some(state) = matching {
        assert!(
            matches!(state, G3TsPackageLocalState::ParseError { .. }),
            "expected local parse error state, got: {state:?}",
        );
    }
}

/// Assert that the parsed local manifest at `expected_rel_path` declares
/// `expected_dependencies` exactly, in order.
///
/// # Panics
///
/// Panics when no local manifest with `expected_rel_path` is present, when
/// the matching state is not `Parsed`, or when the dependency list does not
/// match `expected_dependencies`.
pub fn assert_local_dependency_names(
    input: &G3TsPackageChecksInput,
    expected_rel_path: &str,
    expected_dependencies: &[&str],
) {
    let matching = input
        .locals
        .iter()
        .find(|state| local_rel_path(state) == expected_rel_path);
    assert!(
        matching.is_some(),
        "missing local manifest state for `{expected_rel_path}`",
    );
    let Some(G3TsPackageLocalState::Parsed { snapshot }) = matching else {
        assert!(
            matching.is_none(),
            "expected parsed local package state, got: {matching:?}",
        );
        return;
    };
    let expected = expected_dependencies
        .iter()
        .map(|dependency| (*dependency).to_owned())
        .collect::<Vec<_>>();
    assert_eq!(
        snapshot.dependencies, expected,
        "local dependency list mismatch for `{expected_rel_path}`"
    );
}

/// Compare the actual `SyncpackKind` of `input.syncpack_config` against
/// `expected_kind`, using `label` in the failure message.
fn assert_syncpack_kind(input: &G3TsPackageChecksInput, expected_kind: SyncpackKind, label: &str) {
    let actual_kind = SyncpackKind::of(&input.syncpack_config);
    assert_eq!(
        actual_kind, expected_kind,
        "expected {label} Syncpack state, got: {:?}",
        input.syncpack_config
    );
}

/// Assert that the Syncpack config state is `NotRequired`.
///
/// # Panics
///
/// Panics when the Syncpack state is any other variant.
pub fn assert_syncpack_not_required(input: &G3TsPackageChecksInput) {
    assert_syncpack_kind(input, SyncpackKind::NotRequired, "not-required");
}

/// Assert that the Syncpack config is `Missing` at `expected_rel_path`.
///
/// # Panics
///
/// Panics when the Syncpack state is not `Missing` or the recorded
/// `rel_path` does not match `expected_rel_path`.
pub fn assert_syncpack_missing(input: &G3TsPackageChecksInput, expected_rel_path: &str) {
    assert_syncpack_kind(input, SyncpackKind::Missing, "missing");
    if let G3TsPackageSyncpackConfigState::Missing { rel_path } = &input.syncpack_config {
        assert_eq!(
            rel_path, expected_rel_path,
            "Syncpack missing path mismatch"
        );
    }
}

/// Assert that the parsed Syncpack snapshot lists exactly `expected`
/// missing source entries, in order.
///
/// # Panics
///
/// Panics when the Syncpack state is not `Parsed` or the missing source
/// entries differ from `expected`.
pub fn assert_syncpack_missing_source_entries(input: &G3TsPackageChecksInput, expected: &[&str]) {
    assert_syncpack_kind(input, SyncpackKind::Parsed, "parsed");
    if let G3TsPackageSyncpackConfigState::Parsed { snapshot } = &input.syncpack_config {
        let expected = expected
            .iter()
            .map(|entry| (*entry).to_owned())
            .collect::<Vec<_>>();
        assert_eq!(
            snapshot.missing_source_entries, expected,
            "Syncpack missing source entries mismatch"
        );
    }
}

/// Assert that the parsed Syncpack snapshot lists `expected_dependency`
/// among its missing forbidden bans.
///
/// # Panics
///
/// Panics when the Syncpack state is not `Parsed` or
/// `expected_dependency` is not present in the missing-bans list.
pub fn assert_syncpack_missing_forbidden_ban(
    input: &G3TsPackageChecksInput,
    expected_dependency: &str,
) {
    assert_syncpack_kind(input, SyncpackKind::Parsed, "parsed");
    if let G3TsPackageSyncpackConfigState::Parsed { snapshot } = &input.syncpack_config {
        assert!(
            snapshot
                .missing_forbidden_bans
                .iter()
                .any(|dependency| dependency == expected_dependency),
            "expected missing Syncpack forbidden ban for `{expected_dependency}`, got: {:?}",
            snapshot.missing_forbidden_bans
        );
    }
}
