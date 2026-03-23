use std::collections::BTreeSet;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

#[test]
fn attacks_impl_level_allows_across_multiple_owned_rust_files() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let backend_rel = "apps/backend/crates/app/commands/src/lib.rs";
    let worker_rel = "apps/worker/crates/app/processor/src/lib.rs";

    let backend_content =
        std::fs::read_to_string(root.join(backend_rel)).expect("read backend source");
    let worker_content =
        std::fs::read_to_string(root.join(worker_rel)).expect("read worker source");

    let backend_line = backend_content.lines().count() + 3;
    let worker_line = worker_content.lines().count() + 3;

    write_file(
        root,
        backend_rel,
        &format!(
            "{backend_content}\n\nstruct ImplAudit;\n#[allow(clippy::too_many_lines)]\nimpl ImplAudit {{\n    fn first(&self) {{}}\n    fn second(&self) {{}}\n    fn third(&self) {{}}\n    fn fourth(&self) {{}}\n}}\n"
        ),
    );
    write_file(
        root,
        worker_rel,
        &format!(
            "{worker_content}\n\nstruct QueueAudit;\n#[allow(clippy::too_many_arguments)]\nimpl QueueAudit {{\n    fn first(&self) {{}}\n    fn second(&self) {{}}\n    fn third(&self) {{}}\n    fn fourth(&self) {{}}\n    fn fifth(&self) {{}}\n}}\n"
        ),
    );

    let results = run_family(root);
    let rs_code_17_results = results
        .iter()
        .filter(|result| result.id == "RS-CODE-17")
        .collect::<Vec<_>>();

    assert_eq!(
        files_for_rule(&results, "RS-CODE-17"),
        BTreeSet::from([backend_rel.to_owned(), worker_rel.to_owned()])
    );
    assert_eq!(rs_code_17_results.len(), 2);
    assert_eq!(
        rs_code_17_results
            .iter()
            .map(|result| (result.file.as_deref(), result.line))
            .collect::<Vec<_>>(),
        vec![
            (Some(backend_rel), Some(backend_line)),
            (Some(worker_rel), Some(worker_line)),
        ]
    );
}
