use std::collections::BTreeSet;

use guardrail3_domain_report::{CheckResult, Severity};

const ID: &str = "RS-CLIPPY-12";

pub fn assert_allowed_files(results: &[CheckResult], expected_files: &[&str]) {
    let inventory_results = results
        .iter()
        .filter(|result| result.inventory())
        .collect::<Vec<_>>();
    let actual_files = inventory_results
        .iter()
        .map(|result| result.file().map(str::to_owned).expect("file"))
        .collect::<BTreeSet<_>>();
    let expected_files = expected_files
        .iter()
        .map(|file| (*file).to_owned())
        .collect::<BTreeSet<_>>();

    assert_eq!(actual_files, expected_files);
    assert_eq!(inventory_results.len(), expected_files.len());
    assert!(inventory_results.iter().all(|result| {
        result.id() == ID
            && result.severity() == Severity::Info
            && result.title() == "clippy.toml placement allowed"
    }));
}

pub fn assert_no_forbidden_results(results: &[CheckResult]) {
    let error_results = results
        .iter()
        .filter(|result| result.id() == ID && !result.inventory())
        .collect::<Vec<_>>();

    assert!(
        error_results.is_empty(),
        "expected no forbidden placement results: {error_results:#?}"
    );
}

pub fn assert_forbidden_files(results: &[CheckResult], expected_files: &[&str]) {
    let error_results = results
        .iter()
        .filter(|result| !result.inventory())
        .collect::<Vec<_>>();
    let actual_files = error_results
        .iter()
        .map(|result| result.file().map(str::to_owned).expect("file"))
        .collect::<BTreeSet<_>>();
    let expected_files = expected_files
        .iter()
        .map(|file| (*file).to_owned())
        .collect::<BTreeSet<_>>();

    assert_eq!(actual_files, expected_files);
    assert_eq!(error_results.len(), expected_files.len());
    assert!(error_results.iter().all(|result| {
        result.id() == ID
            && result.severity() == Severity::Error
            && result.title() == "clippy.toml in forbidden location"
    }));
    assert!(error_results.iter().all(|result| {
        result.message().contains("allowed clippy policy root")
            && result.message().contains("workspace roots")
            && result.message().contains("standalone package roots")
    }));
}

pub fn assert_same_root_conflict(results: &[CheckResult], file: &str, preferred_file: &str) {
    let error_results = results
        .iter()
        .filter(|result| !result.inventory())
        .collect::<Vec<_>>();
    assert_eq!(error_results.len(), 1);
    let result = error_results[0];
    assert_eq!(result.id(), ID);
    assert_eq!(result.severity(), Severity::Error);
    assert_eq!(result.title(), "same-root clippy config conflict");
    assert_eq!(result.file(), Some(file));
    assert_eq!(
        result.message(),
        format!(
            "`{file}` conflicts with `{preferred_file}` at the same policy root. Keep only the highest-precedence clippy config file."
        )
    );
}

pub fn assert_cargo_root_parse_error(results: &[CheckResult], config_file: &str, cargo_file: &str) {
    let error_results = results
        .iter()
        .filter(|result| !result.inventory())
        .collect::<Vec<_>>();
    assert_eq!(error_results.len(), 1);
    let result = error_results[0];
    assert_eq!(result.id(), ID);
    assert_eq!(result.severity(), Severity::Error);
    assert_eq!(
        result.title(),
        "clippy.toml placement could not be resolved"
    );
    assert_eq!(result.file(), Some(config_file));
    assert!(
        result.message().contains(cargo_file),
        "expected cargo path in placement failure: {result:#?}"
    );
    assert!(
        result.message().contains("could not be parsed"),
        "expected parse failure context in placement failure: {result:#?}"
    );
}

pub fn assert_unparseable_cargo_root(results: &[CheckResult], file: &str, cargo_rel: &str) {
    let error_results = results
        .iter()
        .filter(|result| !result.inventory())
        .collect::<Vec<_>>();
    assert_eq!(error_results.len(), 1);
    let result = error_results[0];
    assert_eq!(result.id(), ID);
    assert_eq!(result.severity(), Severity::Error);
    assert_eq!(
        result.title(),
        "clippy.toml placement could not be resolved"
    );
    assert_eq!(result.file(), Some(file));
    assert!(
        result.message().contains(cargo_rel),
        "expected Cargo.toml path in message: {result:#?}"
    );
    assert!(
        result.message().contains("could not be parsed"),
        "expected parse failure context in message: {result:#?}"
    );
}
