use std::collections::BTreeSet;

use super::super::copy_fixture;
use super::super::run_family;
use guardrail3_app_rs_family_code_assertions::lint_policy::rs_code_02_unused_crate_dependencies_allow::{
    RuleFinding, Severity, assert_files, assert_findings,
};
use test_support::write_file;

#[test]
fn inventories_unused_crate_dependencies_allow_across_real_owned_files() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let backend_rel = "apps/backend/crates/app/queries/src/lib.rs";
    let worker_rel = "apps/worker/crates/adapters/outbound/sqs/src/lib.rs";
    let test_rel = "apps/backend/tests/unused_deps_inventory_tests.rs";

    let backend_content = test_support::read_file(root, backend_rel);
    let worker_content = test_support::read_file(root, worker_rel);

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
    let relevant_results = results
        .into_iter()
        .filter(|result| {
            matches!(
                result.file(),
                Some(path) if [backend_rel, worker_rel, test_rel].contains(&path)
            )
        })
        .collect::<Vec<_>>();

    assert_files(
        &relevant_results,
        BTreeSet::from([
            backend_rel.to_owned(),
            test_rel.to_owned(),
            worker_rel.to_owned(),
        ]),
    );
    assert_findings(
        &relevant_results,
        &[
            RuleFinding::new(
                Severity::Info,
                "unused_crate_dependencies exemption",
                "unused_crate_dependencies is an approved universal exemption.",
                Some(backend_rel),
                Some(1),
                false,
            ),
            RuleFinding::new(
                Severity::Info,
                "unused_crate_dependencies exemption",
                "unused_crate_dependencies is an approved universal exemption.",
                Some(test_rel),
                Some(1),
                false,
            ),
            RuleFinding::new(
                Severity::Info,
                "unused_crate_dependencies exemption",
                "unused_crate_dependencies is an approved universal exemption.",
                Some(worker_rel),
                Some(1),
                false,
            ),
        ],
    );
}

#[test]
fn only_inventories_crate_level_unused_crate_dependencies_in_mixed_same_file_case() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let rel = "apps/backend/crates/ports/outbound/repo/src/lib.rs";
    let content = test_support::read_file(root, rel);
    let new_content = format!(
        "#![allow(unused_crate_dependencies)]\n{content}\n#[allow(unused_crate_dependencies)]\npub fn item_level_probe() {{}}\nmod outer {{\n    #![allow(unused_crate_dependencies)]\n    pub fn helper() {{}}\n}}\n"
    );

    write_file(root, rel, &new_content);

    let results = run_family(root);
    let inline_line = new_content
        .lines()
        .position(|line| {
            line.contains("#![allow(unused_crate_dependencies)]") && !line.starts_with("#!")
        })
        .map(|index| index + 1)
        .unwrap_or_default();

    assert_files(
        &results,
        BTreeSet::from(["apps/backend/crates/ports/outbound/repo/src/lib.rs".to_owned()]),
    );
    assert_findings(
        &results,
        &[
            RuleFinding::new(
                Severity::Info,
                "unused_crate_dependencies exemption",
                "unused_crate_dependencies is an approved universal exemption.",
                Some("apps/backend/crates/ports/outbound/repo/src/lib.rs"),
                Some(1),
                false,
            ),
            RuleFinding::new(
                Severity::Info,
                "unused_crate_dependencies exemption",
                "unused_crate_dependencies is an approved universal exemption.",
                Some("apps/backend/crates/ports/outbound/repo/src/lib.rs"),
                Some(inline_line),
                false,
            ),
        ],
    );
}

#[test]
fn inventories_each_repeated_crate_level_unused_crate_dependencies_exemption() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let rel = "apps/worker/crates/domain/jobs/src/lib.rs";
    let content = test_support::read_file(root, rel);

    let new_content = format!(
        "#![allow(unused_crate_dependencies)]\n#![allow(unused_crate_dependencies)]\n{content}\n"
    );
    write_file(root, rel, &new_content);

    let results = run_family(root);
    let relevant_results = results
        .into_iter()
        .filter(|result| result.file() == Some(rel))
        .collect::<Vec<_>>();

    assert_findings(
        &relevant_results,
        &[
            RuleFinding::new(
                Severity::Info,
                "unused_crate_dependencies exemption",
                "unused_crate_dependencies is an approved universal exemption.",
                Some(rel),
                Some(1),
                false,
            ),
            RuleFinding::new(
                Severity::Info,
                "unused_crate_dependencies exemption",
                "unused_crate_dependencies is an approved universal exemption.",
                Some(rel),
                Some(2),
                false,
            ),
        ],
    );
}
