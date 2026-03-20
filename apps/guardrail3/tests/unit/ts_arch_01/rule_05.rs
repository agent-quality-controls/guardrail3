use super::helpers::{arch_errors, copy_fixture, remove_dir, run_check, write_file};

// ============================================================================
// Rule 05: Container dirs must not be empty (need subdirs, .gitkeep, or content)
// ============================================================================

const CONTAINER_SUFFIXES: &[&str] = &[
    "application",
    "domain",
    "adapters/inbound",
    "adapters/outbound",
    "ports/inbound",
    "ports/outbound",
];

fn empty_container_errors<'a>(
    errors: &'a [&guardrail3::domain::report::CheckResult],
) -> Vec<&'a &'a guardrail3::domain::report::CheckResult> {
    errors.iter().filter(|e| e.title.contains("empty container")).collect()
}

#[test]
fn empty_domain() {
    let tmp = copy_fixture();
    // Remove all contents of domain/
    remove_dir(tmp.path(), "apps/admin/src/modules/domain");
    // Recreate empty dir
    std::fs::create_dir_all(tmp.path().join("apps/admin/src/modules/domain"))
        .expect("create empty domain dir");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let empty = empty_container_errors(&errors);
    assert_eq!(
        empty.len(),
        1,
        "expected 1 empty-container error for domain/, got {}: {empty:#?}",
        empty.len()
    );
    assert!(
        empty[0].title.contains("domain"),
        "expected error to mention 'domain', got: '{}'",
        empty[0].title
    );
}

#[test]
fn empty_application() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/admin/src/modules/application");
    std::fs::create_dir_all(tmp.path().join("apps/admin/src/modules/application"))
        .expect("create empty application dir");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let empty = empty_container_errors(&errors);
    assert_eq!(
        empty.len(),
        1,
        "expected 1 empty-container error for application/, got {}: {empty:#?}",
        empty.len()
    );
}

#[test]
fn empty_all_containers() {
    let tmp = copy_fixture();
    for suffix in CONTAINER_SUFFIXES {
        let path = format!("apps/admin/src/modules/{suffix}");
        remove_dir(tmp.path(), &path);
        std::fs::create_dir_all(tmp.path().join(&path)).expect("create empty dir");
    }
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let empty = empty_container_errors(&errors);
    assert_eq!(
        empty.len(),
        6,
        "expected 6 empty-container errors, got {}: {empty:#?}",
        empty.len()
    );
}

#[test]
fn gitkeep_satisfies_non_empty() {
    let tmp = copy_fixture();
    // Replace domain contents with just .gitkeep
    remove_dir(tmp.path(), "apps/admin/src/modules/domain");
    write_file(tmp.path(), "apps/admin/src/modules/domain/.gitkeep", "");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let empty = empty_container_errors(&errors);
    let domain_empty: Vec<_> = empty.iter().filter(|e| e.title.contains("domain")).collect();
    assert!(
        domain_empty.is_empty(),
        ".gitkeep should satisfy non-empty check for domain/, got: {domain_empty:#?}"
    );
}

#[test]
fn subdir_satisfies_non_empty() {
    let tmp = copy_fixture();
    // Domain already has types/ subdir with .ts files — should pass
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let empty = empty_container_errors(&errors);
    assert!(
        empty.is_empty(),
        "golden should have 0 empty-container errors, got: {empty:#?}"
    );
}

#[test]
fn empty_container_plus_loose_file_separate_errors() {
    let tmp = copy_fixture();
    // Make ports/inbound empty (remove the subdirs)
    remove_dir(tmp.path(), "apps/admin/src/modules/ports/inbound");
    std::fs::create_dir_all(tmp.path().join("apps/admin/src/modules/ports/inbound"))
        .expect("create empty dir");
    let results = run_check(tmp.path());
    let errors = arch_errors(&results);
    let empty = empty_container_errors(&errors);
    assert_eq!(
        empty.len(),
        1,
        "expected empty-container error for ports/inbound, got: {empty:#?}"
    );
}
