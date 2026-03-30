use std::collections::BTreeSet;
const FIXTURE: super::HexarchFixture = super::HexarchFixture;

fn inner_hex() -> &'static str {
    FIXTURE.inner_hex_root()
}

use super::{copy_fixture, empty_dir, write_file};
use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_05_container_not_empty as assertions;

const CONTAINER_SUFFIXES: &[&str] = &[
    "app",
    "domain",
    "adapters/outbound",
    "ports/inbound",
    "ports/outbound",
];

fn all_safe_owned_container_paths() -> Vec<String> {
    let mut paths = Vec::new();
    for app in ["devctl", "backend", "worker"] {
        for suffix in CONTAINER_SUFFIXES {
            paths.push(format!("apps/{app}/crates/{suffix}"));
        }
    }
    for suffix in CONTAINER_SUFFIXES {
        paths.push(format!("{}/{}", inner_hex(), suffix));
    }
    paths
}

#[test]
fn emptying_all_owned_safe_container_dirs_hits_every_owned_container() {
    let tmp = copy_fixture();
    let expected_files = all_safe_owned_container_paths()
        .into_iter()
        .collect::<BTreeSet<_>>();
    for path in &expected_files {
        empty_dir(tmp.path(), path);
    }

    let results = super::run_family(tmp.path());
    assertions::assert_expected_rule_results(
        &results,
        "",
        &expected_files
            .iter()
            .map(|file| assertions::ExpectedRuleResult {
                file: Some(file.as_str()),
                file_contains: None,
                title_contains: Some(&["empty container"]),
                message_contains: Some(&["is empty"]),
            })
            .collect::<Vec<_>>(),
    );
}

#[test]
fn emptying_outer_adapters_inbound_destroys_the_nested_hex_path_and_does_not_double_fire() {
    let tmp = copy_fixture();
    for app in ["devctl", "backend", "worker"] {
        empty_dir(tmp.path(), &format!("apps/{app}/crates/adapters/inbound"));
    }

    let results = super::run_family(tmp.path());
    assertions::assert_expected_rule_results(
        &results,
        "",
        &[
            assertions::ExpectedRuleResult {
                file: Some("apps/devctl/crates/adapters/inbound"),
                file_contains: None,
                title_contains: Some(&["empty container"]),
                message_contains: Some(&["is empty"]),
            },
            assertions::ExpectedRuleResult {
                file: Some("apps/backend/crates/adapters/inbound"),
                file_contains: None,
                title_contains: Some(&["empty container"]),
                message_contains: Some(&["is empty"]),
            },
            assertions::ExpectedRuleResult {
                file: Some("apps/worker/crates/adapters/inbound"),
                file_contains: None,
                title_contains: Some(&["empty container"]),
                message_contains: Some(&["is empty"]),
            },
        ],
    );
}

#[test]
fn emptying_only_inner_hex_containers_hits_inner_hex_and_leaves_outer_apps_clean() {
    let tmp = copy_fixture();
    let expected_files = [
        format!("{}/app", inner_hex()),
        format!("{}/domain", inner_hex()),
        format!("{}/adapters/inbound", inner_hex()),
        format!("{}/adapters/outbound", inner_hex()),
        format!("{}/ports/inbound", inner_hex()),
        format!("{}/ports/outbound", inner_hex()),
    ]
    .into_iter()
    .collect::<BTreeSet<_>>();

    for path in &expected_files {
        empty_dir(tmp.path(), path);
    }

    let results = super::run_family(tmp.path());
    assertions::assert_expected_rule_results(
        &results,
        "",
        &expected_files
            .iter()
            .map(|file| assertions::ExpectedRuleResult {
                file: Some(file.as_str()),
                file_contains: None,
                title_contains: None,
                message_contains: None,
            })
            .collect::<Vec<_>>(),
    );
}

#[test]
fn files_only_all_owned_safe_containers_hit_every_owned_container() {
    let tmp = copy_fixture();
    let expected_files = all_safe_owned_container_paths()
        .into_iter()
        .collect::<BTreeSet<_>>();
    for path in &expected_files {
        empty_dir(tmp.path(), path);
        write_file(tmp.path(), &format!("{path}/README.md"), "# stray");
    }

    let results = super::run_family(tmp.path());
    assertions::assert_expected_rule_results(
        &results,
        "",
        &expected_files
            .iter()
            .map(|file| assertions::ExpectedRuleResult {
                file: Some(file.as_str()),
                file_contains: None,
                title_contains: None,
                message_contains: Some(&["contains files", "README.md"]),
            })
            .collect::<Vec<_>>(),
    );
}
