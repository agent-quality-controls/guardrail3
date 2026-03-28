use std::collections::BTreeSet;
const FIXTURE: super::HexarchFixture = super::HexarchFixture;

fn inner_hex() -> &'static str {
    FIXTURE.inner_hex_root()
}

use super::{copy_fixture, remove_dir, write_file};
use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_04_loose_files as assertions;

const CONTAINER_SUFFIXES: &[&str] = &[
    "app",
    "domain",
    "adapters/inbound",
    "adapters/outbound",
    "ports/inbound",
    "ports/outbound",
];

fn all_owned_container_paths() -> Vec<String> {
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

fn replace_single_child_with_gitkeep(root: &std::path::Path, container: &str, child: &str) {
    write_file(root, &format!("{container}/.gitkeep"), "");
    remove_dir(root, &format!("{container}/{child}"));
    write_file(
        root,
        &format!("{container}/{child}"),
        "// replaced child dir",
    );
}

#[test]
fn gitkeep_only_in_all_owned_container_dirs_is_clean() {
    let tmp = copy_fixture();
    for path in all_owned_container_paths() {
        write_file(tmp.path(), &format!("{path}/.gitkeep"), "");
    }

    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "");
}

#[test]
fn gitkeep_plus_replaced_single_child_containers_still_hit_rule_04() {
    let tmp = copy_fixture();
    let replacements = [
        ("apps/devctl/crates/app", "core"),
        ("apps/devctl/crates/domain", "types"),
        ("apps/devctl/crates/adapters/inbound", "cli"),
        ("apps/devctl/crates/adapters/outbound", "fs"),
        ("apps/devctl/crates/ports/outbound", "traits"),
        ("apps/worker/crates/app", "processor"),
        ("apps/worker/crates/domain", "jobs"),
        ("apps/worker/crates/adapters/inbound", "poller"),
        ("apps/worker/crates/ports/outbound", "queue"),
        ("apps/backend/crates/ports/inbound", "api"),
        (
            "apps/backend/crates/adapters/inbound/mcp/crates/app",
            "handlers",
        ),
        (
            "apps/backend/crates/adapters/inbound/mcp/crates/domain",
            "protocol",
        ),
        (
            "apps/backend/crates/adapters/inbound/mcp/crates/adapters/inbound",
            "transport",
        ),
    ];

    for (container, child) in &replacements {
        replace_single_child_with_gitkeep(tmp.path(), container, child);
    }

    let results = super::run_family(tmp.path());
    let expected_files = replacements
        .iter()
        .map(|(container, _)| (*container).to_owned())
        .collect::<BTreeSet<_>>();

    assertions::assert_error_summary(
        &results,
        "",
        replacements.len(),
        &expected_files,
        None,
        Some(&["loose files"]),
        None,
        None,
    );
    assertions::assert_error_summary(
        &results,
        "",
        replacements.len(),
        &expected_files,
        None,
        None,
        None,
        None,
    );
}
