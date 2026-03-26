use std::collections::BTreeSet;
const FIXTURE: crate::test_support::HexarchFixture = crate::test_support::HexarchFixture;

fn inner_hex() -> &'static str {
    FIXTURE.inner_hex_root()
}

use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_02_exact_contents as assertions;
use crate::test_support::{copy_fixture, write_file};

#[test]
fn root_loose_files_hit_each_owned_hex_root_once() {
    let tmp = copy_fixture();
    for dir in [
        "apps/devctl/crates",
        "apps/backend/crates",
        "apps/worker/crates",
        inner_hex(),
    ] {
        write_file(tmp.path(), &format!("{dir}/mod.rs"), "// stray");
    }

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-02");

    assert_eq!(
        errors.len(),
        4,
        "expected one loose-file hit per owned hex root: {errors:#?}"
    );

    let actual_files = errors
        .iter()
        .filter_map(|error| error.file.clone())
        .collect::<BTreeSet<_>>();
    let expected_files = [
        "apps/devctl/crates".to_owned(),
        "apps/backend/crates".to_owned(),
        "apps/worker/crates".to_owned(),
        inner_hex().to_owned(),
    ]
    .into_iter()
    .collect::<BTreeSet<_>>();

    assert_eq!(
        actual_files, expected_files,
        "unexpected root loose-file hit set: {errors:#?}"
    );

    for error in &errors {
        assert!(error.title.contains("loose files in"));
        assert!(error.message.contains("mod.rs"));
    }
}

#[test]
fn root_gitkeep_is_still_allowed() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "apps/devctl/crates/.gitkeep", "");

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-02");

    assert!(
        errors.is_empty(),
        "top-level .gitkeep should remain allowed for rule 02: {errors:#?}"
    );
}

#[test]
fn nested_root_gitkeep_is_still_allowed() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        &format!("{}/.gitkeep", inner_hex()),
        "",
    );

    let results = assertions::run_family(tmp.path());
    let nested_rule_02: Vec<_> = assertions::errors_by_id(&results, "RS-HEXARCH-02")
        .into_iter()
        .filter(|error| error.file.as_deref() == Some(inner_hex()))
        .collect();

    assert!(
        nested_rule_02.is_empty(),
        "nested top-level .gitkeep should remain allowed for rule 02: {nested_rule_02:#?}"
    );
}

#[test]
fn root_gitignore_is_a_loose_file_not_an_allowed_dotfile() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "apps/devctl/crates/.gitignore", "*\n");

    let results = assertions::run_family(tmp.path());
    let devctl_rule_02: Vec<_> = assertions::errors_by_id(&results, "RS-HEXARCH-02")
        .into_iter()
        .filter(|error| error.file.as_deref() == Some("apps/devctl/crates"))
        .collect();

    assert_eq!(devctl_rule_02.len(), 1, "{devctl_rule_02:#?}");
    assert!(
        devctl_rule_02[0].title.contains("loose files"),
        "{devctl_rule_02:#?}"
    );
    assert!(
        devctl_rule_02[0].message.contains(".gitignore"),
        "{devctl_rule_02:#?}"
    );
}

#[test]
fn loose_cargo_toml_at_crates_root_is_still_reported_as_a_bad_file() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/devctl/crates/Cargo.toml",
        "[package]\nname = \"wrong-place\"\nversion = \"0.1.0\"\n",
    );

    let results = assertions::run_family(tmp.path());
    let devctl_rule_02: Vec<_> = assertions::errors_by_id(&results, "RS-HEXARCH-02")
        .into_iter()
        .filter(|error| error.file.as_deref() == Some("apps/devctl/crates"))
        .collect();

    assert_eq!(devctl_rule_02.len(), 1, "{devctl_rule_02:#?}");
    assert!(
        devctl_rule_02[0].title.contains("loose files"),
        "{devctl_rule_02:#?}"
    );
    assert!(
        devctl_rule_02[0].message.contains("Cargo.toml"),
        "{devctl_rule_02:#?}"
    );
}

#[test]
fn symlinked_gitkeep_to_file_is_not_treated_as_the_allowed_real_gitkeep() {
    let tmp = copy_fixture();
    std::os::unix::fs::symlink(
        tmp.path().join("apps/devctl/Cargo.toml"),
        tmp.path().join("apps/devctl/crates/.gitkeep"),
    )
    .expect("symlink");

    let results = assertions::run_family(tmp.path());
    let devctl_rule_02: Vec<_> = assertions::errors_by_id(&results, "RS-HEXARCH-02")
        .into_iter()
        .filter(|error| error.file.as_deref() == Some("apps/devctl/crates"))
        .collect();

    assert_eq!(devctl_rule_02.len(), 1, "{devctl_rule_02:#?}");
    assert!(
        devctl_rule_02[0].title.contains("loose files"),
        "{devctl_rule_02:#?}"
    );
    assert!(
        devctl_rule_02[0].message.contains(".gitkeep"),
        "{devctl_rule_02:#?}"
    );
}

#[test]
fn symlinked_gitkeep_to_directory_is_not_treated_as_the_allowed_real_gitkeep() {
    let tmp = copy_fixture();
    std::os::unix::fs::symlink(
        tmp.path().join("apps/devctl/crates/app"),
        tmp.path().join("apps/devctl/crates/.gitkeep"),
    )
    .expect("symlink");

    let results = assertions::run_family(tmp.path());
    let devctl_rule_02: Vec<_> = assertions::errors_by_id(&results, "RS-HEXARCH-02")
        .into_iter()
        .filter(|error| error.file.as_deref() == Some("apps/devctl/crates"))
        .collect();

    assert_eq!(devctl_rule_02.len(), 1, "{devctl_rule_02:#?}");
    assert!(
        devctl_rule_02[0].title.contains("loose files"),
        "{devctl_rule_02:#?}"
    );
    assert!(
        devctl_rule_02[0].message.contains(".gitkeep"),
        "{devctl_rule_02:#?}"
    );
}
