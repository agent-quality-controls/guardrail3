use std::collections::BTreeSet;
const FIXTURE: super::HexarchFixture = super::HexarchFixture;

fn inner_hex() -> &'static str {
    FIXTURE.inner_hex_root()
}

use super::{copy_fixture, remove_dir, write_file};
use guardrail3_app_rs_family_hexarch_assertions::structure::rs_hexarch_05_container_not_empty as assertions;

const SAFE_SUFFIXES: &[&str] = &[
    "app",
    "domain",
    "adapters/outbound",
    "ports/inbound",
    "ports/outbound",
];

fn all_safe_owned_container_paths() -> Vec<String> {
    let mut paths = Vec::new();
    for app in ["devctl", "backend", "worker"] {
        for suffix in SAFE_SUFFIXES {
            paths.push(format!("apps/{app}/crates/{suffix}"));
        }
    }
    for suffix in SAFE_SUFFIXES {
        paths.push(format!("{}/{}", inner_hex(), suffix));
    }
    paths
}

#[test]
fn replacing_container_dirs_with_files_hits_all_owned_app_roots() {
    let tmp = copy_fixture();
    let paths = vec![
        "apps/devctl/crates/app".to_owned(),
        "apps/backend/crates/app".to_owned(),
        "apps/worker/crates/app".to_owned(),
        format!("{}/app", inner_hex()),
    ];
    for path in &paths {
        remove_dir(tmp.path(), path);
        write_file(tmp.path(), path, "not a directory");
    }

    let results = super::run_family(tmp.path());
    let expected = paths
        .iter()
        .map(|file| assertions::ExpectedRuleResult {
            file: Some(file.as_str()),
            file_contains: None,
            title_contains: None,
            message_contains: Some(&["is empty"]),
        })
        .collect::<Vec<_>>();

    assertions::assert_expected_rule_results(&results, "", &expected);
}

#[test]
fn replacing_nested_adapters_inbound_with_a_file_hits_nested_root_only() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), &format!("{}/adapters/inbound", inner_hex()));
    write_file(
        tmp.path(),
        &format!("{}/adapters/inbound", inner_hex()),
        "not a directory",
    );

    let results = super::run_family(tmp.path());
    assertions::assert_expected_rule_results(
        &results,
        "",
        &[assertions::ExpectedRuleResult {
            file: Some("apps/backend/crates/adapters/inbound/mcp/crates/adapters/inbound"),
            file_contains: None,
            title_contains: None,
            message_contains: None,
        }],
    );
}

#[test]
fn replacing_all_owned_safe_containers_with_files_hits_every_owned_container() {
    let tmp = copy_fixture();
    let expected_files = all_safe_owned_container_paths()
        .into_iter()
        .collect::<BTreeSet<_>>();
    for path in &expected_files {
        remove_dir(tmp.path(), path);
        write_file(tmp.path(), path, "not a directory");
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
                message_contains: Some(&["is empty"]),
            })
            .collect::<Vec<_>>(),
    );
}
