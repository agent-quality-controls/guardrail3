use std::collections::BTreeSet;
const FIXTURE: crate::test_support::HexarchFixture = crate::test_support::HexarchFixture;

fn inner_hex() -> &'static str {
    FIXTURE.inner_hex_root()
}

use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_04_loose_files as assertions;
use crate::test_support::{copy_fixture, remove_dir, write_file};

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

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-04");
    assert!(
        errors.is_empty(),
        "expected .gitkeep-only containers to stay clean for rule 04, got: {errors:#?}"
    );
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

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-04");
    assert_eq!(
        errors.len(),
        replacements.len(),
        "expected one loose-file hit per gitkeep-protected replacement: {errors:#?}"
    );

    let actual_files = errors
        .iter()
        .filter_map(|error| error.file.clone())
        .collect::<BTreeSet<_>>();
    let expected_files = replacements
        .iter()
        .map(|(container, _)| (*container).to_owned())
        .collect::<BTreeSet<_>>();

    assert_eq!(
        actual_files, expected_files,
        "gitkeep-protected replacements should still be owned by rule 04: {errors:#?}"
    );
    for error in &errors {
        let bad_files_section = error
            .message
            .split("that don't belong: ")
            .nth(1)
            .and_then(|s| s.split(". Only").next())
            .unwrap_or("");
        assert!(
            !bad_files_section.contains(".gitkeep"),
            ".gitkeep must not be reported as a loose file: '{bad_files_section}'"
        );
    }
}
