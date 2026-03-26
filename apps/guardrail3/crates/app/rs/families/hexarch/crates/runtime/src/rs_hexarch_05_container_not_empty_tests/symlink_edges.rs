use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_05_container_not_empty as assertions;
use super::{copy_fixture, empty_dir};

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

    let results = super::run_family(tmp.path());
    assertions::assert_expected_rule_results(
        &results,
        "",
        &[assertions::ExpectedRuleResult {
            file: Some(container),
            file_contains: None,
            title_contains: None,
            message_contains: Some(&["contains files", "link"]),
        }],
    );
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

    let results = super::run_family(tmp.path());
    assertions::assert_expected_rule_results(
        &results,
        "",
        &[assertions::ExpectedRuleResult {
            file: Some(container),
            file_contains: None,
            title_contains: None,
            message_contains: Some(&["contains files", "dangling"]),
        }],
    );
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

    let results = super::run_family(tmp.path());
    assertions::assert_expected_rule_results(
        &results,
        "",
        &[assertions::ExpectedRuleResult {
            file: Some(container),
            file_contains: None,
            title_contains: None,
            message_contains: Some(&["contains files", "types"]),
        }],
    );

    assertions::assert_no_error_file_contains(&results, "", "apps/devctl/crates/app/types");
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

    let results = super::run_family(tmp.path());
    assertions::assert_expected_rule_results(
        &results,
        "",
        &[assertions::ExpectedRuleResult {
            file: Some(container),
            file_contains: None,
            title_contains: None,
            message_contains: Some(&["contains files", ".gitkeep"]),
        }],
    );
}
