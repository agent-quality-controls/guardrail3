use std::collections::BTreeSet;

use crate::domain::report::Severity;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

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
    let mut rs_code_02_results = results
        .iter()
        .filter(|result| result.id == "RS-CODE-02")
        .map(|result| {
            (
                result.file.clone().expect("file"),
                result.line,
                format!("{:?}", result.severity),
                result.title.clone(),
                result.message.clone(),
            )
        })
        .collect::<Vec<_>>();
    rs_code_02_results.sort();

    assert_eq!(
        files_for_rule(&results, "RS-CODE-02"),
        BTreeSet::from([
            backend_rel.to_owned(),
            worker_rel.to_owned(),
            test_rel.to_owned(),
        ])
    );
    assert_eq!(
        rs_code_02_results,
        vec![
            (
                backend_rel.to_owned(),
                Some(1),
                format!("{:?}", Severity::Info),
                "unused_crate_dependencies exemption".to_owned(),
                "unused_crate_dependencies is an approved universal exemption.".to_owned(),
            ),
            (
                test_rel.to_owned(),
                Some(1),
                format!("{:?}", Severity::Info),
                "unused_crate_dependencies exemption".to_owned(),
                "unused_crate_dependencies is an approved universal exemption.".to_owned(),
            ),
            (
                worker_rel.to_owned(),
                Some(1),
                format!("{:?}", Severity::Info),
                "unused_crate_dependencies exemption".to_owned(),
                "unused_crate_dependencies is an approved universal exemption.".to_owned(),
            ),
        ]
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
    let rs_code_02_results = results
        .iter()
        .filter(|result| result.id == "RS-CODE-02")
        .collect::<Vec<_>>();

    assert_eq!(
        files_for_rule(&results, "RS-CODE-02"),
        BTreeSet::from([rel.to_owned()])
    );
    assert_eq!(rs_code_02_results.len(), 1);
    let result = rs_code_02_results[0];
    assert_eq!(result.file.as_deref(), Some(rel));
    assert_eq!(result.line, Some(1));
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

    let mut rs_code_02_results = run_family(root)
        .into_iter()
        .filter(|result| result.id == "RS-CODE-02")
        .map(|result| (result.file.expect("file"), result.line))
        .collect::<Vec<_>>();
    rs_code_02_results.sort();

    assert_eq!(
        rs_code_02_results,
        vec![(rel.to_owned(), Some(1)), (rel.to_owned(), Some(2))]
    );
}
