const FIXTURE: super::HexarchFixture = super::HexarchFixture;
use std::collections::BTreeSet;

fn inner_hex() -> &'static str {
    FIXTURE.inner_hex_root()
}

use super::{copy_fixture, write_file};
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

#[test]
fn loose_files_in_all_owned_container_dirs_hit_every_owned_container() {
    let tmp = copy_fixture();
    let expected_files = all_owned_container_paths()
        .into_iter()
        .collect::<BTreeSet<_>>();
    for path in &expected_files {
        write_file(tmp.path(), &format!("{path}/mod.rs"), "// stray");
    }

    let results = super::run_family(tmp.path());
    assertions::assert_error_summary(
        &results,
        "",
        expected_files.len(),
        expected_files,
        None,
        Some(&["loose files"]),
        None,
        Some(&["mod.rs"]),
    );
}

#[test]
fn multiple_loose_files_in_all_owned_container_dirs_emit_one_error_per_container() {
    let tmp = copy_fixture();
    let expected_files = all_owned_container_paths()
        .into_iter()
        .collect::<BTreeSet<_>>();
    for path in &expected_files {
        write_file(tmp.path(), &format!("{path}/mod.rs"), "// stray");
        write_file(tmp.path(), &format!("{path}/README.md"), "# stray");
    }

    let results = super::run_family(tmp.path());
    assertions::assert_error_summary(
        &results,
        "",
        expected_files.len(),
        expected_files,
        None,
        Some(&["loose files"]),
        None,
        Some(&["mod.rs", "README.md"]),
    );
}

#[test]
fn near_miss_placeholder_files_hit_every_owned_container() {
    let tmp = copy_fixture();
    let expected_files = all_owned_container_paths()
        .into_iter()
        .collect::<BTreeSet<_>>();
    for path in &expected_files {
        write_file(tmp.path(), &format!("{path}/.gitignore"), "target/");
    }

    let results = super::run_family(tmp.path());
    assertions::assert_error_summary(
        &results,
        "",
        expected_files.len(),
        expected_files,
        None,
        Some(&["loose files"]),
        None,
        Some(&[".gitignore"]),
    );
}
