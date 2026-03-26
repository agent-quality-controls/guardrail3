use std::collections::BTreeSet;
const FIXTURE: crate::test_support::HexarchFixture = crate::test_support::HexarchFixture;

fn inner_hex() -> &'static str {
    FIXTURE.inner_hex_root()
}

use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_02_exact_contents as assertions;
use crate::test_support::{copy_fixture, write_file};

#[test]
fn unexpected_utils_hits_all_owned_outer_and_nested_hex_roots() {
    let tmp = copy_fixture();
    for dir in [
        "apps/devctl/crates",
        "apps/backend/crates",
        "apps/worker/crates",
        inner_hex(),
    ] {
        write_file(tmp.path(), &format!("{dir}/utils/.gitkeep"), "");
    }

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-02");
    assert_eq!(
        errors.len(),
        4,
        "expected one unexpected-utils hit per owned root: {errors:#?}"
    );
    let actual_files = errors
        .iter()
        .filter_map(|error| error.file.clone())
        .collect::<BTreeSet<_>>();
    let expected_files = [
        "apps/devctl/crates/utils".to_owned(),
        "apps/backend/crates/utils".to_owned(),
        "apps/worker/crates/utils".to_owned(),
        format!("{}/utils", inner_hex()),
    ]
    .into_iter()
    .collect::<BTreeSet<_>>();
    assert_eq!(
        actual_files, expected_files,
        "unexpected hit set: {errors:#?}"
    );
}

#[test]
fn unexpected_dir_inner_hex_only_hits_only_the_nested_hex_root() {
    let tmp = copy_fixture();
    write_file(tmp.path(), &format!("{}/utils/.gitkeep", inner_hex()), "");

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-02");
    assert_eq!(
        errors.len(),
        1,
        "expected one nested unexpected-dir hit: {errors:#?}"
    );
    let expected = format!("{}/utils", inner_hex());
    assert_eq!(
        errors[0].file.as_deref(),
        Some(expected.as_str()),
        "{errors:#?}"
    );
}

#[test]
fn multiple_unexpected_dirs_hit_each_owned_root_once_per_dir() {
    let tmp = copy_fixture();
    for dir in [
        "apps/devctl/crates",
        "apps/backend/crates",
        "apps/worker/crates",
        inner_hex(),
    ] {
        write_file(tmp.path(), &format!("{dir}/utils/.gitkeep"), "");
        write_file(tmp.path(), &format!("{dir}/helpers/.gitkeep"), "");
        write_file(tmp.path(), &format!("{dir}/config/.gitkeep"), "");
    }

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-02");
    assert_eq!(
        errors.len(),
        12,
        "expected three unexpected-dir hits per owned root: {errors:#?}"
    );
}

#[test]
fn near_miss_required_dir_names_are_unexpected_everywhere() {
    let tmp = copy_fixture();
    for dir in [
        "apps/devctl/crates",
        "apps/backend/crates",
        "apps/worker/crates",
        inner_hex(),
    ] {
        for name in ["domains", "adapter", "port", "application"] {
            write_file(tmp.path(), &format!("{dir}/{name}/.gitkeep"), "");
        }
    }

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-02");
    assert_eq!(
        errors.len(),
        16,
        "expected one unexpected-dir hit per near-miss name and owned root: {errors:#?}"
    );
}

#[test]
fn gitkeep_directory_is_unexpected_not_an_allowed_gitkeep_file() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/devctl/crates/.gitkeep/nested.txt",
        "not allowed",
    );

    let results = assertions::run_family(tmp.path());
    let devctl_rule_02: Vec<_> = assertions::errors_by_id(&results, "RS-HEXARCH-02")
        .into_iter()
        .filter(|error| error.file.as_deref() == Some("apps/devctl/crates/.gitkeep"))
        .collect();

    assert_eq!(devctl_rule_02.len(), 1, "{devctl_rule_02:#?}");
    assert!(
        devctl_rule_02[0].title.contains("unexpected"),
        "{devctl_rule_02:#?}"
    );
}
