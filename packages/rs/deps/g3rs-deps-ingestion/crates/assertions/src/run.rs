use g3rs_deps_ingestion_runtime::IngestionError;
use guardrail3_check_types::{G3CheckResult, G3Severity};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Finding {
    id: String,
    severity: G3Severity,
    title: String,
    message: String,
    file: Option<String>,
    inventory: bool,
}

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

fn has_result(results: &[G3CheckResult], id: &str, title: &str, file: Option<&str>) -> bool {
    results.iter().any(|result| {
        result.id() == id && result.title() == title && result.file() == file
    })
}

pub fn assert_missing_guardrail3_rs(err: &IngestionError) {
    assert!(
        matches!(err, IngestionError::Guardrail3RsTomlNotFound),
        "{err:#?}"
    );
}

pub fn assert_source_ingestion_not_implemented(err: &IngestionError) {
    assert!(
        matches!(err, IngestionError::SourceIngestionNotImplemented),
        "{err:#?}"
    );
}

pub fn assert_unreadable_error(err: &IngestionError, expected_suffix: &str) {
    if let IngestionError::Unreadable { path, reason } = err {
        assert!(path.ends_with(expected_suffix), "{path:?}");
        assert!(!reason.is_empty(), "{err:#?}");
    } else {
        assert!(false, "expected unreadable error, got {err:#?}");
    }
}

pub fn assert_parse_failed_error(err: &IngestionError, expected_suffix: &str) {
    if let IngestionError::ParseFailed { path, reason } = err {
        assert!(path.ends_with(expected_suffix), "{path:?}");
        assert!(!reason.is_empty(), "{err:#?}");
    } else {
        assert!(false, "expected parse failure, got {err:#?}");
    }
}

pub fn assert_normalization_failed_contains(
    err: &IngestionError,
    expected_suffix: &str,
    expected_fragment: &str,
) {
    if let IngestionError::NormalizationFailed { path, reason } = err {
        assert!(path.ends_with(expected_suffix), "{path:?}");
        assert!(
            reason.contains(expected_fragment),
            "expected `{expected_fragment}` in `{reason}`"
        );
    } else {
        assert!(false, "expected normalization failure, got {err:#?}");
    }
}

pub fn assert_pipeline_missing_dependency_allowlist_for_library(results: &[G3CheckResult]) {
    assert!(
        has_result(
            results,
            "RS-DEPS-CONFIG-04",
            "dependency allowlist missing",
            Some("crates/core/Cargo.toml"),
        ),
        "{results:#?}"
    );
}

pub fn assert_pipeline_workspace_tool_presence(results: &[G3CheckResult]) {
    assert_eq!(
        findings(results),
        vec![
            Finding {
                id: "RS-DEPS-CONFIG-06".to_owned(),
                severity: G3Severity::Info,
                title: "cargo-deny installed".to_owned(),
                message: "`cargo-deny` is available on PATH.".to_owned(),
                file: Some("Cargo.toml".to_owned()),
                inventory: true,
            },
            Finding {
                id: "RS-DEPS-CONFIG-07".to_owned(),
                severity: G3Severity::Info,
                title: "cargo-machete installed".to_owned(),
                message: "`cargo-machete` is available on PATH.".to_owned(),
                file: Some("Cargo.toml".to_owned()),
                inventory: true,
            },
            Finding {
                id: "RS-DEPS-CONFIG-08".to_owned(),
                severity: G3Severity::Warn,
                title: "cargo-dupes missing".to_owned(),
                message: "`cargo-dupes` was not found on PATH. Install with `cargo install cargo-dupes`.".to_owned(),
                file: Some("Cargo.toml".to_owned()),
                inventory: false,
            },
            Finding {
                id: "RS-DEPS-CONFIG-09".to_owned(),
                severity: G3Severity::Info,
                title: "gitleaks installed".to_owned(),
                message: "`gitleaks` is available on PATH.".to_owned(),
                file: Some("Cargo.toml".to_owned()),
                inventory: true,
            },
        ],
    );
}

pub fn assert_pipeline_workspace_tool_absence(results: &[G3CheckResult]) {
    assert_eq!(
        findings(results),
        vec![
            Finding {
                id: "RS-DEPS-CONFIG-06".to_owned(),
                severity: G3Severity::Error,
                title: "cargo-deny missing".to_owned(),
                message: "`cargo-deny` was not found on PATH. Install with `cargo install cargo-deny`.".to_owned(),
                file: Some("Cargo.toml".to_owned()),
                inventory: false,
            },
            Finding {
                id: "RS-DEPS-CONFIG-07".to_owned(),
                severity: G3Severity::Error,
                title: "cargo-machete missing".to_owned(),
                message: "`cargo-machete` was not found on PATH. Install with `cargo install cargo-machete`.".to_owned(),
                file: Some("Cargo.toml".to_owned()),
                inventory: false,
            },
            Finding {
                id: "RS-DEPS-CONFIG-08".to_owned(),
                severity: G3Severity::Warn,
                title: "cargo-dupes missing".to_owned(),
                message: "`cargo-dupes` was not found on PATH. Install with `cargo install cargo-dupes`.".to_owned(),
                file: Some("Cargo.toml".to_owned()),
                inventory: false,
            },
            Finding {
                id: "RS-DEPS-CONFIG-09".to_owned(),
                severity: G3Severity::Error,
                title: "gitleaks missing".to_owned(),
                message: "`gitleaks` was not found on PATH. Install with `brew install gitleaks` or download from GitHub.".to_owned(),
                file: Some("Cargo.toml".to_owned()),
                inventory: false,
            },
        ],
    );
}

pub fn assert_filetree_missing_lockfile_for_service(results: &[G3CheckResult]) {
    assert_eq!(
        findings(results),
        vec![
            Finding {
                id: "RS-DEPS-FILETREE-09".to_owned(),
                severity: G3Severity::Error,
                title: "Cargo.lock missing".to_owned(),
                message:
                    "`Cargo.lock` is missing. Run `cargo generate-lockfile` and commit the result."
                        .to_owned(),
                file: Some("Cargo.lock".to_owned()),
                inventory: false,
            },
            Finding {
                id: "RS-DEPS-FILETREE-10".to_owned(),
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

pub fn assert_filetree_missing_lockfile_for_library(results: &[G3CheckResult]) {
    assert_eq!(
        findings(results),
        vec![
            Finding {
                id: "RS-DEPS-FILETREE-09".to_owned(),
                severity: G3Severity::Info,
                title: "Cargo.lock missing".to_owned(),
                message: "Library-profile workspace is missing `Cargo.lock`.".to_owned(),
                file: Some("Cargo.lock".to_owned()),
                inventory: false,
            },
            Finding {
                id: "RS-DEPS-FILETREE-10".to_owned(),
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

pub fn assert_filetree_ignored_lockfile(results: &[G3CheckResult]) {
    assert_eq!(
        findings(results),
        vec![
            Finding {
                id: "RS-DEPS-FILETREE-09".to_owned(),
                severity: G3Severity::Info,
                title: "Cargo.lock committed".to_owned(),
                message: "Workspace root has `Cargo.lock` committed.".to_owned(),
                file: Some("Cargo.lock".to_owned()),
                inventory: true,
            },
            Finding {
                id: "RS-DEPS-FILETREE-10".to_owned(),
                severity: G3Severity::Error,
                title: "Cargo.lock ignored in gitignore".to_owned(),
                message: "`.gitignore` ignores `Cargo.lock`. Remove the line ignoring `Cargo.lock` from this `.gitignore`.".to_owned(),
                file: Some(".gitignore".to_owned()),
                inventory: false,
            },
        ],
    );
}

pub fn assert_filetree_unignored_lockfile(results: &[G3CheckResult]) {
    assert_eq!(
        findings(results),
        vec![
            Finding {
                id: "RS-DEPS-FILETREE-09".to_owned(),
                severity: G3Severity::Info,
                title: "Cargo.lock committed".to_owned(),
                message: "Workspace root has `Cargo.lock` committed.".to_owned(),
                file: Some("Cargo.lock".to_owned()),
                inventory: true,
            },
            Finding {
                id: "RS-DEPS-FILETREE-10".to_owned(),
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
