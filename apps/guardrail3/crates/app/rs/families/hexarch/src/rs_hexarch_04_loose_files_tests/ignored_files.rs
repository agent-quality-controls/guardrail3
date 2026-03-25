use super::super::super::test_support::{copy_fixture, errors_by_id, run_family, write_file};

#[test]
fn ignored_untracked_loose_files_still_hit_rule_04() {
    let tmp = copy_fixture();
    write_file(tmp.path(), ".gitignore", "*.env\n");
    write_file(tmp.path(), "apps/devctl/crates/app/.env", "SECRET=1\n");

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-04");
    assert_eq!(errors.len(), 1, "{errors:#?}");
    assert_eq!(errors[0].file.as_deref(), Some("apps/devctl/crates/app"));
    assert!(errors[0].message.contains(".env"), "{errors:#?}");
}

#[test]
#[cfg(unix)]
fn ignored_symlink_loose_files_still_hit_rule_04() {
    let tmp = copy_fixture();
    write_file(tmp.path(), ".gitignore", "*.env\n");
    std::os::unix::fs::symlink(
        tmp.path().join("apps/devctl/Cargo.toml"),
        tmp.path().join("apps/devctl/crates/app/.env"),
    )
    .expect("symlink");

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-04");
    assert_eq!(errors.len(), 1, "{errors:#?}");
    assert_eq!(errors[0].file.as_deref(), Some("apps/devctl/crates/app"));
    assert!(errors[0].message.contains(".env"), "{errors:#?}");
}

#[test]
#[cfg(unix)]
fn ignored_broken_symlink_loose_files_still_hit_rule_04() {
    let tmp = copy_fixture();
    write_file(tmp.path(), ".gitignore", "*.env\n");
    std::os::unix::fs::symlink(
        tmp.path().join("missing-target"),
        tmp.path().join("apps/devctl/crates/app/.env"),
    )
    .expect("symlink");

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-04");
    assert_eq!(errors.len(), 1, "{errors:#?}");
    assert_eq!(errors[0].file.as_deref(), Some("apps/devctl/crates/app"));
    assert!(errors[0].message.contains(".env"), "{errors:#?}");
}
