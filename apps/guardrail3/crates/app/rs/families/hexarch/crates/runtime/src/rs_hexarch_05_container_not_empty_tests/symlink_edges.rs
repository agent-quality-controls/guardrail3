use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_05_container_not_empty as assertions;
use test_support::{copy_fixture, empty_dir};

#[test]
#[cfg(unix)]
fn symlink_only_container_reports_contains_files_instead_of_empty() {
    let tmp = copy_fixture();
    let container = "apps/devctl/crates/ports/inbound";
    empty_dir(tmp.path(), container);
    std::os::unix::fs::symlink(
        tmp.path().join("apps/devctl/crates/app"),
        tmp.path().join(format!("{container}/link")),
    )
    .expect("symlink");

    let results = assertions::run_family(tmp.path());
    let rule_05 = assertions::errors_by_id(&results, "RS-HEXARCH-05");
    let owned: Vec<_> = rule_05
        .into_iter()
        .filter(|error| error.file.as_deref() == Some(container))
        .collect();

    assert_eq!(owned.len(), 1, "{owned:#?}");
    assert!(owned[0].message.contains("contains files"), "{owned:#?}");
    assert!(owned[0].message.contains("link"), "{owned:#?}");
}

#[test]
#[cfg(unix)]
fn dangling_symlink_only_container_reports_contains_files_instead_of_empty() {
    let tmp = copy_fixture();
    let container = "apps/devctl/crates/ports/inbound";
    empty_dir(tmp.path(), container);
    std::os::unix::fs::symlink(
        "/nonexistent/path/that/does/not/exist",
        tmp.path().join(format!("{container}/dangling")),
    )
    .expect("symlink");

    let results = assertions::run_family(tmp.path());
    let rule_05 = assertions::errors_by_id(&results, "RS-HEXARCH-05");
    let owned: Vec<_> = rule_05
        .into_iter()
        .filter(|error| error.file.as_deref() == Some(container))
        .collect();

    assert_eq!(owned.len(), 1, "{owned:#?}");
    assert!(owned[0].message.contains("contains files"), "{owned:#?}");
    assert!(owned[0].message.contains("dangling"), "{owned:#?}");
}

#[test]
#[cfg(unix)]
fn symlinked_child_directory_does_not_count_as_a_real_subdirectory() {
    let tmp = copy_fixture();
    let container = "apps/devctl/crates/app";
    empty_dir(tmp.path(), container);
    std::os::unix::fs::symlink(
        tmp.path().join("apps/devctl/crates/domain/types"),
        tmp.path().join(format!("{container}/types")),
    )
    .expect("symlink");

    let results = assertions::run_family(tmp.path());
    let rule_05 = assertions::errors_by_id(&results, "RS-HEXARCH-05");
    let owned: Vec<_> = rule_05
        .into_iter()
        .filter(|error| error.file.as_deref() == Some(container))
        .collect();

    assert_eq!(owned.len(), 1, "{owned:#?}");
    assert!(owned[0].message.contains("contains files"), "{owned:#?}");
    assert!(owned[0].message.contains("types"), "{owned:#?}");

    let rule_06 = assertions::errors_by_id(&results, "RS-HEXARCH-06");
    assert!(
        rule_06
            .iter()
            .all(|error| error.file.as_deref() != Some("apps/devctl/crates/app/types")),
        "symlinked child dirs should not materialize fake leaf results: {rule_06:#?}"
    );
}

#[test]
#[cfg(unix)]
fn symlinked_gitkeep_does_not_suppress_rule_05() {
    let tmp = copy_fixture();
    let container = "apps/devctl/crates/domain";
    empty_dir(tmp.path(), container);
    std::os::unix::fs::symlink(
        tmp.path().join("apps/devctl/Cargo.toml"),
        tmp.path().join(format!("{container}/.gitkeep")),
    )
    .expect("symlink");

    let results = assertions::run_family(tmp.path());
    let rule_05 = assertions::errors_by_id(&results, "RS-HEXARCH-05");
    let owned: Vec<_> = rule_05
        .into_iter()
        .filter(|error| error.file.as_deref() == Some(container))
        .collect();

    assert_eq!(owned.len(), 1, "{owned:#?}");
    assert!(owned[0].message.contains("contains files"), "{owned:#?}");
    assert!(owned[0].message.contains(".gitkeep"), "{owned:#?}");
}
