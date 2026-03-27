use std::collections::BTreeSet;

use guardrail3_domain_report::{CheckResult, Severity};

const ID: &str = "RS-CLIPPY-12";

pub fn assert_no_results(results: &[CheckResult]) {
    assert!(
        results.is_empty(),
        "expected no placement findings: {results:#?}"
    );
}

pub fn assert_forbidden_files(results: &[CheckResult], expected_files: &[&str]) {
    let actual_files = results
        .iter()
        .map(|result| result.file.clone().expect("file"))
        .collect::<BTreeSet<_>>();
    let expected_files = expected_files
        .iter()
        .map(|file| (*file).to_owned())
        .collect::<BTreeSet<_>>();

    assert_eq!(actual_files, expected_files);
    assert_eq!(results.len(), expected_files.len());
    assert!(results.iter().all(|result| {
        result.id == ID
            && result.severity == Severity::Error
            && result.title == "clippy.toml in forbidden location"
    }));
    assert!(results.iter().all(|result| {
        result.message.contains("allowed clippy policy root")
            && result.message.contains("workspace roots")
            && result.message.contains("standalone package roots")
    }));
}

pub fn assert_same_root_conflict(results: &[CheckResult], file: &str, preferred_file: &str) {
    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, ID);
    assert_eq!(result.severity, Severity::Error);
    assert_eq!(result.title, "same-root clippy config conflict");
    assert_eq!(result.file.as_deref(), Some(file));
    assert_eq!(
        result.message,
        format!(
            "`{file}` conflicts with `{preferred_file}` at the same policy root. Keep only the highest-precedence clippy config file."
        )
    );
}
