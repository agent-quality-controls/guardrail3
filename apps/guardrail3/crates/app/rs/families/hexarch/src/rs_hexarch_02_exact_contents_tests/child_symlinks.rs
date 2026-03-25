use super::super::super::test_support::{
    INNER_HEX, copy_fixture, errors_by_id, remove_dir, run_family,
};

#[test]
fn required_child_symlink_to_valid_directory_hits_missing_and_loose_for_that_root() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/devctl/crates/domain");
    std::os::unix::fs::symlink(
        tmp.path().join("apps/devctl/crates/app"),
        tmp.path().join("apps/devctl/crates/domain"),
    )
    .expect("symlink");

    let results = run_family(tmp.path());
    let devctl_rule_02: Vec<_> = errors_by_id(&results, "RS-HEXARCH-02")
        .into_iter()
        .filter(|error| {
            error
                .file
                .as_deref()
                .is_some_and(|file| file.starts_with("apps/devctl/crates"))
        })
        .collect();
    assert_eq!(devctl_rule_02.len(), 2, "{devctl_rule_02:#?}");
    assert!(
        devctl_rule_02
            .iter()
            .any(|error| error.title.contains("missing") && error.title.contains("domain/"))
    );
    assert!(
        devctl_rule_02
            .iter()
            .any(|error| error.title.contains("loose files"))
    );
}

#[test]
fn required_child_broken_symlink_hits_missing_and_loose_for_that_root() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/devctl/crates/domain");
    std::os::unix::fs::symlink(
        "/nonexistent/path",
        tmp.path().join("apps/devctl/crates/domain"),
    )
    .expect("symlink");

    let results = run_family(tmp.path());
    let devctl_rule_02: Vec<_> = errors_by_id(&results, "RS-HEXARCH-02")
        .into_iter()
        .filter(|error| {
            error
                .file
                .as_deref()
                .is_some_and(|file| file.starts_with("apps/devctl/crates"))
        })
        .collect();
    assert_eq!(devctl_rule_02.len(), 2, "{devctl_rule_02:#?}");
    assert!(
        devctl_rule_02
            .iter()
            .any(|error| error.title.contains("missing") && error.title.contains("domain/"))
    );
    assert!(
        devctl_rule_02
            .iter()
            .any(|error| error.title.contains("loose files"))
    );
}

#[test]
fn required_child_dev_null_symlink_hits_missing_and_loose_for_that_root() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/devctl/crates/domain");
    std::os::unix::fs::symlink("/dev/null", tmp.path().join("apps/devctl/crates/domain"))
        .expect("symlink");

    let results = run_family(tmp.path());
    let devctl_rule_02: Vec<_> = errors_by_id(&results, "RS-HEXARCH-02")
        .into_iter()
        .filter(|error| {
            error
                .file
                .as_deref()
                .is_some_and(|file| file.starts_with("apps/devctl/crates"))
        })
        .collect();
    assert_eq!(devctl_rule_02.len(), 2, "{devctl_rule_02:#?}");
}

#[test]
fn nested_required_child_valid_symlink_hits_missing_and_loose_for_that_root() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), &format!("{INNER_HEX}/domain"));
    std::os::unix::fs::symlink(
        tmp.path().join(format!("{INNER_HEX}/app")),
        tmp.path().join(format!("{INNER_HEX}/domain")),
    )
    .expect("symlink");

    let results = run_family(tmp.path());
    let nested_rule_02: Vec<_> = errors_by_id(&results, "RS-HEXARCH-02")
        .into_iter()
        .filter(|error| {
            error
                .file
                .as_deref()
                .is_some_and(|file| file.starts_with(INNER_HEX))
        })
        .collect();
    assert_eq!(nested_rule_02.len(), 2, "{nested_rule_02:#?}");
    assert!(
        nested_rule_02
            .iter()
            .any(|error| error.title.contains("missing") && error.title.contains("domain/"))
    );
    assert!(
        nested_rule_02
            .iter()
            .any(|error| error.title.contains("loose files"))
    );
}

#[test]
fn required_child_symlink_hits_every_owned_root_for_non_special_required_name() {
    let tmp = copy_fixture();
    for dir in [
        "apps/devctl/crates",
        "apps/backend/crates",
        "apps/worker/crates",
        INNER_HEX,
    ] {
        remove_dir(tmp.path(), &format!("{dir}/domain"));
        std::os::unix::fs::symlink(
            tmp.path().join(format!("{dir}/app")),
            tmp.path().join(format!("{dir}/domain")),
        )
        .expect("symlink");
    }

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-02");
    assert_eq!(errors.len(), 8, "{errors:#?}");
    assert_eq!(
        errors
            .iter()
            .filter(|error| error.title.contains("missing") && error.title.contains("domain/"))
            .count(),
        4,
        "{errors:#?}"
    );
    assert_eq!(
        errors
            .iter()
            .filter(|error| error.title.contains("loose files"))
            .count(),
        4,
        "{errors:#?}"
    );
}

#[test]
fn outer_adapters_symlink_hits_only_outer_roots_because_nested_hex_becomes_unreachable() {
    let tmp = copy_fixture();
    for dir in [
        "apps/devctl/crates",
        "apps/backend/crates",
        "apps/worker/crates",
    ] {
        remove_dir(tmp.path(), &format!("{dir}/adapters"));
        std::os::unix::fs::symlink(
            tmp.path().join(format!("{dir}/app")),
            tmp.path().join(format!("{dir}/adapters")),
        )
        .expect("symlink");
    }

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-02");
    assert_eq!(errors.len(), 6, "{errors:#?}");
    assert_eq!(
        errors
            .iter()
            .filter(|error| error.title.contains("missing") && error.title.contains("adapters/"))
            .count(),
        3,
        "{errors:#?}"
    );
    assert_eq!(
        errors
            .iter()
            .filter(|error| error.title.contains("loose files"))
            .count(),
        3,
        "{errors:#?}"
    );
    assert!(
        errors.iter().all(|error| !error
            .file
            .as_deref()
            .is_some_and(|file| file.starts_with(INNER_HEX))),
        "{errors:#?}"
    );
}
