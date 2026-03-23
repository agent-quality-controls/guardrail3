use std::collections::BTreeSet;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

#[test]
fn attacks_undocumented_deny_forbid_attrs_across_multiple_owned_files() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let backend_rel = "apps/backend/crates/domain/types/src/lib.rs";
    let worker_rel = "apps/worker/crates/domain/jobs/src/lib.rs";

    let backend_content =
        std::fs::read_to_string(root.join(backend_rel)).expect("read backend source");
    let worker_content =
        std::fs::read_to_string(root.join(worker_rel)).expect("read worker source");

    let backend_line = backend_content.lines().count() + 2;
    let worker_info_line = worker_content.lines().count() + 2;
    let worker_error_line = worker_content.lines().count() + 6;

    write_file(
        root,
        backend_rel,
        &format!("{backend_content}\n#[deny(clippy::panic)]\nfn planner_policy_probe() {{}}\n"),
    );
    write_file(
        root,
        worker_rel,
        &format!(
            "{worker_content}\n#![forbid(unsafe_code)]\n\n#[forbid(clippy::expect_used)]\nfn worker_policy_probe() {{}}\n"
        ),
    );

    let results = run_family(root);
    let rs_code_22_results = results
        .iter()
        .filter(|result| result.id == "RS-CODE-22")
        .collect::<Vec<_>>();

    assert_eq!(
        files_for_rule(&results, "RS-CODE-22"),
        BTreeSet::from([backend_rel.to_owned(), worker_rel.to_owned()])
    );
    assert_eq!(rs_code_22_results.len(), 3);
    assert_eq!(
        rs_code_22_results
            .iter()
            .map(|result| (result.file.as_deref(), result.line, result.inventory))
            .collect::<Vec<_>>(),
        vec![
            (Some(backend_rel), Some(backend_line), false),
            (Some(worker_rel), Some(worker_info_line), true),
            (Some(worker_rel), Some(worker_error_line), false),
        ]
    );
}
