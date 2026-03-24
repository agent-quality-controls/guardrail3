use std::collections::BTreeSet;

use crate::domain::report::Severity;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

#[test]
fn attacks_impl_level_allows_across_multiple_owned_rust_files_with_exact_metadata() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let backend_rel = "apps/backend/crates/app/commands/src/lib.rs";
    let worker_rel = "apps/worker/crates/app/processor/src/lib.rs";

    let backend_content =
        std::fs::read_to_string(root.join(backend_rel)).expect("read backend source");
    let worker_content =
        std::fs::read_to_string(root.join(worker_rel)).expect("read worker source");

    let backend_new = format!(
        "{backend_content}\n\nstruct ImplAudit;\n#[allow(clippy::too_many_lines)]\nimpl ImplAudit {{\n    fn first(&self) {{}}\n    fn second(&self) {{}}\n    fn third(&self) {{}}\n    fn fourth(&self) {{}}\n}}\n"
    );
    let worker_new = format!(
        "{worker_content}\n\nstruct QueueAudit;\n#[allow(clippy::too_many_arguments, clippy::too_many_lines)]\nimpl QueueAudit {{\n    fn first(&self) {{}}\n    fn second(&self) {{}}\n    fn third(&self) {{}}\n    fn fourth(&self) {{}}\n    fn fifth(&self) {{}}\n}}\n\nstruct SecondaryAudit;\n#[allow(clippy::type_complexity)]\nimpl SecondaryAudit {{\n    fn first(&self) {{}}\n    fn second(&self) {{}}\n    fn third(&self) {{}}\n    fn fourth(&self) {{}}\n}}\n"
    );

    write_file(root, backend_rel, &backend_new);
    write_file(root, worker_rel, &worker_new);

    let backend_line = backend_new
        .lines()
        .position(|line| line.contains("#[allow(clippy::too_many_lines)]"))
        .expect("backend allow line")
        + 1;
    let worker_grouped_line = worker_new
        .lines()
        .position(|line| {
            line.contains("#[allow(clippy::too_many_arguments, clippy::too_many_lines)]")
        })
        .expect("worker grouped allow line")
        + 1;
    let worker_secondary_line = worker_new
        .lines()
        .position(|line| line.contains("#[allow(clippy::type_complexity)]"))
        .expect("worker secondary allow line")
        + 1;

    let results = run_family(root);
    let mut rs_code_17_results = results
        .iter()
        .filter(|result| result.id == "RS-CODE-17")
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
    rs_code_17_results.sort();

    assert_eq!(
        files_for_rule(&results, "RS-CODE-17"),
        BTreeSet::from([backend_rel.to_owned(), worker_rel.to_owned()])
    );
    assert_eq!(
        rs_code_17_results,
        vec![
            (
                backend_rel.to_owned(),
                Some(backend_line),
                format!("{:?}", Severity::Error),
                "blanket impl-level allow".to_owned(),
                "`#[allow(clippy::too_many_lines)]` covers an impl block with 4 methods. Apply lint suppressions to individual methods instead."
                    .to_owned(),
            ),
            (
                worker_rel.to_owned(),
                Some(worker_grouped_line),
                format!("{:?}", Severity::Error),
                "blanket impl-level allow".to_owned(),
                "`#[allow(clippy::too_many_arguments)]` covers an impl block with 5 methods. Apply lint suppressions to individual methods instead."
                    .to_owned(),
            ),
            (
                worker_rel.to_owned(),
                Some(worker_grouped_line),
                format!("{:?}", Severity::Error),
                "blanket impl-level allow".to_owned(),
                "`#[allow(clippy::too_many_lines)]` covers an impl block with 5 methods. Apply lint suppressions to individual methods instead."
                    .to_owned(),
            ),
            (
                worker_rel.to_owned(),
                Some(worker_secondary_line),
                format!("{:?}", Severity::Error),
                "blanket impl-level allow".to_owned(),
                "`#[allow(clippy::type_complexity)]` covers an impl block with 4 methods. Apply lint suppressions to individual methods instead."
                    .to_owned(),
            ),
        ]
    );
}
