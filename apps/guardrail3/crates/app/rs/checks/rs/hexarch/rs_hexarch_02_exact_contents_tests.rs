use super::super::test_support::{
    INNER_HEX, RUST_APPS, assert_no_error, copy_fixture, errors_by_id, remove_dir, run_family,
    write_file,
};

#[test]
fn golden_has_no_rule_02_errors() {
    let tmp = copy_fixture();
    let results = run_family(tmp.path());
    assert_no_error(&results, "RS-HEXARCH-02");
}

#[test]
fn missing_domain_everywhere_hits_outer_apps_and_inner_hex() {
    let tmp = copy_fixture();
    for dir in [
        "apps/devctl/crates",
        "apps/backend/crates",
        "apps/worker/crates",
        INNER_HEX,
    ] {
        remove_dir(tmp.path(), &format!("{dir}/domain"));
    }

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-02");
    assert_eq!(errors.len(), 4, "expected missing domain at 4 hex roots: {errors:#?}");
    for error in &errors {
        assert!(error.title.contains("missing"));
        assert!(error.title.contains("domain/"));
        assert!(error.file.as_deref().unwrap_or("").contains("crates"));
    }
    for app in RUST_APPS {
        assert!(
            errors.iter().any(|error| error.title.contains(app)),
            "missing outer app error for {app}: {errors:#?}"
        );
    }
    assert!(
        errors
            .iter()
            .any(|error| error.file.as_deref().unwrap_or("").contains("mcp/crates")),
        "missing inner hex error: {errors:#?}"
    );
}

#[test]
fn crates_with_only_gitkeep_passes_rule_01_and_fails_rule_02_for_missing_dirs() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/devctl/crates");
    write_file(tmp.path(), "apps/devctl/crates/.gitkeep", "");

    let results = run_family(tmp.path());
    let rule_01 = errors_by_id(&results, "RS-HEXARCH-01");
    let rule_02 = errors_by_id(&results, "RS-HEXARCH-02");
    assert!(rule_01.is_empty(), "rule 01 should treat .gitkeep as present: {rule_01:#?}");
    let devctl_rule_02: Vec<_> = rule_02
        .into_iter()
        .filter(|error| error.file.as_deref() == Some("apps/devctl/crates"))
        .collect();
    assert_eq!(
        devctl_rule_02.len(),
        4,
        "expected four missing top-level dirs for devctl: {devctl_rule_02:#?}"
    );
}

#[test]
fn unexpected_top_level_dir_is_error() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "apps/devctl/crates/misc/.gitkeep", "");

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-02");
    assert_eq!(errors.len(), 1, "expected one unexpected-dir error: {errors:#?}");
    assert!(errors[0].title.contains("unexpected directory crates/misc/"));
    assert_eq!(errors[0].file.as_deref(), Some("apps/devctl/crates/misc"));
}

#[test]
fn optional_macros_dir_is_allowed() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "apps/devctl/crates/macros/.gitkeep", "");

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-02");
    assert!(
        errors.is_empty(),
        "optional crates/macros/ should not trigger rule 02: {errors:#?}"
    );
}
