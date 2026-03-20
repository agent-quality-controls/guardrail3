use super::helpers::{
    arch_errors, assert_no_landing, assert_no_packages, assert_no_rust_apps, copy_fixture,
    remove_dir, run_check, write_file,
};

// ============================================================================
// Rule 02: modules/ must contain exactly {domain, ports, application, adapters}
// ============================================================================

const EXPECTED_DIRS: &[&str] = &["domain", "ports", "application", "adapters"];

#[test]
fn missing_domain() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/admin/src/modules/domain");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert!(
        errors.iter().any(|e| e.title.contains("missing") && e.title.contains("domain")),
        "expected error about missing domain/, got: {errors:#?}"
    );
}

#[test]
fn missing_ports() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/admin/src/modules/ports");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert!(
        errors.iter().any(|e| e.title.contains("missing") && e.title.contains("ports")),
        "expected error about missing ports/, got: {errors:#?}"
    );
}

#[test]
fn missing_application() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/admin/src/modules/application");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert!(
        errors.iter().any(|e| e.title.contains("missing") && e.title.contains("application")),
        "expected error about missing application/, got: {errors:#?}"
    );
}

#[test]
fn missing_adapters() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/admin/src/modules/adapters");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert!(
        errors.iter().any(|e| e.title.contains("missing") && e.title.contains("adapters")),
        "expected error about missing adapters/, got: {errors:#?}"
    );
}

#[test]
fn missing_all_four() {
    let tmp = copy_fixture();
    for dir in EXPECTED_DIRS {
        remove_dir(tmp.path(), &format!("apps/admin/src/modules/{dir}"));
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let missing: Vec<_> = errors.iter().filter(|e| e.title.contains("missing")).collect();
    assert_eq!(
        missing.len(),
        4,
        "expected 4 missing-dir errors, got {}: {missing:#?}",
        missing.len()
    );
    for dir in EXPECTED_DIRS {
        assert!(
            missing.iter().any(|e| e.title.contains(dir)),
            "expected error mentioning '{dir}', got: {missing:#?}"
        );
    }
}

#[test]
fn unexpected_directory() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/admin/src/modules/services/handler.ts",
        "export function handle() {}",
    );
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert!(
        errors
            .iter()
            .any(|e| e.title.contains("unexpected") && e.title.contains("services")),
        "expected error about unexpected 'services' directory, got: {errors:#?}"
    );
}

#[test]
fn unexpected_plus_missing() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/admin/src/modules/domain");
    write_file(
        tmp.path(),
        "apps/admin/src/modules/services/handler.ts",
        "export function handle() {}",
    );
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let missing: Vec<_> = errors.iter().filter(|e| e.title.contains("missing") && e.title.contains("domain")).collect();
    let unexpected: Vec<_> = errors.iter().filter(|e| e.title.contains("unexpected") && e.title.contains("services")).collect();
    assert_eq!(missing.len(), 1, "expected 1 missing-domain error: {errors:#?}");
    assert_eq!(unexpected.len(), 1, "expected 1 unexpected-services error: {errors:#?}");
}

#[test]
fn false_positives_excluded() {
    let tmp = copy_fixture();
    // Break admin's modules/ structure
    remove_dir(tmp.path(), "apps/admin/src/modules/domain");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert_no_rust_apps(&errors);
    assert_no_packages(&errors);
    assert_no_landing(&errors);
}

#[test]
fn all_errors_mention_admin() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/admin/src/modules/domain");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    for err in &errors {
        assert!(
            err.title.contains("admin"),
            "expected error to mention 'admin', got: '{}'",
            err.title
        );
    }
}
