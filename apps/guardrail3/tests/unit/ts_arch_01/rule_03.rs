use super::helpers::{
    arch_errors, assert_no_landing, assert_no_packages, assert_no_rust_apps, copy_fixture,
    remove_dir, run_check, write_file,
};

// ============================================================================
// Rule 03: adapters/ and ports/ must contain exactly {inbound, outbound}
// ============================================================================

const STRUCTURAL_DIRS: &[&str] = &[
    "apps/admin/src/modules/adapters",
    "apps/admin/src/modules/ports",
];

#[test]
fn missing_adapters_inbound() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/admin/src/modules/adapters/inbound");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert!(
        errors
            .iter()
            .any(|e| e.title.contains("missing") && e.title.contains("adapters") && e.title.contains("inbound")),
        "expected error about missing adapters/inbound/, got: {errors:#?}"
    );
}

#[test]
fn missing_adapters_outbound() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/admin/src/modules/adapters/outbound");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert!(
        errors
            .iter()
            .any(|e| e.title.contains("missing") && e.title.contains("adapters") && e.title.contains("outbound")),
        "expected error about missing adapters/outbound/, got: {errors:#?}"
    );
}

#[test]
fn missing_ports_inbound() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/admin/src/modules/ports/inbound");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert!(
        errors
            .iter()
            .any(|e| e.title.contains("missing") && e.title.contains("ports") && e.title.contains("inbound")),
        "expected error about missing ports/inbound/, got: {errors:#?}"
    );
}

#[test]
fn missing_ports_outbound() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/admin/src/modules/ports/outbound");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert!(
        errors
            .iter()
            .any(|e| e.title.contains("missing") && e.title.contains("ports") && e.title.contains("outbound")),
        "expected error about missing ports/outbound/, got: {errors:#?}"
    );
}

#[test]
fn missing_all_four_inbound_outbound() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/admin/src/modules/adapters/inbound");
    remove_dir(tmp.path(), "apps/admin/src/modules/adapters/outbound");
    remove_dir(tmp.path(), "apps/admin/src/modules/ports/inbound");
    remove_dir(tmp.path(), "apps/admin/src/modules/ports/outbound");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let missing: Vec<_> = errors
        .iter()
        .filter(|e| e.title.contains("missing") && (e.title.contains("inbound") || e.title.contains("outbound")))
        .collect();
    assert_eq!(
        missing.len(),
        4,
        "expected 4 missing inbound/outbound errors, got {}: {missing:#?}",
        missing.len()
    );
}

#[test]
fn unexpected_dir_in_adapters() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/admin/src/modules/adapters/middleware/auth.ts",
        "export function auth() {}",
    );
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert!(
        errors
            .iter()
            .any(|e| e.title.contains("unexpected") && e.title.contains("middleware")),
        "expected error about unexpected 'middleware' in adapters/, got: {errors:#?}"
    );
}

#[test]
fn unexpected_dir_in_ports() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/admin/src/modules/ports/shared/types.ts",
        "export type Shared = {};",
    );
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert!(
        errors
            .iter()
            .any(|e| e.title.contains("unexpected") && e.title.contains("shared")),
        "expected error about unexpected 'shared' in ports/, got: {errors:#?}"
    );
}

#[test]
fn loose_files_in_structural_dirs() {
    let tmp = copy_fixture();
    for dir in STRUCTURAL_DIRS {
        write_file(tmp.path(), &format!("{dir}/mod.ts"), "// stray");
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let loose: Vec<_> = errors.iter().filter(|e| e.title.contains("loose files")).collect();
    assert_eq!(
        loose.len(),
        2,
        "expected 2 loose-file errors (adapters/ + ports/), got {}: {loose:#?}",
        loose.len()
    );
}

#[test]
fn false_positives_excluded() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/admin/src/modules/adapters/inbound");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    assert_no_rust_apps(&errors);
    assert_no_packages(&errors);
    assert_no_landing(&errors);
}
