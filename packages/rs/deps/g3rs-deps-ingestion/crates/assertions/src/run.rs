#![expect(
    clippy::panic,
    clippy::missing_assert_message,
    reason = "this crate is the test-assertion harness for the deps-ingestion family; pub fn `assert_*` helpers exist precisely to fail-fast and panic with rich `{err:#?}` context, and the assert messages embedded directly in matches! / matches macros are the documented diagnostic"
)]

use g3rs_deps_ingestion_runtime::IngestionError;
use guardrail3_check_types::{G3CheckResult, G3Severity};

/// Snapshot of the publicly observable fields of a `G3CheckResult` used to compare
/// expected vs actual ingestion-pipeline outcomes in tests.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Finding {
    /// Rule identifier emitted by the check that produced this finding.
    id: String,
    /// Severity emitted by the check rule.
    severity: G3Severity,
    /// Human-readable title emitted by the check rule.
    title: String,
    /// Human-readable message emitted by the check rule.
    message: String,
    /// File path the finding refers to, when set.
    file: Option<String>,
    /// Whether the finding is an inventory record rather than a violation.
    inventory: bool,
}

/// Returns the deterministic, sorted list of `Finding`s extracted from `results`.
fn findings(results: &[G3CheckResult]) -> Vec<Finding> {
    let mut findings = results
        .iter()
        .map(|result| Finding {
            id: result.id().to_owned(),
            severity: result.severity(),
            title: result.title().to_owned(),
            message: result.message().to_owned(),
            file: result.file().map(str::to_owned),
            inventory: result.inventory(),
        })
        .collect::<Vec<_>>();
    findings.sort_by(|left, right| {
        (
            left.id.as_str(),
            format!("{:?}", left.severity),
            left.title.as_str(),
            left.message.as_str(),
            left.file.as_deref(),
            left.inventory,
        )
            .cmp(&(
                right.id.as_str(),
                format!("{:?}", right.severity),
                right.title.as_str(),
                right.message.as_str(),
                right.file.as_deref(),
                right.inventory,
            ))
    });
    findings
}

/// Reports whether `results` contains a finding whose id, title, and optional file match.
fn has_result(results: &[G3CheckResult], id: &str, title: &str, file: Option<&str>) -> bool {
    results
        .iter()
        .any(|result| result.id() == id && result.title() == title && result.file() == file)
}

/// Asserts that `err` is the missing-guardrail3-rs.toml ingestion error.
///
/// # Panics
///
/// Panics when `err` is any other variant of `IngestionError`.
pub fn assert_missing_guardrail3_rs(err: &IngestionError) {
    assert!(
        matches!(err, IngestionError::Guardrail3RsTomlNotFound),
        "{err:#?}"
    );
}

/// Asserts that `err` is the source-ingestion-not-implemented stub error.
///
/// # Panics
///
/// Panics when `err` is any other variant of `IngestionError`.
pub fn assert_source_ingestion_not_implemented(err: &IngestionError) {
    assert!(
        matches!(err, IngestionError::SourceIngestionNotImplemented),
        "{err:#?}"
    );
}

/// Asserts that `err` is `Unreadable { path, reason }` with `path.ends_with(expected_suffix)`
/// and a non-empty `reason`.
///
/// # Panics
///
/// Panics when `err` is any other variant or when the path/reason invariants do not hold.
pub fn assert_unreadable_error(err: &IngestionError, expected_suffix: &str) {
    let IngestionError::Unreadable { path, reason } = err else {
        panic!("expected unreadable error, got {err:#?}");
    };
    assert!(
        path.ends_with(expected_suffix),
        "expected path ending with `{expected_suffix}`, got {}",
        path.display()
    );
    assert!(!reason.is_empty(), "{err:#?}");
}

/// Asserts that `err` is `ParseFailed { path, reason }` with `path.ends_with(expected_suffix)`
/// and a non-empty `reason`.
///
/// # Panics
///
/// Panics when `err` is any other variant or when the path/reason invariants do not hold.
pub fn assert_parse_failed_error(err: &IngestionError, expected_suffix: &str) {
    let IngestionError::ParseFailed { path, reason } = err else {
        panic!("expected parse failure, got {err:#?}");
    };
    assert!(
        path.ends_with(expected_suffix),
        "expected path ending with `{expected_suffix}`, got {}",
        path.display()
    );
    assert!(!reason.is_empty(), "{err:#?}");
}

/// Asserts that `err` is `NormalizationFailed { path, reason }` with `path.ends_with(expected_suffix)`
/// and a `reason` that contains `expected_fragment`.
///
/// # Panics
///
/// Panics when `err` is any other variant or when the path/reason invariants do not hold.
pub fn assert_normalization_failed_contains(
    err: &IngestionError,
    expected_suffix: &str,
    expected_fragment: &str,
) {
    let IngestionError::NormalizationFailed { path, reason } = err else {
        panic!("expected normalization failure, got {err:#?}");
    };
    assert!(
        path.ends_with(expected_suffix),
        "expected path ending with `{expected_suffix}`, got {}",
        path.display()
    );
    assert!(
        reason.contains(expected_fragment),
        "expected `{expected_fragment}` in `{reason}`"
    );
}

/// Asserts the expected `assert_pipeline_missing_dependency_allowlist_for_library` outcome on `results`.
///
/// # Panics
///
/// Panics when `results` does not match the expected outcome.
pub fn assert_pipeline_missing_dependency_allowlist_for_library(results: &[G3CheckResult]) {
    assert!(
        has_result(
            results,
            "g3rs-deps/library-allowlist-present",
            "dependency allowlist missing",
            Some("crates/core/Cargo.toml"),
        ),
        "{results:#?}"
    );
}

/// Asserts the expected `assert_pipeline_workspace_tool_presence` outcome on `results`.
///
/// # Panics
///
/// Panics when `results` does not match the expected outcome.
pub fn assert_pipeline_workspace_tool_presence(results: &[G3CheckResult]) {
    assert_eq!(
        findings(results),
        vec![
            Finding {
                id: "g3rs-deps/cargo-deny-installed".to_owned(),
                severity: G3Severity::Info,
                title: "cargo-deny installed".to_owned(),
                message: "`cargo-deny` is available on PATH.".to_owned(),
                file: Some("Cargo.toml".to_owned()),
                inventory: true,
            },
            Finding {
                id: "g3rs-deps/cargo-dupes-installed".to_owned(),
                severity: G3Severity::Warn,
                title: "cargo-dupes missing".to_owned(),
                message:
                    "`cargo-dupes` was not found on PATH. Install with `cargo install cargo-dupes`."
                        .to_owned(),
                file: Some("Cargo.toml".to_owned()),
                inventory: false,
            },
            Finding {
                id: "g3rs-deps/cargo-machete-installed".to_owned(),
                severity: G3Severity::Info,
                title: "cargo-machete installed".to_owned(),
                message: "`cargo-machete` is available on PATH.".to_owned(),
                file: Some("Cargo.toml".to_owned()),
                inventory: true,
            },
            Finding {
                id: "g3rs-deps/gitleaks-installed".to_owned(),
                severity: G3Severity::Info,
                title: "gitleaks installed".to_owned(),
                message: "`gitleaks` is available on PATH.".to_owned(),
                file: Some("Cargo.toml".to_owned()),
                inventory: true,
            },
        ],
    );
}

/// Asserts the expected `assert_pipeline_workspace_tool_absence` outcome on `results`.
///
/// # Panics
///
/// Panics when `results` does not match the expected outcome.
pub fn assert_pipeline_workspace_tool_absence(results: &[G3CheckResult]) {
    assert_eq!(
        findings(results),
        vec![
            Finding {
                id: "g3rs-deps/cargo-deny-installed".to_owned(),
                severity: G3Severity::Error,
                title: "cargo-deny missing".to_owned(),
                message: "`cargo-deny` was not found on PATH. Install with `cargo install cargo-deny`.".to_owned(),
                file: Some("Cargo.toml".to_owned()),
                inventory: false,
            },
            Finding {
                id: "g3rs-deps/cargo-dupes-installed".to_owned(),
                severity: G3Severity::Warn,
                title: "cargo-dupes missing".to_owned(),
                message: "`cargo-dupes` was not found on PATH. Install with `cargo install cargo-dupes`.".to_owned(),
                file: Some("Cargo.toml".to_owned()),
                inventory: false,
            },
            Finding {
                id: "g3rs-deps/cargo-machete-installed".to_owned(),
                severity: G3Severity::Error,
                title: "cargo-machete missing".to_owned(),
                message: "`cargo-machete` was not found on PATH. Install with `cargo install cargo-machete`.".to_owned(),
                file: Some("Cargo.toml".to_owned()),
                inventory: false,
            },
            Finding {
                id: "g3rs-deps/gitleaks-installed".to_owned(),
                severity: G3Severity::Error,
                title: "gitleaks missing".to_owned(),
                message: "`gitleaks` was not found on PATH. Install with `brew install gitleaks` or download from GitHub.".to_owned(),
                file: Some("Cargo.toml".to_owned()),
                inventory: false,
            },
        ],
    );
}

/// Asserts the expected `assert_filetree_missing_lockfile_for_service` outcome on `results`.
///
/// # Panics
///
/// Panics when `results` does not match the expected outcome.
pub fn assert_filetree_missing_lockfile_for_service(results: &[G3CheckResult]) {
    assert_eq!(
        findings(results),
        vec![
            Finding {
                id: "g3rs-deps/cargo-lock-present".to_owned(),
                severity: G3Severity::Error,
                title: "Cargo.lock missing".to_owned(),
                message:
                    "`Cargo.lock` is missing. Run `cargo generate-lockfile` and commit the result."
                        .to_owned(),
                file: Some("Cargo.lock".to_owned()),
                inventory: false,
            },
            Finding {
                id: "g3rs-deps/gitignore-not-ignoring-cargo-lock".to_owned(),
                severity: G3Severity::Info,
                title: "Cargo.lock tracked by git".to_owned(),
                message: "No relevant `.gitignore` masks `Cargo.lock` at the workspace root."
                    .to_owned(),
                file: Some("Cargo.lock".to_owned()),
                inventory: true,
            },
        ],
    );
}

/// Asserts the expected `assert_filetree_missing_lockfile_for_library` outcome on `results`.
///
/// # Panics
///
/// Panics when `results` does not match the expected outcome.
pub fn assert_filetree_missing_lockfile_for_library(results: &[G3CheckResult]) {
    assert_eq!(
        findings(results),
        vec![
            Finding {
                id: "g3rs-deps/cargo-lock-present".to_owned(),
                severity: G3Severity::Info,
                title: "Cargo.lock missing".to_owned(),
                message: "Library-profile workspace is missing `Cargo.lock`.".to_owned(),
                file: Some("Cargo.lock".to_owned()),
                inventory: false,
            },
            Finding {
                id: "g3rs-deps/gitignore-not-ignoring-cargo-lock".to_owned(),
                severity: G3Severity::Info,
                title: "Cargo.lock tracked by git".to_owned(),
                message: "No relevant `.gitignore` masks `Cargo.lock` at the workspace root."
                    .to_owned(),
                file: Some("Cargo.lock".to_owned()),
                inventory: true,
            },
        ],
    );
}

/// Asserts the expected `assert_filetree_ignored_lockfile` outcome on `results`.
///
/// # Panics
///
/// Panics when `results` does not match the expected outcome.
pub fn assert_filetree_ignored_lockfile(results: &[G3CheckResult]) {
    assert_eq!(
        findings(results),
        vec![
            Finding {
                id: "g3rs-deps/cargo-lock-present".to_owned(),
                severity: G3Severity::Info,
                title: "Cargo.lock committed".to_owned(),
                message: "Workspace root has `Cargo.lock` committed.".to_owned(),
                file: Some("Cargo.lock".to_owned()),
                inventory: true,
            },
            Finding {
                id: "g3rs-deps/gitignore-not-ignoring-cargo-lock".to_owned(),
                severity: G3Severity::Error,
                title: "Cargo.lock ignored in gitignore".to_owned(),
                message: "`.gitignore` ignores `Cargo.lock`. Remove the line ignoring `Cargo.lock` from this `.gitignore`.".to_owned(),
                file: Some(".gitignore".to_owned()),
                inventory: false,
            },
        ],
    );
}

/// Asserts the expected `assert_filetree_unignored_lockfile` outcome on `results`.
///
/// # Panics
///
/// Panics when `results` does not match the expected outcome.
pub fn assert_filetree_unignored_lockfile(results: &[G3CheckResult]) {
    assert_eq!(
        findings(results),
        vec![
            Finding {
                id: "g3rs-deps/cargo-lock-present".to_owned(),
                severity: G3Severity::Info,
                title: "Cargo.lock committed".to_owned(),
                message: "Workspace root has `Cargo.lock` committed.".to_owned(),
                file: Some("Cargo.lock".to_owned()),
                inventory: true,
            },
            Finding {
                id: "g3rs-deps/gitignore-not-ignoring-cargo-lock".to_owned(),
                severity: G3Severity::Info,
                title: "Cargo.lock tracked by git".to_owned(),
                message: "No relevant `.gitignore` masks `Cargo.lock` at the workspace root."
                    .to_owned(),
                file: Some("Cargo.lock".to_owned()),
                inventory: true,
            },
        ],
    );
}
