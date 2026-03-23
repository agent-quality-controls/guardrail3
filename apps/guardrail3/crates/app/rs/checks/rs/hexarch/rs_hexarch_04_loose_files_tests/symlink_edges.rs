use super::super::super::test_support::{INNER_HEX, copy_fixture, errors_by_id, run_family};

#[test]
fn symlinked_gitkeep_is_not_treated_as_the_allowed_real_gitkeep_in_outer_container() {
    let tmp = copy_fixture();
    std::os::unix::fs::symlink(
        tmp.path().join("apps/devctl/Cargo.toml"),
        tmp.path().join("apps/devctl/crates/app/.gitkeep"),
    )
    .expect("symlink");

    let results = run_family(tmp.path());
    let devctl_rule_04: Vec<_> = errors_by_id(&results, "RS-HEXARCH-04")
        .into_iter()
        .filter(|error| error.file.as_deref() == Some("apps/devctl/crates/app"))
        .collect();

    assert_eq!(devctl_rule_04.len(), 1, "{devctl_rule_04:#?}");
    assert!(
        devctl_rule_04[0].title.contains("loose files"),
        "{devctl_rule_04:#?}"
    );
    assert!(
        devctl_rule_04[0].message.contains(".gitkeep"),
        "{devctl_rule_04:#?}"
    );
}

#[test]
fn symlinked_gitkeep_is_not_treated_as_the_allowed_real_gitkeep_in_nested_container() {
    let tmp = copy_fixture();
    std::os::unix::fs::symlink(
        tmp.path().join(format!("{INNER_HEX}/app/handlers")),
        tmp.path().join(format!("{INNER_HEX}/app/.gitkeep")),
    )
    .expect("symlink");

    let results = run_family(tmp.path());
    let nested_rule_04: Vec<_> = errors_by_id(&results, "RS-HEXARCH-04")
        .into_iter()
        .filter(|error| error.file.as_deref() == Some(&format!("{INNER_HEX}/app")))
        .collect();

    assert_eq!(nested_rule_04.len(), 1, "{nested_rule_04:#?}");
    assert!(
        nested_rule_04[0].title.contains("loose files"),
        "{nested_rule_04:#?}"
    );
    assert!(
        nested_rule_04[0].message.contains(".gitkeep"),
        "{nested_rule_04:#?}"
    );
}

#[test]
fn loose_non_gitkeep_symlink_is_reported_as_a_bad_file() {
    let tmp = copy_fixture();
    std::os::unix::fs::symlink(
        tmp.path().join("apps/devctl/crates/app/core"),
        tmp.path().join("apps/devctl/crates/app/mod.rs"),
    )
    .expect("symlink");

    let results = run_family(tmp.path());
    let devctl_rule_04: Vec<_> = errors_by_id(&results, "RS-HEXARCH-04")
        .into_iter()
        .filter(|error| error.file.as_deref() == Some("apps/devctl/crates/app"))
        .collect();

    assert_eq!(devctl_rule_04.len(), 1, "{devctl_rule_04:#?}");
    assert!(
        devctl_rule_04[0].message.contains("mod.rs"),
        "{devctl_rule_04:#?}"
    );
}
