use std::collections::BTreeMap;
use std::path::Path;

use guardrail3_domain_report::{CheckResult, Severity};

use super::{
    RustFamilyApplicability, applicability_allows_result, filter_results_for_applicability,
};

fn result(file: Option<&str>) -> CheckResult {
    CheckResult {
        id: "TEST".to_owned(),
        severity: Severity::Error,
        title: "test".to_owned(),
        message: "test".to_owned(),
        file: file.map(str::to_owned),
        line: None,
        inventory: false,
    }
}

fn applicability() -> RustFamilyApplicability {
    RustFamilyApplicability {
        global_enabled: false,
        app_enabled: BTreeMap::from([
            ("apps/enabled".to_owned(), true),
            ("apps/disabled".to_owned(), false),
        ]),
        packages_enabled: Some(true),
        global_only: false,
    }
}

#[test]
fn filters_disabled_app_results_by_path() {
    let filtered = filter_results_for_applicability(
        Path::new("/repo"),
        &applicability(),
        vec![
            result(Some("apps/enabled/Cargo.toml")),
            result(Some("apps/disabled/Cargo.toml")),
            result(Some("packages/lib/Cargo.toml")),
            result(Some("Cargo.toml")),
        ],
    );

    let files = filtered
        .iter()
        .map(|item| item.file.as_deref().unwrap_or("<none>"))
        .collect::<Vec<_>>();
    assert_eq!(
        files,
        vec!["apps/enabled/Cargo.toml", "packages/lib/Cargo.toml"]
    );
}

#[test]
fn allows_absolute_paths_under_enabled_scope() {
    let result = result(Some("/repo/apps/enabled/src/lib.rs"));
    assert!(applicability_allows_result(
        Path::new("/repo"),
        &applicability(),
        &result
    ));
}

#[test]
fn keeps_rootless_results_for_now() {
    let result = result(None);
    assert!(applicability_allows_result(
        Path::new("/repo"),
        &applicability(),
        &result
    ));
}
