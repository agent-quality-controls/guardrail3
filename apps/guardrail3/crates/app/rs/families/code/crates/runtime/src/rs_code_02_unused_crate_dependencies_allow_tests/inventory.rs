use std::collections::BTreeSet;

use guardrail3_domain_report::Severity;

use guardrail3_app_rs_family_code_assertions::rs_code_02_unused_crate_dependencies_allow::{assert_files, assert_findings, assert_value_eq, RuleFinding};
use super::super::run_family;
use super::super::copy_fixture;
use test_support::write_file;

#[test]
fn inventories_unused_crate_dependencies_allow_across_real_owned_files() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let backend_rel = "apps/backend/crates/app/queries/src/lib.rs";
    let worker_rel = "apps/worker/crates/adapters/outbound/sqs/src/lib.rs";
    let test_rel = "apps/backend/tests/unused_deps_inventory_tests.rs";

    let backend_content =
        std::fs::read_to_string(root.join(backend_rel)).expect("read backend file");
    let worker_content = std::fs::read_to_string(root.join(worker_rel)).expect("read worker file");

    write_file(
        root,
        backend_rel,
        &format!("#![allow(unused_crate_dependencies)]\n{backend_content}\n"),
    );
    write_file(
        root,
        worker_rel,
        &format!("#![allow(unused_crate_dependencies)]\n{worker_content}\n"),
    );
    write_file(
        root,
        test_rel,
        "#![allow(unused_crate_dependencies)]\npub fn test_inventory_probe() {}\n",
    );

    let results = run_family(root);

    assert_files(&results, BTreeSet::from([
            backend_rel.to_owned(),
            worker_rel.to_owned(),
            test_rel.to_owned(),
        ]));
    assert_findings(
        &results,
        &[
            RuleFinding {
                severity: Severity::Info,
                title: "unused_crate_dependencies exemption",
                message: "unused_crate_dependencies is an approved universal exemption.",
                file: Some(backend_rel),
                line: Some(1),
                inventory: true,
            },
            RuleFinding {
                severity: Severity::Info,
                title: "unused_crate_dependencies exemption",
                message: "unused_crate_dependencies is an approved universal exemption.",
                file: Some(test_rel),
                line: Some(1),
                inventory: true,
            },
            RuleFinding {
                severity: Severity::Info,
                title: "unused_crate_dependencies exemption",
                message: "unused_crate_dependencies is an approved universal exemption.",
                file: Some(worker_rel),
                line: Some(1),
                inventory: true,
            },
        ],
    );
}

#[test]
fn only_inventories_crate_level_unused_crate_dependencies_in_mixed_same_file_case() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let rel = "apps/backend/crates/ports/outbound/repo/src/lib.rs";
    let content = std::fs::read_to_string(root.join(rel)).expect("read mixed file");

    write_file(
        root,
        rel,
        &format!(
            "#![allow(unused_crate_dependencies)]\n{content}\n#[allow(unused_crate_dependencies)]\npub fn item_level_probe() {{}}\nmod outer {{\n    #![allow(unused_crate_dependencies)]\n    pub fn helper() {{}}\n}}\n"
        ),
    );

    let results = run_family(root);

    assert_files(&results, BTreeSet::from([rel.to_owned()]));
    assert_findings(
        &results,
        &[RuleFinding {
            severity: Severity::Info,
            title: "unused_crate_dependencies exemption",
            message: "unused_crate_dependencies is an approved universal exemption.",
            file: Some(rel),
            line: Some(1),
            inventory: true,
        }],
    );
}

#[test]
fn inventories_each_repeated_crate_level_unused_crate_dependencies_exemption() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let rel = "apps/worker/crates/domain/jobs/src/lib.rs";
    let content = std::fs::read_to_string(root.join(rel)).expect("read repeated file");

    let new_content = format!(
        "#![allow(unused_crate_dependencies)]\n#![allow(unused_crate_dependencies)]\n{content}\n"
    );
    write_file(root, rel, &new_content);

    let results = run_family(root);

    assert_findings(
        &results,
        &[
            RuleFinding {
                severity: Severity::Info,
                title: "unused_crate_dependencies exemption",
                message: "unused_crate_dependencies is an approved universal exemption.",
                file: Some(rel),
                line: Some(1),
                inventory: true,
            },
            RuleFinding {
                severity: Severity::Info,
                title: "unused_crate_dependencies exemption",
                message: "unused_crate_dependencies is an approved universal exemption.",
                file: Some(rel),
                line: Some(2),
                inventory: true,
            },
        ],
    );
}
