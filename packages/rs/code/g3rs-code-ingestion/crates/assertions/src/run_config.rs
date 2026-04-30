#![allow(
    clippy::expect_used,
    clippy::missing_const_for_fn,
    clippy::missing_panics_doc,
    reason = "assertion helpers are reusable panic-based proof sites for test harnesses"
)]

use cargo_toml_parser::types::LintValue;
use g3rs_code_types as code_types;
use guardrail3_check_types::{G3CheckResult, G3Severity};

pub fn require_config_file<'a>(
    input: &'a code_types::G3RsCodeConfigChecksInput,
    rel_path: &str,
) -> &'a code_types::G3RsCodeConfigFile {
    let index = input
        .files
        .iter()
        .position(|file| file.rel_path == rel_path);
    assert!(
        index.is_some(),
        "missing config file {rel_path}; input: {input:#?}"
    );
    &input.files[index.unwrap_or(0)]
}

pub fn assert_exception_comments(
    input: &code_types::G3RsCodeConfigChecksInput,
    rel_path: &str,
    expected: &[(usize, &str)],
) {
    let comments = input
        .exception_comments
        .iter()
        .filter(|comment| comment.rel_path == rel_path)
        .collect::<Vec<&code_types::G3RsCodeExceptionComment>>();

    assert_eq!(comments.len(), expected.len(), "{input:#?}");
    for (comment, (line, text)) in comments.iter().zip(expected.iter()) {
        assert_eq!(comment.line, *line, "{input:#?}");
        assert_eq!(comment.text, *text, "{input:#?}");
    }
}

pub fn assert_parser_backed_config_files_only(input: &code_types::G3RsCodeConfigChecksInput) {
    assert!(
        input.files.iter().all(|file| {
            matches!(
                file.kind,
                code_types::G3RsCodeConfigFileKind::Guardrail3RsToml { .. }
                    | code_types::G3RsCodeConfigFileKind::ClippyToml { .. }
                    | code_types::G3RsCodeConfigFileKind::DenyToml { .. }
                    | code_types::G3RsCodeConfigFileKind::CargoToml { .. }
                    | code_types::G3RsCodeConfigFileKind::RustfmtToml { .. }
                    | code_types::G3RsCodeConfigFileKind::RustToolchainToml { .. }
            )
        }),
        "code config inputs must not expose raw Text files: {input:#?}"
    );
}

pub fn assert_workspace_unsafe_code_level(
    input: &code_types::G3RsCodeConfigChecksInput,
    rel_path: &str,
    expected: &str,
) {
    let file = require_config_file(input, rel_path);
    assert!(
        matches!(
            file.kind,
            code_types::G3RsCodeConfigFileKind::CargoToml { .. }
        ),
        "expected Cargo.toml config file in {rel_path}"
    );
    let code_types::G3RsCodeConfigFileKind::CargoToml { cargo } = &file.kind else {
        return;
    };
    assert_eq!(
        cargo
            .workspace
            .as_ref()
            .and_then(|workspace| workspace.lints.as_ref())
            .and_then(|lints| lints.tools.get("rust"))
            .and_then(|tool| tool.get("unsafe_code")),
        Some(&LintValue::Level(expected.to_owned()))
    );
}

pub fn assert_workspace_unsafe_code_detailed_level(
    input: &code_types::G3RsCodeConfigChecksInput,
    rel_path: &str,
    expected: &str,
) {
    let file = require_config_file(input, rel_path);
    assert!(
        matches!(
            file.kind,
            code_types::G3RsCodeConfigFileKind::CargoToml { .. }
        ),
        "expected Cargo.toml config file in {rel_path}"
    );
    let code_types::G3RsCodeConfigFileKind::CargoToml { cargo } = &file.kind else {
        return;
    };
    let value = cargo
        .workspace
        .as_ref()
        .and_then(|workspace| workspace.lints.as_ref())
        .and_then(|lints| lints.tools.get("rust"))
        .and_then(|tool| tool.get("unsafe_code"));
    match value {
        Some(LintValue::Detailed(detail)) => assert_eq!(detail.level, expected),
        _ => assert!(false, "expected detailed unsafe_code lint in {rel_path}"),
    }
}

pub fn assert_config_file_paths(input: &code_types::G3RsCodeConfigChecksInput, expected: &[&str]) {
    let actual = input
        .files
        .iter()
        .map(|file| file.rel_path.as_str())
        .collect::<Vec<_>>();
    assert_eq!(actual, expected, "{input:#?}");
}

pub fn assert_config_pipeline_reports_exception_comments_and_unsafe_code_lints(
    results: &[G3CheckResult],
) {
    assert_has_result_id(
        results,
        "Cargo.toml",
        "g3rs-code/exception-comment-inventory",
    );
    assert_has_result_id_with_severity(
        results,
        "Cargo.toml",
        "g3rs-code/unsafe-code-lint",
        G3Severity::Info,
    );
    assert_has_result_id(
        results,
        "deny.toml",
        "g3rs-code/exception-comment-inventory",
    );
}

pub fn assert_config_pipeline_stays_clean_for_harmless_comments_and_non_workspace_manifests(
    results: &[G3CheckResult],
) {
    assert_result_count(results, 1);
    assert_result_set(
        results,
        "crates/core/Cargo.toml",
        1,
        &["g3rs-code/exception-comment-inventory"],
    );
    for file in ["Cargo.toml", "deny.toml"] {
        assert_no_results_for_file(results, file);
    }
}

pub fn assert_config_pipeline_reports_exact_exception_comment_counts(results: &[G3CheckResult]) {
    assert_result_count(results, 3);
    for (file, count) in [("Cargo.toml", 2), ("deny.toml", 1)] {
        assert_file_result_count(results, file, count);
    }
}

pub fn assert_config_pipeline_ignores_foreign_nested_repo_findings(results: &[G3CheckResult]) {
    assert_result_count(results, 1);
    assert_result_set(results, "Cargo.toml", 1, &["g3rs-code/unsafe-code-lint"]);
    for file in ["vendor/foreign/Cargo.toml", "vendor/foreign/deny.toml"] {
        assert_no_results_for_file(results, file);
    }
}

pub fn assert_config_pipeline_reports_deny_through_full_lane(results: &[G3CheckResult]) {
    assert_result_count(results, 1);
    assert_has_result_id_with_severity(
        results,
        "Cargo.toml",
        "g3rs-code/unsafe-code-lint",
        G3Severity::Error,
    );
}

fn assert_file_result_count(results: &[G3CheckResult], file: &str, expected: usize) {
    let actual = results
        .iter()
        .filter(|result| result.file() == Some(file))
        .count();
    assert_eq!(actual, expected, "{results:#?}");
}

fn assert_has_result_id(results: &[G3CheckResult], file: &str, id: &str) {
    assert!(
        results
            .iter()
            .any(|result| result.file() == Some(file) && result.id() == id),
        "{results:#?}"
    );
}

fn assert_has_result_id_with_severity(
    results: &[G3CheckResult],
    file: &str,
    id: &str,
    severity: G3Severity,
) {
    assert!(
        results.iter().any(|result| {
            result.file() == Some(file) && result.id() == id && result.severity() == severity
        }),
        "{results:#?}"
    );
}

fn assert_no_results_for_file(results: &[G3CheckResult], file: &str) {
    assert!(
        !results.iter().any(|result| result.file() == Some(file)),
        "{results:#?}"
    );
}

fn assert_result_count(results: &[G3CheckResult], expected: usize) {
    assert_eq!(results.len(), expected, "{results:#?}");
}

fn assert_result_set(results: &[G3CheckResult], file: &str, count: usize, ids: &[&str]) {
    assert_file_result_count(results, file, count);
    for id in ids {
        assert_has_result_id(results, file, id);
    }
}
