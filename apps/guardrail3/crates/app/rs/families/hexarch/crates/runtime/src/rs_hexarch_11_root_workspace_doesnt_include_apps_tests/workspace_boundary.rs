use std::collections::BTreeSet;

use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_11_root_workspace_doesnt_include_apps as assertions;
use crate::test_support::{copy_fixture, write_file};

#[test]
fn root_workspace_including_all_rust_apps_hits_every_owned_app_member() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "Cargo.toml",
        "[workspace]\nmembers = [\"packages/shared-types\", \"apps/devctl\", \"apps/backend\", \"apps/worker\"]\nresolver = \"2\"\n",
    );

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-11");
    let actual_titles = errors
        .iter()
        .map(|error| error.title.clone())
        .collect::<BTreeSet<_>>();
    let expected_titles = [
        "root workspace includes app member `apps/devctl`".to_owned(),
        "root workspace includes app member `apps/backend`".to_owned(),
        "root workspace includes app member `apps/worker`".to_owned(),
    ]
    .into_iter()
    .collect::<BTreeSet<_>>();

    assert_eq!(
        actual_titles, expected_titles,
        "unexpected hit set: {errors:#?}"
    );
    for error in &errors {
        assert_eq!(error.file.as_deref(), Some("Cargo.toml"));
    }
}

#[test]
fn normalized_root_workspace_app_member_still_hits_rule_11() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "Cargo.toml",
        "[workspace]\nmembers = [\"packages/shared-types\", \"./apps/devctl/\"]\nresolver = \"2\"\n",
    );

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-11");

    assert_eq!(
        errors.len(),
        1,
        "expected one normalized app-member hit: {errors:#?}"
    );
    assert!(errors[0].title.contains("./apps/devctl/"));
}

#[test]
fn absolute_root_workspace_app_member_still_hits_rule_11() {
    let tmp = copy_fixture();
    let absolute_member = tmp.path().join("apps/devctl").display().to_string();
    write_file(
        tmp.path(),
        "Cargo.toml",
        &format!(
            "[workspace]\nmembers = [\"packages/shared-types\", \"{absolute_member}\"]\nresolver = \"2\"\n"
        ),
    );

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-11");

    assert_eq!(
        errors.len(),
        1,
        "expected one absolute app-member hit: {errors:#?}"
    );
    assert!(errors[0].title.contains(&absolute_member));
}

#[test]
fn app_subpath_member_still_hits_rule_11() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "Cargo.toml",
        "[workspace]\nmembers = [\"packages/shared-types\", \"apps/devctl/crates/domain/types\"]\nresolver = \"2\"\n",
    );

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-11");

    assert_eq!(errors.len(), 1, "expected one app-subpath hit: {errors:#?}");
    assert!(errors[0].title.contains("apps/devctl/crates/domain/types"));
}

#[test]
fn root_workspace_glob_covering_apps_hits_every_owned_app_member() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "Cargo.toml",
        "[workspace]\nmembers = [\"packages/shared-types\", \"apps/*\"]\nresolver = \"2\"\n",
    );

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-11");

    assert_eq!(errors.len(), 1, "expected one glob-member hit: {errors:#?}");
    assert!(
        errors
            .iter()
            .all(|error| error.title == "root workspace includes app member `apps/*`")
    );
}

#[test]
fn normalized_package_member_does_not_false_positive_under_rule_11() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "Cargo.toml",
        "[workspace]\nmembers = [\"./packages/shared-types/\"]\nresolver = \"2\"\n",
    );

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-11");

    assert!(
        errors.is_empty(),
        "packages members should stay out of rule 11: {errors:#?}"
    );
}
