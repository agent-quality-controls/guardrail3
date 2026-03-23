use std::collections::BTreeSet;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

#[test]
fn attacks_always_true_cfg_attr_bypasses_across_multiple_owned_files() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let backend_rel = "apps/backend/crates/app/commands/src/lib.rs";
    let worker_rel = "apps/worker/crates/app/processor/src/lib.rs";

    let backend_content =
        std::fs::read_to_string(root.join(backend_rel)).expect("read backend source");
    let worker_content =
        std::fs::read_to_string(root.join(worker_rel)).expect("read worker source");

    let backend_line = backend_content.lines().count() + 2;
    let worker_line = worker_content.lines().count() + 2;

    write_file(
        root,
        backend_rel,
        &format!(
            "{backend_content}\n#[cfg_attr(any(unix, windows), allow(clippy::unwrap_used))]\nfn cfg_attr_backend_probe() {{}}\n"
        ),
    );
    write_file(
        root,
        worker_rel,
        &format!(
            "{worker_content}\n#[cfg_attr(not(never_target), allow(clippy::expect_used, clippy::panic))]\nfn cfg_attr_worker_probe() {{}}\n"
        ),
    );

    let results = run_family(root);
    let rs_code_18_results = results
        .iter()
        .filter(|result| result.id == "RS-CODE-18")
        .collect::<Vec<_>>();

    assert_eq!(
        files_for_rule(&results, "RS-CODE-18"),
        BTreeSet::from([backend_rel.to_owned(), worker_rel.to_owned()])
    );
    assert_eq!(rs_code_18_results.len(), 3);
    assert_eq!(
        rs_code_18_results
            .iter()
            .map(|result| (result.file.as_deref(), result.line))
            .collect::<Vec<_>>(),
        vec![
            (Some(backend_rel), Some(backend_line)),
            (Some(worker_rel), Some(worker_line)),
            (Some(worker_rel), Some(worker_line)),
        ]
    );
}
