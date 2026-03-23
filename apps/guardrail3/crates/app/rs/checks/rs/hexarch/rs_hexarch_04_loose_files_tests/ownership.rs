use super::super::super::test_support::{
    INNER_HEX, copy_fixture, empty_dir, errors_by_id, remove_dir, run_family, write_file,
};

#[test]
fn files_only_container_is_owned_by_rule_05_not_rule_04() {
    let tmp = copy_fixture();
    empty_dir(tmp.path(), "apps/devctl/crates/domain");
    write_file(tmp.path(), "apps/devctl/crates/domain/README.md", "# stray");

    let results = run_family(tmp.path());
    let rule_04 = errors_by_id(&results, "RS-HEXARCH-04");
    let rule_05 = errors_by_id(&results, "RS-HEXARCH-05");
    assert!(
        rule_04.is_empty(),
        "rule 04 should not double-fire on files-only container: {rule_04:#?}"
    );
    assert_eq!(
        rule_05.len(),
        1,
        "expected rule 05 to own files-only container: {rule_05:#?}"
    );
    assert!(rule_05[0].message.contains("README.md"));
}

#[test]
fn empty_container_is_owned_by_rule_05_not_rule_04() {
    let tmp = copy_fixture();
    empty_dir(tmp.path(), "apps/devctl/crates/domain");

    let results = run_family(tmp.path());
    let rule_04 = errors_by_id(&results, "RS-HEXARCH-04");
    let rule_05 = errors_by_id(&results, "RS-HEXARCH-05");
    assert!(
        rule_04.is_empty(),
        "rule 04 should stay silent for truly empty containers: {rule_04:#?}"
    );
    assert_eq!(rule_05.len(), 1, "{rule_05:#?}");
    assert_eq!(
        rule_05[0].file.as_deref(),
        Some("apps/devctl/crates/domain")
    );
}

#[test]
fn missing_container_dir_does_not_emit_rule_04_for_that_path() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/devctl/crates/domain");

    let results = run_family(tmp.path());
    let rule_04 = errors_by_id(&results, "RS-HEXARCH-04");
    assert!(
        rule_04
            .iter()
            .all(|error| error.file.as_deref() != Some("apps/devctl/crates/domain")),
        "rule 04 should stay silent for an absent container owned by another rule: {rule_04:#?}"
    );
}

#[test]
fn removing_outer_adapters_parent_does_not_create_nested_rule_04_hits() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/backend/crates/adapters");

    let results = run_family(tmp.path());
    let rule_04 = errors_by_id(&results, "RS-HEXARCH-04");
    assert!(
        rule_04
            .iter()
            .all(|error| !error.file.as_deref().unwrap_or("").starts_with(INNER_HEX)),
        "rule 04 should not double-fire on nested containers destroyed with the outer adapters parent: {rule_04:#?}"
    );
}

#[test]
fn ts_apps_and_packages_stay_out_of_scope() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/admin/src/modules/domain/README.md",
        "# stray",
    );
    write_file(tmp.path(), "packages/shared-types/README.md", "# stray");

    let results = run_family(tmp.path());
    let rule_04 = errors_by_id(&results, "RS-HEXARCH-04");
    assert!(
        rule_04.is_empty(),
        "rule 04 should ignore TS apps and packages entirely: {rule_04:#?}"
    );
}

#[test]
#[cfg(unix)]
fn symlink_only_container_is_owned_by_rule_05_not_rule_04() {
    let tmp = copy_fixture();
    empty_dir(tmp.path(), "apps/devctl/crates/domain");
    std::os::unix::fs::symlink(
        tmp.path().join("apps/devctl/Cargo.toml"),
        tmp.path().join("apps/devctl/crates/domain/README.md"),
    )
    .expect("symlink");

    let results = run_family(tmp.path());
    let rule_04 = errors_by_id(&results, "RS-HEXARCH-04");
    let rule_05 = errors_by_id(&results, "RS-HEXARCH-05");
    assert!(
        rule_04.is_empty(),
        "rule 04 should stay silent for symlink-only containers owned by rule 05: {rule_04:#?}"
    );
    assert_eq!(rule_05.len(), 1, "{rule_05:#?}");
}

#[test]
fn mixed_rule_04_and_rule_05_states_split_ownership_exactly() {
    let tmp = copy_fixture();

    remove_dir(tmp.path(), "apps/backend/crates/app/commands");
    write_file(
        tmp.path(),
        "apps/backend/crates/app/commands",
        "// replaced child dir",
    );

    empty_dir(tmp.path(), "apps/devctl/crates/domain");
    write_file(tmp.path(), "apps/devctl/crates/domain/README.md", "# stray");

    let results = run_family(tmp.path());
    let rule_04 = errors_by_id(&results, "RS-HEXARCH-04");
    let rule_05 = errors_by_id(&results, "RS-HEXARCH-05");

    assert_eq!(rule_04.len(), 1, "{rule_04:#?}");
    assert_eq!(rule_05.len(), 1, "{rule_05:#?}");
    assert_eq!(rule_04[0].file.as_deref(), Some("apps/backend/crates/app"));
    assert_eq!(
        rule_05[0].file.as_deref(),
        Some("apps/devctl/crates/domain")
    );
}

#[test]
fn mixed_root_structural_and_container_loose_files_split_across_neighbor_rules() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "apps/devctl/crates/stray.rs", "// root stray");
    write_file(
        tmp.path(),
        "apps/devctl/crates/adapters/stray.rs",
        "// structural stray",
    );
    write_file(
        tmp.path(),
        "apps/devctl/crates/app/mod.rs",
        "// container stray",
    );

    let results = run_family(tmp.path());
    let rule_02 = errors_by_id(&results, "RS-HEXARCH-02");
    let rule_03 = errors_by_id(&results, "RS-HEXARCH-03");
    let rule_04 = errors_by_id(&results, "RS-HEXARCH-04");

    assert_eq!(rule_02.len(), 1, "{rule_02:#?}");
    assert_eq!(rule_03.len(), 1, "{rule_03:#?}");
    assert_eq!(rule_04.len(), 1, "{rule_04:#?}");
    assert_eq!(rule_02[0].file.as_deref(), Some("apps/devctl/crates"));
    assert_eq!(
        rule_03[0].file.as_deref(),
        Some("apps/devctl/crates/adapters")
    );
    assert_eq!(rule_04[0].file.as_deref(), Some("apps/devctl/crates/app"));
}
